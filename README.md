# Cogs [![Build Status](https://travis-ci.org/lloydmeta/cogs.svg?branch=master)](https://travis-ci.org/lloydmeta/cogs)

Non-blocking Microsoft Cognition APIs client for Rust.

WIP

_Note_: Since Tokio-based Hyper is not yet published to Crates.io, this lib hasn't been published yet either.

# Usage

## Library

You'll need to clone this repo and specify a path.

```toml
cogs = { path = "$cloned_path" }
```

## Command line

Clone this repo, cd into it, then run

```
cargo install
```

Then either set an `AZURE_SUBSCRIPTION_KEY` in your environment, or pass it in via command line.

```shell
cogs --from=en --to=de --repl


**************************** Cogs ****************************

Enter text and get back a translation. Ctrl+C to exit.


Hi, my name is Lloyd.
Hallo, ist mein Name Lloyd.
```


## Links

1. Translation API docs [here](http://docs.microsofttranslator.com/text-translate.html)
