//! Holds Engine related logic
use hyper;
use hyper::client::*;
use hyper::Uri;
use hyper::Body;
use hyper::Method;
use hyper::header::{Authorization, Bearer, ContentLength};
use std::str::FromStr;
use futures::future::Future;
use futures::{future, Stream};
use time::*;
use std::sync::{RwLock, Arc};
use cogs::*;
use std::convert::From;

const TOKEN_EXPIRES_IN_MINS: i64 = 9;
lazy_static! {
    static ref ISSUE_TOKEN_URI: Uri = {
        Uri::from_str("https://api.cognitive.microsoft.com/sts/v1.0/issueToken").unwrap()
    };
}

header! {
    (SubscriptionKeyHeader, "Ocp-Apim-Subscription-Key") => [String]
}

/// Struct for holding Engine data
///
/// Instantiate one using Engine::new
pub struct Engine<Connector>
    where Connector: Connect + Service
{
    credentials: Arc<RwLock<Credentials>>,
    client: Arc<Client<Connector>>,
}

impl<Connector> Engine<Connector>
    where Connector: Connect,
          Connector: Service
{
    /// Returns a new Engine
    ///
    /// ```
    /// # extern crate hyper;
    /// # extern crate hyper_tls;
    /// # extern crate tokio_core;
    /// # extern crate cogs;
    /// # fn main() {
    /// # let core = tokio_core::reactor::Core::new().unwrap();
    /// # let handle = core.handle();
    /// # use cogs::engine::*;
    /// # use hyper_tls;
    /// let client = hyper::Client::configure()
    ///     .connector(hyper_tls::HttpsConnector::new(4, &handle))
    ///     .keep_alive(true)
    ///     .build(&handle);
    /// let credentials = Credentials::new(SubscriptionKey::new("abc123"));
    /// Engine::new(credentials, client);
    /// # }
    /// ```
    pub fn new(credentials: Credentials, client: Client<Connector>) -> Self {
        Engine {
            credentials: Arc::new(RwLock::new(credentials)),
            client: Arc::new(client),
        }
    }

    /// Runs a Cog and returns its Output
    pub fn run<A>(&self, cog: A) -> Box<Future<Item = <A as Cog>::Item, Error = <A as Cog>::Error>>
        where A: Cog
    {
        let mut req: Request = cog.into();
        let client_ref = self.client.clone();
        // TODO see if we can get rid of boxing with RFC 1522 is finished
        let f = self.renew_token().then(move |engine_result| {
            // TODO see if we can simplify the awkward match dance here for the sake of error types
            let inner_f: Box<Future<Item = <A as Cog>::Item, Error = <A as Cog>::Error>> =
                match engine_result {
                    Ok(AccessToken { token, .. }) => {
                        {
                            let mut headers = req.headers_mut();
                            headers.set(Authorization(Bearer { token: token.clone() }));
                        }
                        Box::new(client_ref.request(req)
                                     .map_err(|e| Error::HyperError(e))
                                     .then(|r| <A as Cog>::Output::from(r)))
                    }
                    Err(e) => Box::new(<A as Cog>::Output::from(Err(e))),
                };
            inner_f
        });
        Box::new(f)
    }

    /// Conditionally renews the token and returns a valid value in a Future.
    ///
    /// Note, using Box<Future<_,_>> because heck, even Hyper does this (See FutureResponse).
    /// Optimise later.
    fn renew_token(&self) -> Box<Future<Item = AccessToken, Error = Error>> {
        // First, try to retrieve the token using just a read lock.
        let retrieve_token_sync: Option<AccessToken> = {
            let read_lock = self.credentials.read();
            match read_lock {
                Ok(ref creds) => {
                    if creds.should_renew_token() {
                        None
                    } else {
                        creds.access_token.clone()
                    }
                }
                _ => None, // ignore the error for now. Maybe we can work with it.
            }
        };
        match retrieve_token_sync {
            // Success. Just wrap it in a future
            Some(t) => future::ok(t).boxed(),
            // Nope
            None => {
                // Grab a write lock.
                let write_lock_sync = self.credentials.write();
                match write_lock_sync {
                    Ok(mut creds) => {
                        /*  Check again now that we're inside a write lock
                         *  It's possible that the token was renewed already by another thread
                         *  by the time we get here
                         */
                        if creds.should_renew_token() {
                            if creds.renewing_token {
                                /* Someone is already working on this (likely in another thread),
                                 * so just loop.
                                 */
                                self.renew_token()
                            } else {
                                let mut req: Request<Body> = Request::new(Method::Post,
                                                                          ISSUE_TOKEN_URI.clone());
                                {
                                    let headers = req.headers_mut();
                                    headers.set(SubscriptionKeyHeader(creds.subscription_key
                                                                          .0
                                                                          .clone()));
                                    headers.set(ContentLength(0));
                                }
                                // Set the renewing token flag before hitting any async barriers
                                creds.renewing_token = true;
                                let req_f =
                                    self.client.request(req).map_err(|e| Error::HyperError(e));
                                let creds_ref_err = self.credentials.clone();
                                let creds_ref = self.credentials.clone();
                                let f = req_f.and_then(|r| {
                                    read_to_string(r).and_then(move |t| {
                                        // These weird brackets help the compiler understand scope
                                        // of the write lock.
                                        let r = {
                                            let creds_write_lock = creds_ref.write();
                                            creds_write_lock
                                                .map_err(|_| Error::LockPoisonedError)
                                                .map(|mut creds| {
                                                    let access = AccessToken {
                                                        token: t,
                                                        expires_at: now() + Duration::minutes(TOKEN_EXPIRES_IN_MINS),
                                                    };
                                                    creds.renewing_token = false;
                                                    creds.update_token(access.clone());
                                                    access
                                                })
                                        };
                                        r
                                    })
                                }).map_err(move |e| {
                                    // We failed somewhere so set the renewing token flag to false
                                    let creds_err_lock = creds_ref_err.write();
                                    match creds_err_lock {
                                        Ok(mut creds) => {
                                            creds.renewing_token = false;
                                            e
                                        }
                                        _ => Error::LockPoisonedError // this is more important at this point
                                    }
                                });
                                Box::new(f)
                            }
                        } else {
                            // Another thread did the work for us. Just clone and return.
                            let t = creds.access_token.clone().unwrap();
                            future::ok(t).boxed()
                        }
                    }
                    _ => future::err(Error::LockPoisonedError).boxed(),
                }
            }
        }
    }
}

