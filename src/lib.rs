//! Cogs is a non-blocking client for Microsoft cognition services
//!
//! Although it is a WIP and not all the APIs are implemented, it
//! contains enough infrastructure for you to being using it. All you need
//! to do to add support for a new endpoint is to implement Cog (see cogs module)
//! for your endpoint. To see an example of this, check the translations module.
//!
//! Example usage
//!
//! ```
//! # extern crate hyper;
//! # extern crate hyper_tls;
//! # extern crate tokio_core;
//! # extern crate cogs;
//! # fn main() {
//! # use cogs::engine::*;
//! # use hyper_tls;
//! # use std::env;
//! # use cogs::cogs::translation::TranslateRequest;
//! let mut core = tokio_core::reactor::Core::new().unwrap();
//! let handle = core.handle();
//! let client = hyper::Client::configure()
//!     .connector(hyper_tls::HttpsConnector::new(4, &handle))
//!     .keep_alive(true)
//!     .build(&handle);
//! # let sub_key = SubscriptionKey::new(env::var("AZURE_SUBSCRIPTION_KEY").unwrap().as_str());
//! let credentials = Credentials::new(sub_key);
//! let engine = Engine::new(credentials, client);
//! let translate_req = TranslateRequest {
//!     text: "Hello",
//!     from: Some("en"),
//!     to: "de",
//!     content_type: None,
//!     category: None,
//! };
//! let work = engine.run(translate_req);
//! assert_eq!(core.run(work).unwrap(), "Hallo")
//! # }
//! ```
extern crate time;
#[macro_use]
extern crate hyper;
extern crate futures;
extern crate core;
#[macro_use]
extern crate lazy_static;
extern crate elementtree;
extern crate url;
pub mod engine;
pub mod cogs;

pub use cogs::*;

#[cfg(test)]
extern crate tokio_core;
#[cfg(test)]
extern crate hyper_tls;
