//! This module holds Cogs related to translation
use hyper::client::Request;
use hyper::client::Response;
use engine;
use futures::Future;
use url::Url;
use hyper::Uri;
use hyper::Method;
use core::str::FromStr;
use futures::Poll;
use std::fmt;
use elementtree::*;
use futures::future;
use super::*;

/// A Translation request.
pub struct TranslateRequest<'a> {
    pub text: &'a str,
    pub from: Option<&'a str>,
    pub to: &'a str,
    pub content_type: Option<TranslateContentType>,
    pub category: Option<&'a str>,
}

/// Possible Content types for translation
pub enum TranslateContentType {
    Plain,
    Html,
}

/// Wrapper type for our boxed future
pub struct FutureTranslateResponse(Box<Future<Item = String, Error = Error>>);

impl fmt::Debug for FutureTranslateResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("FutureTranslateResponse")
    }
}

impl Future for FutureTranslateResponse {
    type Item = String;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

const TRANSLATE_BASE_URI: &'static str = "https://api.microsofttranslator.com/v2/http.svc/Translate";

impl<'a> From<TranslateRequest<'a>> for Request {
    fn from(t: TranslateRequest<'a>) -> Self {
        let mut url = Url::parse_with_params(TRANSLATE_BASE_URI, &[("to", t.to), ("text", t.text)])
            .unwrap();
        {
            let mut mut_pairs = url.query_pairs_mut();
            match t.from {
                Some(from) => {
                    mut_pairs.append_pair("from", from);
                }
                _ => (),
            }
            match t.content_type {
                Some(TranslateContentType::Html) => {
                    mut_pairs.append_pair("contentType", "text/html");
                }
                Some(TranslateContentType::Plain) => {
                    mut_pairs.append_pair("contentType", "text/plain");
                }
                _ => (),
            }
            match t.category {
                Some(cat) => {
                    mut_pairs.append_pair("category", cat);
                }
                _ => (),
            }
            mut_pairs.finish();
        }
        let as_uri = Uri::from_str(url.as_str()).unwrap();
        Request::new(Method::Get, as_uri)
    }
}

impl<'a> Cog for TranslateRequest<'a> {
    type Output = FutureTranslateResponse;
    type Item = String;
    type Error = Error;
}

impl From<Result<Response, engine::Error>> for FutureTranslateResponse {
    fn from(result: Result<Response, engine::Error>) -> Self {
        match result {
            Ok(resp) => {
                let body_bytes = engine::read_to_bytes(resp);
                let f = body_bytes
                    .map_err(|e| Error::EngineError(e))
                    .and_then(|b| match Element::from_reader(b.as_slice()) {
                                  Ok(root) => Ok(root.text().to_string()),
                                  _ => Err(Error::XMLParsingError),
                              });

                FutureTranslateResponse(Box::new(f))
            }
            Err(e) => FutureTranslateResponse(future::err(Error::EngineError(e)).boxed()),
        }
    }
}

/// Translation error mapping
#[derive(Debug)]
pub enum Error {
    XMLParsingError,
    EngineError(engine::Error),
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::engine::*;
    use hyper::client::*;
    use tokio_core;
    use std::env;
    use hyper_tls;

    fn subscription_key() -> String {
        env::var("AZURE_SUBSCRIPTION_KEY").unwrap()
    }

    #[test]
    fn translation_test() {
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(hyper_tls::HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);
        let sub_key = SubscriptionKey::new(subscription_key().as_str());
        let credentials = Credentials::new(sub_key);
        let engine = Engine::new(credentials, client);
        let translate_req = TranslateRequest {
            text: "Hello",
            from: Some("en"),
            to: "de",
            content_type: None,
            category: None,
        };
        let work = engine.run(translate_req);
        assert_eq!(core.run(work).unwrap(), "") // TODO: get a sandbox key so this starts working again.
    }
}
