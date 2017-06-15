extern crate cogs;
extern crate clap;
extern crate tokio_core;
extern crate hyper;
extern crate hyper_tls;
extern crate native_tls;

use std::io;
use clap::{Arg, App};
use cogs::engine::*;
use cogs::translation::*;
use hyper::client::*;

const FROM_KEY: &'static str = "from";
const TO_KEY: &'static str = "to";
const SUBSCRIPTION_KEY: &'static str = "subscription-key";
const REPL_MODE_KEY: &'static str = "repl";
const AZURE_SUBSCRIPTION_KEY: &'static str = "AZURE_SUBSCRIPTION_KEY";

const GREET: &'static str = r#"

**************************** Cogs ****************************

Enter text and get back a translation. Ctrl+C to exit.

"#;

fn version() -> String {
    let (maj, min, pat) = (option_env!("CARGO_PKG_VERSION_MAJOR"),
                           option_env!("CARGO_PKG_VERSION_MINOR"),
                           option_env!("CARGO_PKG_VERSION_PATCH"));
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => format!("{}.{}.{}", maj, min, pat),
        _ => "".to_owned(),
    }
}

fn main() {

    let sub_key_help = format!("Azure subscription key. Defaults to environment variable {} if set.",
                               AZURE_SUBSCRIPTION_KEY);
    let v = version();
    let app = App::new("Cogs")
        .version(v.as_str())
        .author("Lloyd (github.com/lloydmeta)")
        .about("Translate things with MS Cognition services")
        .arg(Arg::with_name(FROM_KEY)
                 .short("f")
                 .long(FROM_KEY)
                 .number_of_values(1)
                 .takes_value(true)
                 .help("Language to translate from")
                 .required(true))
        .arg(Arg::with_name(TO_KEY)
                 .short("t")
                 .long(TO_KEY)
                 .takes_value(true)
                 .number_of_values(1)
                 .help("Language to translate to")
                 .required(true))
        .arg(Arg::with_name(SUBSCRIPTION_KEY)
                 .short("s")
                 .long(SUBSCRIPTION_KEY)
                 .takes_value(true)
                 .number_of_values(1)
                 .help(sub_key_help.as_ref())
                 .required(false))
        .arg(Arg::with_name(REPL_MODE_KEY)
                 .long(REPL_MODE_KEY)
                 .short("r")
                 .takes_value(false)
                 .help("Set this flag to start a REPL session.")
                 .required(false));
    let mut app_clone = app.clone();

    let matches = app.get_matches();

    let sub_from_env = std::env::var(AZURE_SUBSCRIPTION_KEY).ok();
    let sub_from_str = match sub_from_env {
        Some(ref s) => Some(s.as_ref()),
        _ => None,
    };
    let sub_str = matches
        .value_of(SUBSCRIPTION_KEY)
        .map(|s| s)
        .or(sub_from_str);

    match (matches.value_of(FROM_KEY), matches.value_of(TO_KEY), sub_str) {
        (Some(from), Some(to), Some(sub)) => {
            let (mut core, engine) = build_engine(SubscriptionKey::new(sub)).unwrap();
            if matches.is_present(REPL_MODE_KEY) {
                println!("{}", GREET);
                loop {
                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer).unwrap();
                    let translate_req = TranslateRequest {
                        text: buffer.as_str(),
                        from: Some(from),
                        to: to,
                        content_type: None,
                        category: None,
                    };
                    let work = engine.run(translate_req);
                    let result = core.run(work).unwrap();
                    println!("{}\n", result)
                }
            } else {
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
                let translate_req = TranslateRequest {
                    text: buffer.as_str(),
                    from: Some(from),
                    to: to,
                    content_type: None,
                    category: None,
                };
                let work = engine.run(translate_req);
                let result = core.run(work).unwrap();
                println!("{}", result)
            }
        }
        _ => {
            let _ = app_clone.print_help();
        }
    }

}

fn build_engine(sub_key: SubscriptionKey)
                -> Result<(tokio_core::reactor::Core,
                           Engine<hyper_tls::HttpsConnector<HttpConnector>>),
                          native_tls::Error> {
    let core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle)?)
        .build(&handle);
    let credentials = Credentials::new(sub_key);
    let engine = Engine::new(credentials, client);
    Ok((core, engine))
}
