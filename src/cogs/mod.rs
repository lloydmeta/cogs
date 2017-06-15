//! This module holds logic and data structures related to cognition services,
//! called Cogs.

pub mod translation;

use hyper::client::{Request, Response};
use futures::Future;
use super::engine;

/// Trait representing something that can be turned into a Cognitive Service endpoint.
///
/// In essence, it is capable of of
///
///   1. Transforming from your data structure to a hyper::Request
///   2. Transforming from a Result<hyper::Response, Error> -> Future<Item, Error>
///
/// # Examples
///
/// ```
/// # extern crate hyper;
/// # extern crate hyper_tls;
/// # extern crate tokio_core;
/// # extern crate cogs;
/// # fn main() {
/// # use cogs::engine::*;
/// # use hyper_tls;
/// # use std::env;
/// # use cogs::cogs::translation::TranslateRequest;
/// let mut core = tokio_core::reactor::Core::new().unwrap();
/// let handle = core.handle();
/// let client = hyper::Client::configure()
///     .connector(hyper_tls::HttpsConnector::new(4, &handle).unwrap())
///     .keep_alive(true)
///     .build(&handle);
/// # let sub_key = SubscriptionKey::new(env::var("AZURE_SUBSCRIPTION_KEY").unwrap().as_str());
/// let credentials = Credentials::new(sub_key);
/// let engine = Engine::new(credentials, client);
/// let translate_req = TranslateRequest {
///     text: "Hello",
///     from: Some("en"),
///     to: "de",
///     content_type: None,
///     category: None,
/// };
/// let work = engine.run(translate_req);
/// // TODO: get a sandbox key so this actually works as expected, returning "Hallo"
/// assert_eq!(core.run(work).unwrap(), "")
/// # }
/// ```
pub trait Cog: Into<Request> {
    /// What this Cog returns using input from the Engine
    ///
    /// In short it needs be able to convert input from the Engine of
    /// the form Result<Response, engine::Error>> into something that
    /// implements Future<Self::Item, Self::Error>.
    ///
    /// It must also be "static", meaning that it doesn't refer to data
    /// that lives on the stack
    type Output: Future<Item = Self::Item, Error = Self::Error> + From<Result<Response, engine::Error>> + 'static;

    /// Item type
    type Item: 'static;

    /// Error type
    type Error: 'static;
}