/// Consumes the body and reads it into a String.
pub fn read_to_string(resp: Response) -> Box<Future<Item = String, Error = Error>> {
    Box::new(read_to_bytes(resp).and_then(|b| {
                                              String::from_utf8(b).map_err(|_| Error::FromUtf8Error)
                                          }))
}

/// Consumes a response, returning the body as a vector of bytes
pub fn read_to_bytes(resp: Response) -> Box<Future<Item = Vec<u8>, Error = Error>> {
    let vec = if let Some(len) = resp.headers().get::<ContentLength>() {
        Vec::with_capacity(**len as usize)
    } else {
        vec![]
    };
    Box::new(resp.body()
            // Body is a Stream (-.- " )which effectively uses Hyper::Error as well
            .map_err(|e| Error::HyperError(e))
            .fold(vec, |mut acc, chunk| {
                acc.extend_from_slice(chunk.as_ref());
                Ok::<_, Error>(acc)
            }))
}

#[derive(Debug)]
pub enum Error {
    CouldNotRetrieveToken,
    FromUtf8Error,
    LockPoisonedError,
    HyperError(hyper::Error),
}

/// Holds credential information for accessing Cognitive services
pub struct Credentials {
    pub subscription_key: SubscriptionKey,
    #[doc(hidden)]
    access_token: Option<AccessToken>,
    #[doc(hidden)]
    renewing_token: bool,
}

impl Credentials {
    /// Returns a new, uninitiated set of credentials
    pub fn new(key: SubscriptionKey) -> Credentials {
        Credentials {
            subscription_key: key,
            access_token: None,
            renewing_token: false,
        }
    }

    /// Convenience method for determining whether we should renew the token
    pub fn should_renew_token(&self) -> bool {
        let now = now();
        match self.token_expires_at() {
            Some(expires) => now >= *expires,
            _ => true,
        }
    }

    /// Updates the access token when needed
    fn update_token(&mut self, token: AccessToken) {
        self.access_token = Some(token);
    }

    /// Exposes the token expires at.
    fn token_expires_at(&self) -> Option<&Tm> {
        match self.access_token {
            Some(ref tm) => Some(&tm.expires_at),
            _ => None,
        }
    }
}

/// Wraps a subscription key
pub struct SubscriptionKey(String);

impl SubscriptionKey {
    /// Returns a new Subscription key
    pub fn new(key: &str) -> SubscriptionKey {
        SubscriptionKey(key.to_string())
    }

    /// Returns the string value of a SubscriptionKey
    pub fn value(&self) -> &str {
        &self.0[..]
    }
}

/// Holds an Access Token
#[derive(Clone)]
struct AccessToken {
    token: String,
    expires_at: Tm,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_core;
    use std::env;
    use hyper_tls;

    fn subscription_key() -> String {
        env::var("AZURE_SUBSCRIPTION_KEY").unwrap()
    }

    #[test]
    fn renew_token_test() {
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(hyper_tls::HttpsConnector::new(4, &handle))
            .build(&handle);
        let sub_key = SubscriptionKey::new(subscription_key().as_str());
        let credentials = Credentials::new(sub_key);
        let engine = Engine::new(credentials, client);
        let work = engine.renew_token();
        assert!(core.run(work).is_ok())
    }
}
