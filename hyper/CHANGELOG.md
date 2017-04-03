## v0.10.0 (2017-01-10)

#### Features

* **client:**
  * change ProxyConfig to allow HTTPS proxies ([14a4f1c2](https://github.com/hyperium/hyper/commit/14a4f1c2f735efe7b638e9078710ca32dc1e360a))
  * remove experimental HTTP2 support ([d301c6a1](https://github.com/hyperium/hyper/commit/d301c6a1708c7d408b7f03ac46674a5f0edd3253))
* **header:** remove `cookie` dependency ([f22701f7](https://github.com/hyperium/hyper/commit/f22701f7e7258ad4a26645eba47a3d374e452e86))
* **lib:**
  * remove SSL dependencies ([2f48612c](https://github.com/hyperium/hyper/commit/2f48612c7e141a9d612d7cb9d524b2f460561f56))
  * remove `serde-serialization` feature ([7b9817ed](https://github.com/hyperium/hyper/commit/7b9817edcf4451bd033e55467c75577031bfe740))


#### Breaking Changes

* There is no more `hyper::http::h2`.

  ([d301c6a1](https://github.com/hyperium/hyper/commit/d301c6a1708c7d408b7f03ac46674a5f0edd3253))
* The `Cookie` and `SetCookie` headers no longer use the
  cookie crate. New headers can be written for any header, or the ones
  provided in hyper can be accessed as strings.

  ([f22701f7](https://github.com/hyperium/hyper/commit/f22701f7e7258ad4a26645eba47a3d374e452e86))
* There is no longer a `serde-serialization` feature.
  Look at external crates, like `hyper-serde`, to fulfill this feature.

  ([7b9817ed](https://github.com/hyperium/hyper/commit/7b9817edcf4451bd033e55467c75577031bfe740))
* hyper will no longer provide OpenSSL support out of the
  box. The `hyper::net::Openssl` and related types are gone. The `Client`
  now uses an `HttpConnector` by default, which will error trying to
  access HTTPS URLs.

  TLS support should be added in from other crates, such as
  hyper-openssl, or similar using different TLS implementations.

  ([2f48612c](https://github.com/hyperium/hyper/commit/2f48612c7e141a9d612d7cb9d524b2f460561f56))
* Usage of `with_proxy_config` will need to change to
  provide a network connector. For the same functionality, a
  `hyper::net::HttpConnector` can be easily created and passed.

  ([14a4f1c2](https://github.com/hyperium/hyper/commit/14a4f1c2f735efe7b638e9078710ca32dc1e360a))


### v0.9.14 (2016-12-12)


#### Features

* **headers:** add star, json, text, image constructors to Accept ([a9fbbd7f](https://github.com/hyperium/hyper/commit/a9fbbd7fdbcbec51ef560e9882a8fefa64a93b54))
* **server:** add 'take_buf' method to BufReader ([bbbce5fc](https://github.com/hyperium/hyper/commit/bbbce5fc8bca0bcc34df4a4a9223432085fba2ff))


### v0.9.13 (2016-11-27)


#### Bug Fixes

* **client:** close Pooled streams on sockopt error ([d5ffee2e](https://github.com/hyperium/hyper/commit/d5ffee2ec801274ac271273289084b7251b4ce89))


### v0.9.12 (2016-11-09)


#### Features

* **error:** re-export url::ParseError ([30e78ac2](https://github.com/hyperium/hyper/commit/30e78ac212ed3085a5217e8d7f641c2f161ddc87))


### v0.9.11 (2016-10-31)


#### Bug Fixes

* **headers:** Allow IPv6 Addresses in Host header ([20f177ab](https://github.com/hyperium/hyper/commit/20f177abec12397f23adf43f6b726daee1a731cf))


#### Features

* **headers:**
  * Add strict-origin and strict-origin-when-cross-origin referer policy ([1be4e769](https://github.com/hyperium/hyper/commit/1be4e7693f7d27c049f35fefb9fffead2581b1f4))
  * support multiple values for Referrer-Policy header ([dc476657](https://github.com/hyperium/hyper/commit/dc4766573af9bd31d57fede5b9ef0ffa56fe44ab), closes [#882](https://github.com/hyperium/hyper/issues/882))
  * add last-event-id header ([2277987f](https://github.com/hyperium/hyper/commit/2277987f3c25380353db606ca7baaf0c854095cd))
* **server:** accept combined certificate files ([eeb1f48e](https://github.com/hyperium/hyper/commit/eeb1f48e17f4c71162ce90f88bda3dc37b489cc7))


### v0.9.10 (2016-07-11)


#### Features

* **headers:**
  * add origin header ([64881ae0](https://github.com/hyperium/hyper/commit/64881ae05458f06261b2e7d0f790184678cc42b9))
  * Add Referrer-Policy header ([b76a02cc](https://github.com/hyperium/hyper/commit/b76a02cc446f2a3935006035fd73f5f7a47ec428))


### v0.9.9 (2016-06-21)


#### Bug Fixes

* **headers:** Remove raw part when getting mutable reference to typed header ([63b61524](https://github.com/hyperium/hyper/commit/63b615249443b8f897018f21473c2f1f8e43663c), closes [#821](https://github.com/hyperium/hyper/issues/821))


#### Features

* **error:** Display for Error shows better info ([5620fbf9](https://github.com/hyperium/hyper/commit/5620fbf98f1fd43482a9ffa3c98f2f38b42fd4b0), closes [#694](https://github.com/hyperium/hyper/issues/694))


### v0.9.8 (2016-06-14)


#### Features

* **client:** enable use of custom TLS wrapper for proxied connections ([0476196c](https://github.com/hyperium/hyper/commit/0476196c320765a66f730c56048998980b173caf), closes [#824](https://github.com/hyperium/hyper/issues/824))


### v0.9.7 (2016-06-09)


#### Bug Fixes

* **proxy:** fix the 0.9.x build with `--no-default-features --features=security-framework` ([6caffe9f](https://github.com/hyperium/hyper/commit/6caffe9fb302da99ce8cf0c8027c06b8c6de782d), closes [#819](https://github.com/hyperium/hyper/issues/819))
* **server:** Request.ssl() works ([ce0b62ea](https://github.com/hyperium/hyper/commit/ce0b62eae7688987b722599be8e8b2ff6764b224))


### v0.9.6 (2016-05-23)


#### Bug Fixes

* **client:** Manually impl Debug for PooledStream ([aa692236](https://github.com/hyperium/hyper/commit/aa692236a851d29abec63b6a0d61d957cea5fd26))
* **server:** Switch Ssl to SslServer in bounds ([470bc8ec](https://github.com/hyperium/hyper/commit/470bc8ec396bfc9ead6782f72e6de58268767a5a))


### v0.9.5 (2016-05-18)


#### Bug Fixes

* **windows:** disable openssl cert validation for Windows ([c89aca81](https://github.com/hyperium/hyper/commit/c89aca812bf863aadb52326f534a65c1d3cf31d6), closes [#794](https://github.com/hyperium/hyper/issues/794))


#### Features

* **net:** Add OpensslClient constructor ([3c0e1050](https://github.com/hyperium/hyper/commit/3c0e105011fc8a4fc639370836aa6a2e576b6f0e))


### v0.9.4 (2016-05-09)


#### Bug Fixes

* **warnings:** remove unused_code warnings from newest nightlies ([e7229480](https://github.com/hyperium/hyper/commit/e7229480ea669bbe62fe644e312ba06cdca45b1c))


#### Features

* **ssl:**
  * enable hostname verification by default for OpenSSL ([01160abd](https://github.com/hyperium/hyper/commit/01160abd92956e5f995cc45790df7a2b86c8989f), closes [#472](https://github.com/hyperium/hyper/issues/472))
  * use secure ciphers by default in openssl ([54bf6ade](https://github.com/hyperium/hyper/commit/54bf6adeee1c3a231925f3efa7e38f875bc2d4d5))

### v0.9.3 (2016-05-09)


#### Bug Fixes

* **client:** fix panic in Pool::connect ([e51bafe2](https://github.com/hyperium/hyper/commit/e51bafe2e4f2a1efc36790232bef488c91131d0b), closes [#780](https://github.com/hyperium/hyper/issues/780))


### v0.9.2 (2016-05-04)


#### Features

* **client:**
  *  proper proxy and tunneling in Client ([f36c6b25](https://github.com/hyperium/hyper/commit/f36c6b25), closes [#774](https://github.com/hyperium/hyper/issues/774))
  *  add Proxy support ([25010fc1](https://github.com/hyperium/hyper/commit/25010fc1), closes [#531](https://github.com/hyperium/hyper/issues/531))

#### Performance

* **client:**  don't keep Pool mutex locked during getaddrinfo ([5fcc04a6](https://github.com/hyperium/hyper/commit/5fcc04a6))


### v0.9.1 (2016-04-21)


#### Bug Fixes

* **Cargo.toml:** update documentation link ([b783ddf4](https://github.com/hyperium/hyper/commit/b783ddf455ee93cc38510f3179ffe18733c797c1))


## v0.9.0 (2016-04-21)


#### Features

* **net:** Add Ssl impls for security-framework ([f37315b2](https://github.com/hyperium/hyper/commit/f37315b2708e092eaf5177a6960df9f7bf11eb5c))


#### Breaking Changes

* The re-exported Url type has breaking changes.
 ([8fa7a989](https://github.com/hyperium/hyper/commit/8fa7a9896809ef2a24994993b91981105a520f26))


### v0.8.1 (2016-04-13)


#### Bug Fixes

* **headers:** correctly handle repeated headers ([70c69142](https://github.com/hyperium/hyper/commit/70c6914217a9b48880e61b7fb59acd15c6e1421e), closes [#683](https://github.com/hyperium/hyper/issues/683))


#### Features

* **header:** add prefer and preference applied headers ([6f649301](https://github.com/hyperium/hyper/commit/6f6493010a9c190b29aceb3c10c65785923a85f5), closes [#747](https://github.com/hyperium/hyper/issues/747))
* **net:** Split Ssl into SslClient and SslServer ([2c86e807](https://github.com/hyperium/hyper/commit/2c86e8078ec01db2283e1fee1461db4c7bf76d3f), closes [#756](https://github.com/hyperium/hyper/issues/756))


## v0.8.0 (2016-03-14)


#### Bug Fixes

* **headers:** remove charset from `ContentType::json()` convenience method ([ec568e9a](https://github.com/hyperium/hyper/commit/ec568e9a551018b3353b6754eb2fcd729c7ea3c6))
* **net:** fix the typo in `set_write_timeout` ([7c76fff3](https://github.com/hyperium/hyper/commit/7c76fff3aaf0f0a300e76622acb56eaf1e2cb474))


#### Features

* **client:** Implement Debug for Client ([8c7ef7fd](https://github.com/hyperium/hyper/commit/8c7ef7fd937616798780d43f80a6b46507bc3433))
* **status:** add HTTP statuses 421 and 451 ([93fd5a87](https://github.com/hyperium/hyper/commit/93fd5a87bddc5bfe29f35f86d44d3f46c81ff5fa))


#### Breaking Changes

* mime 0.2 depends on serde 0.7, so any instances of
  using older versions of serde will need to upgrade.

 ([146df53c](https://github.com/hyperium/hyper/commit/146df53caf2a70cd15f97710738ba8d350040c12))


### v0.7.2 (2016-01-04)


#### Bug Fixes

* **buffer:** fix incorrect resizing of BufReader ([3a18e72b](https://github.com/hyperium/hyper/commit/3a18e72be67152834f6967c6d208f214288178ee), closes [#715](https://github.com/hyperium/hyper/issues/715))


#### Features

* **headers:** allow ExtendedValue structs to be formatted and used as struct members ([da0abe89](https://github.com/hyperium/hyper/commit/da0abe8988a61281b447a554b65ea8fd5d54f81b))


### v0.7.1 (2015-12-19)


#### Bug Fixes

* **cargo:** remove * dependendies for serde and env_logger ([4a05bee9](https://github.com/hyperium/hyper/commit/4a05bee9abdc426bbd904fe356b771e492dc8f43))
* **server:**
  * Flush 100-continue messages ([92ff50f2](https://github.com/hyperium/hyper/commit/92ff50f2e57fa2cb8a55b3d6d9fa43ef9a1b5526), closes [#704](https://github.com/hyperium/hyper/issues/704))
  * Removed check for GET/HEAD request when parsing body ([0b05c590](https://github.com/hyperium/hyper/commit/0b05c5903e86327cc9cb4cac39217e496851fce3), closes [#698](https://github.com/hyperium/hyper/issues/698))


#### Features

* **headers:** add extended parameter parser to the public API ([402fb76b](https://github.com/hyperium/hyper/commit/402fb76bb2f3dab101509e4703743ab075ae41be))


## v0.7.0 (2015-11-24)


#### Features

* **all:** add socket timeouts ([fec6e3e8](https://github.com/hyperium/hyper/commit/fec6e3e873eb79bd17d1c072d2ca3c7b91624f9c))
* **headers:**
  * Add Content-Disposition header ([7623ecc2](https://github.com/hyperium/hyper/commit/7623ecc26466e2e072eb2b03afc5e6c16d8e9bc9), closes [#561](https://github.com/hyperium/hyper/issues/561))
  * Add Access-Control-Allow-Credentials header ([19348b89](https://github.com/hyperium/hyper/commit/19348b892be4687e2c0e48b3d01562562340aa1f), closes [#655](https://github.com/hyperium/hyper/issues/655))
  * re-export CookiePair and CookieJar ([799698ca](https://github.com/hyperium/hyper/commit/799698ca87bc8f2f5446e9cb1301e5976657db6b))


#### Breaking Changes

* This adds 2 required methods to the `NetworkStream`
  trait, `set_read_timeout` and `set_write_timeout`. Any local
  implementations will need to add them.

 ([fec6e3e8](https://github.com/hyperium/hyper/commit/fec6e3e873eb79bd17d1c072d2ca3c7b91624f9c))
* LanguageTags api is changed.

 ([c747f99d](https://github.com/hyperium/hyper/commit/c747f99d2137e03b5f4393ee3731f6ebeab9ee6e))


### v0.6.16 (2015-11-16)


#### Bug Fixes

* **response:** respond with a 500 if a handler panics ([63c6762c](https://github.com/hyperium/hyper/commit/63c6762c15ec790f54391a71794315599ae0ced8))


#### Features

* **headers:** Add Access-Control-Expose-Headers ([f783e991](https://github.com/hyperium/hyper/commit/f783e9913b988f3d5c28707e2291145999756dbe))
* **server:** Add hooks for HttpListener and HttpsListener to be started from existing listene ([fa0848d4](https://github.com/hyperium/hyper/commit/fa0848d4216aa81e7b7619b7ce0a650356ee7ab7))


#### Breaking Changes

* `RequestBuilder<U>` should be replaced by `RequestBuilder`.

 ([ff4a6070](https://github.com/hyperium/hyper/commit/ff4a6070573955d1623d51a3d5302a17eed8f8d6))


### v0.6.15 (2015-10-09)


#### Bug Fixes

* **server:** use a timeout for Server keep-alive ([cdaa2547](https://github.com/hyperium/hyper/commit/cdaa2547ed18dfb0e3b8ed2ca15cfda1f98fa9fc), closes [#368](https://github.com/hyperium/hyper/issues/368))


#### Features

* **client:** add patch method to Client builder interface ([03827c31](https://github.com/hyperium/hyper/commit/03827c3156b5c0a7c865c5846aca2c1ce7a9f4ce))


### v0.6.14 (2015-09-21)


#### Bug Fixes

* **http:**
  * Add a stream enum that makes it impossible to lose a stream ([be4e7181](https://github.com/hyperium/hyper/commit/be4e7181456844180963d0e5234656c319ce92a6))
  * Make sure not to lose the stream when CL is invalid ([a36e44af](https://github.com/hyperium/hyper/commit/a36e44af7d4e665a122c1498011ff10035f7376f))
* **server:** use EmptyWriter for status codes that have no body ([9b2998bd](https://github.com/hyperium/hyper/commit/9b2998bddc3c033e4fc4e6a9b7d18504339ded3f))
* **timeouts:** remove rust #![feature] for socket timeouts ([b8729698](https://github.com/hyperium/hyper/commit/b872969880be502b681def26d6b9780cc90ac74b))


#### Features

* **headers:** add PartialEq impl for Headers struct ([76cbf384](https://github.com/hyperium/hyper/commit/76cbf384231e602d888e49932bf9c4fafdd88051))


### v0.6.13 (2015-09-02)


#### Bug Fixes

* **client:** EofReader by nature means the connection is closed ([32e09a04](https://github.com/hyperium/hyper/commit/32e09a04292b0247456a8fb9003a75a6abaa998e))


### v0.6.12 (2015-09-01)


#### Bug Fixes

* **client:** be resilient to invalid response bodies ([75c71170](https://github.com/hyperium/hyper/commit/75c71170206db3119d9b298ea5cf3ee860803124), closes [#640](https://github.com/hyperium/hyper/issues/640))
* **examples:** "cargo test --features serde-serialization" ([63608c49](https://github.com/hyperium/hyper/commit/63608c49c0168634238a119eb64ea1074df1b7e6))
* **http:** fix several cases in HttpReader ([5c7195ab](https://github.com/hyperium/hyper/commit/5c7195ab4a213bf0016f2185a63a6341e4cef4de))


#### Features

* **server:** Add Handler per-connection hooks ([6b6182e8](https://github.com/hyperium/hyper/commit/6b6182e8c4c81f634becebe7b45dc21bff59a286))


### v0.6.11 (2015-08-27)


#### Bug Fixes

* **client:** fix panics when some errors occured inside HttpMessage ([ef15257b](https://github.com/hyperium/hyper/commit/ef15257b733d40bc3a7c598f61918f91385585f9))
* **headers:** case insensitive values for Connection header ([341f8eae](https://github.com/hyperium/hyper/commit/341f8eae6eb33e2242be09541807cdad9afc732e), closes [#635](https://github.com/hyperium/hyper/issues/635))


#### Breaking Changes

* This changes the signature of HttpWriter.end(),
  returning a `EndError` that is similar to std::io::IntoInnerError,
  allowing HttpMessage to retrieve the broken connections and not panic.

  The breaking change isn't exposed in any usage of the `Client` API,
  but for anyone using `HttpWriter` directly, since this was technically
  a public method, that change is breaking.

 ([ef15257b](https://github.com/hyperium/hyper/commit/ef15257b733d40bc3a7c598f61918f91385585f9))


### v0.6.10 (2015-08-19)


#### Bug Fixes

* **client:** close connection when there is an Error ([d32d35bb](https://github.com/hyperium/hyper/commit/d32d35bbea947172224082e1f9b711022ce75e30))


#### Features

* **uri:** implement fmt::Display for RequestUri () ([80931cf4](https://github.com/hyperium/hyper/commit/80931cf4c31d291125700ed3f9be5b3cb015d797), closes [#629](https://github.com/hyperium/hyper/issues/629))


### v0.6.9 (2015-08-13)


#### Bug Fixes

* **client:**
  * improve keep-alive of bodyless Responses ([67c284a9](https://github.com/hyperium/hyper/commit/67c284a96a006f888f43d8af929516465de76dea))
  * improve HttpReader selection for client Responses ([31f117ea](https://github.com/hyperium/hyper/commit/31f117ea08c01889016fd45e7084e9a049c53f7a), closes [#436](https://github.com/hyperium/hyper/issues/436))
* **nightly:** remove feature flag for duration ([0455663a](https://github.com/hyperium/hyper/commit/0455663a98d7969c23d64d0b775799342507ef8e))


#### Features

* **headers:** Content-Range header ([af062ac9](https://github.com/hyperium/hyper/commit/af062ac954d5b90275138880ce2f5013d6664b5a))
* **net:** impl downcast methods for NetworkStream (without + Send) ([1a91835a](https://github.com/hyperium/hyper/commit/1a91835abaa804aabf2e9bb45e9ab087274b8a18), closes [#521](https://github.com/hyperium/hyper/issues/521))
* **server:** add Request.ssl() to get underlying ssl stream ([7909829f](https://github.com/hyperium/hyper/commit/7909829f98bd9a2f454430f89b6143b977aedb35), closes [#627](https://github.com/hyperium/hyper/issues/627))


### v0.6.8 (2015-08-03)


#### Features

* **raw-fd:** implement FromRawFd/FromRawSocket ([664bde58](https://github.com/hyperium/hyper/commit/664bde58d8a6b2d6ce5624ed96b8d6d68214a782))


### v0.6.7 (2015-08-03)


#### Bug Fixes

* **headers:** fix broken deserialization of headers ([f5f5e1cb](https://github.com/hyperium/hyper/commit/f5f5e1cb2d01a22f170432e73b9c5757380cc18b))


#### Features

* **net:**
  * Implement NetworkConnector for closure to be more flexible ([abdd4c5d](https://github.com/hyperium/hyper/commit/abdd4c5d632059ebef9bbee95032c9500620212e))
  * add socket timeouts to Server and Client ([7d1f154c](https://github.com/hyperium/hyper/commit/7d1f154cb7b4db4a029b52857c377000a3f23419), closes [#315](https://github.com/hyperium/hyper/issues/315))


#### Breaking Changes

* Any custom implementation of NetworkStream must now
  implement `set_read_timeout` and `set_write_timeout`, so those will
  break. Most users who only use the provided streams should work with
  no changes needed.

Closes #315

 ([7d1f154c](https://github.com/hyperium/hyper/commit/7d1f154cb7b4db4a029b52857c377000a3f23419))


### v0.6.5 (2015-07-23)


#### Bug Fixes

* **tests:** iter.connect() is now iter.join() ([d2e8b5dc](https://github.com/hyperium/hyper/commit/d2e8b5dc3d2e6f0386656f4a5926acb848d4a61d))


#### Features

* **status:**
  * implement `Hash` for `StatusCode` ([d84f291a](https://github.com/hyperium/hyper/commit/d84f291abc0a64e270143eee943a76a7aebec029))
  * implement `Hash` for `StatusCode` ([aa85f609](https://github.com/hyperium/hyper/commit/aa85f609b5136cb2a9b23408a2b125c6a8a20f37))


### v0.6.4 (2015-07-23)


#### Features

* **http:** add optional serialization of common types via `serde` ([87de1b77](https://github.com/hyperium/hyper/commit/87de1b77bcd5533c70a6ab9379121001acc5d366))


### v0.6.3 (2015-07-08)


#### Bug Fixes

* **lint:** change deny(missing_docs) to only apply for tests ([5994a6f8](https://github.com/hyperium/hyper/commit/5994a6f8b4e48c9ab766e425dba210bdac59b00b), closes [#600](https://github.com/hyperium/hyper/issues/600))


### v0.6.2 (2015-07-06)


#### Bug Fixes

* **http:** no longer keep alive for Http1.0 if no Connection header ([ddecb262](https://github.com/hyperium/hyper/commit/ddecb262b39b90e594a95ba16c4dc8085930677e), closes [#596](https://github.com/hyperium/hyper/issues/596))


#### Features

* **client:** add url property Response ([82ed9092](https://github.com/hyperium/hyper/commit/82ed9092e30385de004912582a7838e037497c82))
* **headers:** add strict-transport-security header ([7c2e5124](https://github.com/hyperium/hyper/commit/7c2e5124e6678a5806f980902031e6f01652d218), closes [#589](https://github.com/hyperium/hyper/issues/589))


#### Breaking Changes

* Access-Control-Allow-Origin does no longer use Url

 ([ed458628](https://github.com/hyperium/hyper/commit/ed458628e54bd241b45f50fb0cf55a84ffb12205))
* Technically a break, since `Response::new()` takes an
  additional argument. In practice, the only place that should have been
  creating Responses directly is inside the Client, so it shouldn't
  break anyone. If you were creating Responses manually, you'll need to
  pass a Url argument.

 ([82ed9092](https://github.com/hyperium/hyper/commit/82ed9092e30385de004912582a7838e037497c82))


### v0.6.1 (2015-06-26)


#### Bug Fixes

* **benches:** adjust to missing `set_ssl_verifier` ([eb38a11b](https://github.com/hyperium/hyper/commit/eb38a11b9ab401d6b909077f92507fa872349d13))
* **cargo:** fix linking on OSX 10.10 ([9af2b66f](https://github.com/hyperium/hyper/commit/9af2b66fe4003706517d95ed94013af9cd365b24))
* **client:** use Ssl instance in creation of SslStream ([1a490e25](https://github.com/hyperium/hyper/commit/1a490e25c321bdd173d47ed7a7a704039746fb29))


## v0.6.0 (2015-06-24)


#### Bug Fixes

* **client:** check for drained stream in Response::drop ([e689f203](https://github.com/hyperium/hyper/commit/e689f20376d3e078f5d380902d39f8ae9c043486))


#### Features

* **client:**
  * impl Sync for Client ([64e47b4b](https://github.com/hyperium/hyper/commit/64e47b4bbd0433065a059804adeb2b4a2d72f327), closes [#254](https://github.com/hyperium/hyper/issues/254))
  * implement Protocol trait for HTTP/1.1 ([dccdf8d6](https://github.com/hyperium/hyper/commit/dccdf8d65a9b900daec34555d3b97c2c3c678067))
  * add `Protocol` trait ([3417303a](https://github.com/hyperium/hyper/commit/3417303a4a9aa4809729d53f0d018338e876da51))
  * implement HttpMessage for HTTP/1.1 ([ecb713f8](https://github.com/hyperium/hyper/commit/ecb713f8494b13bdba91258b1507e8f7ce62b8d9))
  * add `HttpMessage` trait ([289fd02b](https://github.com/hyperium/hyper/commit/289fd02b55a42748cbce8de428939208713a765d))
* **error:** add private `__Nonexhaustive` variant to Error ([7c0421e3](https://github.com/hyperium/hyper/commit/7c0421e3fc1d5a8b4868b57acca87abd685f3430))
* **headers:**
  * add bearer token support ([edf6ac20](https://github.com/hyperium/hyper/commit/edf6ac2074d11694ded275807a66df3a8a8e33a6))
  * add `Range` header ([05c31998](https://github.com/hyperium/hyper/commit/05c319984630b31d18dfbfa9b7567f6c7613d7f8))
* **http2:**
  * implement message API for HTTP/2 ([f0fe2c5a](https://github.com/hyperium/hyper/commit/f0fe2c5a83bd4e654a4ff684f75a1b602f8f38fc))
  * add new error variant for HTTP/2 ([48e9ca2f](https://github.com/hyperium/hyper/commit/48e9ca2f70f6c6475f1579ae9212af7b4ca87e88))
  * add dependency on `solicit` ([3122ffef](https://github.com/hyperium/hyper/commit/3122ffefc2d56ffc03a6fcc264086df0c9d74083))
* **langtags:** use true language tags in headers ([99ff7e62](https://github.com/hyperium/hyper/commit/99ff7e62573865a1fc431db26b6a18c43b9127de))
* **ssl:** redesign SSL usage ([53bba6eb](https://github.com/hyperium/hyper/commit/53bba6eb7f34e61e5c8a835281d625436532de8f))


#### Breaking Changes

* AcceptLanguage and ContentLanguage use LanguageTag now,
Language removed from Hyper.

 ([99ff7e62](https://github.com/hyperium/hyper/commit/99ff7e62573865a1fc431db26b6a18c43b9127de))
* Server::https was changed to allow any implementation
  of Ssl. Server in general was also changed. HttpConnector no longer
  uses SSL; using HttpsConnector instead.

 ([53bba6eb](https://github.com/hyperium/hyper/commit/53bba6eb7f34e61e5c8a835281d625436532de8f))
* Connectors and Protocols passed to the `Client` must
  now also have a `Sync` bounds, but this shouldn't break default usage.

 ([64e47b4b](https://github.com/hyperium/hyper/commit/64e47b4bbd0433065a059804adeb2b4a2d72f327))
* parse_header returns Result instead of Option, related
code did also change

 ([195a89fa](https://github.com/hyperium/hyper/commit/195a89fa918a83c9dcab47a4b09edb464d4e8006))
* Adds a new variant to public Error enum. The proper fix
  is to stop matching exhaustively on `hyper::Error`.

 ([7c0421e3](https://github.com/hyperium/hyper/commit/7c0421e3fc1d5a8b4868b57acca87abd685f3430))
* A new variant `Http2` added to a public enum
`hyper::Error`.

 ([48e9ca2f](https://github.com/hyperium/hyper/commit/48e9ca2f70f6c6475f1579ae9212af7b4ca87e88))
* `hyper::client::request::Response` is no longer generic
over `NetworkStream` types. It no longer requires a generic type
parameter at all.

 ([aa297f45](https://github.com/hyperium/hyper/commit/aa297f45322d66980bb2b51c413b15dfd51533ea))


### v0.5.2 (2015-06-01)


#### Bug Fixes

* **buffer:** check capacity before resizing ([b1686d1b](https://github.com/hyperium/hyper/commit/b1686d1b22aa95a17088f99054d577bbb2aef9dc))


### v0.5.1 (2015-05-25)


#### Bug Fixes

* **client:** don't close stream until EOF ([a5e6174e](https://github.com/hyperium/hyper/commit/a5e6174efd57afb1df7113c64f4e7718a3a94187), closes [#543](https://github.com/hyperium/hyper/issues/543))


#### Features

* **client:** implement Default trait for client ([be041d91](https://github.com/hyperium/hyper/commit/be041d915a55fa1b5088e112b81727b864949976))
* **header:** add ContentType::form_url_encoded() constructor ([2c99d4e9](https://github.com/hyperium/hyper/commit/2c99d4e9068b30ecb6d4eac4d364924fb253fdcd))
* **headers:** return hyper::Error instead of () from header components ([5d669399](https://github.com/hyperium/hyper/commit/5d669399b6ca5ec7d0f01b9d30513cd1cc4cc47b))
* **http:** add get_mut method to HttpReader ([e64ce8c0](https://github.com/hyperium/hyper/commit/e64ce8c05e847b2396e4b7e2bb656240e9806ed8))


#### Breaking Changes

* Error enum extended. Return type of header/shared/
types changed.

 ([5d669399](https://github.com/hyperium/hyper/commit/5d669399b6ca5ec7d0f01b9d30513cd1cc4cc47b))


## v0.5.0 (2015-05-12)


#### Bug Fixes

* **client:**
  * dont call close() inside Request ([3334fca2](https://github.com/hyperium/hyper/commit/3334fca278e662b2755e41045ce641238514bea9), closes [#519](https://github.com/hyperium/hyper/issues/519))
  * keep the underlying connector when setting an SSL verifier ([f4556d55](https://github.com/hyperium/hyper/commit/f4556d554faa2a1170fec0af5b4076c31e7c3600), closes [#495](https://github.com/hyperium/hyper/issues/495))
* **mock:** adjust ChannelMockConnector connect method to compile ([085d7b07](https://github.com/hyperium/hyper/commit/085d7b0752d7fc0134e99e9eec2a67cc66b319b3))


#### Features

* **header:**
  * add ContentType::json(), plaintext(), html(), jpeg(), and png() constructors ([b6114ecd](https://github.com/hyperium/hyper/commit/b6114ecd2e65bd59e79a67a45913adaf0f1552f0))
  * add Connection::close() and ::keep_alive() constructors ([c2938fb4](https://github.com/hyperium/hyper/commit/c2938fb45f9c1fff2a1235d82b7741531de21445))
  * export __hyper__tm! macro so test modules work with header! ([f64fb10b](https://github.com/hyperium/hyper/commit/f64fb10bc87bb4b5a5291d09364ad6c725a842d8))
* **net:**
  * remove mut requirement for NetworkConnector.connect() ([1b318724](https://github.com/hyperium/hyper/commit/1b318724a5fd425366daddf15c5964d7c3cbc240))
  * add `set_ssl_verifier` method to `NetworkConnector` trait ([a5d632b6](https://github.com/hyperium/hyper/commit/a5d632b6ea53d0988d6383dd734d0b5e6245ba2b))
* **server:** check Response headers for Connection: close in keep_alive loop ([49b5b8fd](https://github.com/hyperium/hyper/commit/49b5b8fdfe256ead8f3aa3d489bc4b299c190a9a))


#### Breaking Changes

* Usage of Response.deconstruct() and construct() now use
  a &mut Headers, instead of the struct proper.

 ([49b5b8fd](https://github.com/hyperium/hyper/commit/49b5b8fdfe256ead8f3aa3d489bc4b299c190a9a))
* If you use deref! from the header module, you'll need
  to switch to using __hyper__deref!.

 ([62d96adc](https://github.com/hyperium/hyper/commit/62d96adc6b852b3836b47fc2e154bbdbab9ad7f6))
* Any custom Connectors will need to change to &self in
  the connect method. Any Connectors that needed the mutablity need to
  figure out a synchronization strategy.

  Request::with_connector() takes a &NetworkConnector instead of &mut.
  Any uses of with_connector will need to change to passing &C.

 ([1b318724](https://github.com/hyperium/hyper/commit/1b318724a5fd425366daddf15c5964d7c3cbc240))
* Adding a new required method to a public trait is a
breaking change.

 ([a5d632b6](https://github.com/hyperium/hyper/commit/a5d632b6ea53d0988d6383dd734d0b5e6245ba2b))


## v0.4.0 (2015-05-07)


#### Bug Fixes

* **net:** ignore NotConnected error in NetworkStream.close ([6be60052](https://github.com/hyperium/hyper/commit/6be60052c627b7e498d973465b4a3ee7efc40665), closes [#508](https://github.com/hyperium/hyper/issues/508))


#### Features

* **error:** add Ssl variant to hyper::Error ([972b3a38](https://github.com/hyperium/hyper/commit/972b3a388ac3af98ba038927c551b92be3a68d62), closes [#483](https://github.com/hyperium/hyper/issues/483))
* **headers:**
  * Allow `null` value in Access-Control-Allow-Origin ([5e341714](https://github.com/hyperium/hyper/commit/5e3417145ced116147ef1e890b4f1e7c775ad173))
  * Parse Upgrade header protocols further ([f47d11b9](https://github.com/hyperium/hyper/commit/f47d11b97bb4a4bf67c3f9aa47c203babf4a9c72), closes [#480](https://github.com/hyperium/hyper/issues/480))
  * Add From header field ([ce9c4af1](https://github.com/hyperium/hyper/commit/ce9c4af1e0a46abc9f7908c2cb0659a2ecab137c))
  * Add Accept-Ranges header field ([2dbe3f9b](https://github.com/hyperium/hyper/commit/2dbe3f9b9a3fc9f04346712e55f40dabaf72d9a8))
* **method:** implement `AsRef<str>` for `Method` ([c29af729](https://github.com/hyperium/hyper/commit/c29af729726ae782bece5e790bce02b0d3ab9ef9))
* **server:**
  * add Response.send to write a sized body ([d5558b68](https://github.com/hyperium/hyper/commit/d5558b687d32d0affb9aaa7185227a4e294f5454), closes [#446](https://github.com/hyperium/hyper/issues/446))
  * dropping a Response will write out to the underlying stream ([a9dcc59c](https://github.com/hyperium/hyper/commit/a9dcc59cd9846609a5733678f66353655c075279))


#### Breaking Changes

* Adds a variant to `hyper::Error`, which may break any
exhaustive matches.

 ([972b3a38](https://github.com/hyperium/hyper/commit/972b3a388ac3af98ba038927c551b92be3a68d62))
* The terms `Http` and `Error` have been removed from the Error
  type and its variants. `HttpError` should now be accessed as `hyper::Error`,
  and variants like `HttpIoError` should be accessed as `Error::Io`.

 ([9ba074d1](https://github.com/hyperium/hyper/commit/9ba074d150a55a749161317405fe8b28253c5a9d))
* Add variant to Access-Control-Allow-Origin enum

 ([5e341714](https://github.com/hyperium/hyper/commit/5e3417145ced116147ef1e890b4f1e7c775ad173))
* Upgrade header Protocol changed.

 ([f47d11b9](https://github.com/hyperium/hyper/commit/f47d11b97bb4a4bf67c3f9aa47c203babf4a9c72))
* `from_one_raw_str()` returns `None` on empty values.

 ([a6974c99](https://github.com/hyperium/hyper/commit/a6974c99d39fcbaf3fb9ed38428b21e0301f3602))


### v0.3.16 (2015-05-01)


#### Bug Fixes

* **header:**
  * make test_module of header! optional ([a5ce9c59](https://github.com/hyperium/hyper/commit/a5ce9c59fa61410551b07252364564a2bb13bb86), closes [#490](https://github.com/hyperium/hyper/issues/490))
  * exporting test_header! macro ([2bc5a779](https://github.com/hyperium/hyper/commit/2bc5a779bdc3fce67e06c398ac8702fcbea93dab))
* **http:** keep raw reason phrase in RawStatus ([8cdb9d5d](https://github.com/hyperium/hyper/commit/8cdb9d5d3b0972629e8843d3c1db58dbbbaf49cf), closes [#497](https://github.com/hyperium/hyper/issues/497))


#### Features

* **client:** add a Connection Pool ([1e72a8ab](https://github.com/hyperium/hyper/commit/1e72a8ab3a0092bb863686ad2e65646710706c1b), closes [#363](https://github.com/hyperium/hyper/issues/363), [#41](https://github.com/hyperium/hyper/issues/41))
* **headers:** Add If-Range header ([a39735f1](https://github.com/hyperium/hyper/commit/a39735f1d3d1a314969b5b0085e8f77f0c10c863), closes [#388](https://github.com/hyperium/hyper/issues/388))


### v0.3.15 (2015-04-29)


#### Bug Fixes

* **headers:**
  * Do not parse empty values in list headers. ([093a29ba](https://github.com/hyperium/hyper/commit/093a29bab7eb27e78bb10506478ac486e8d61671))
  * Fix formatting of 0 qualites and formatting of empty list header fields. ([621ef521](https://github.com/hyperium/hyper/commit/621ef521f6723ba2d59beff05ff39ae8fd6df2c3))


#### Features

* **client:**
  * remove Clone requirement for NetworkStream in Client ([60d92c29](https://github.com/hyperium/hyper/commit/60d92c296a445b352679919c03c5ed2a2a297e16))
  * accept &String as Body in RequestBuilder ([a2aefd9a](https://github.com/hyperium/hyper/commit/a2aefd9a5689d4816f7c054bd6c32aa5c6fe3087))
  * accept &String for a Url in RequestBuilder ([8bc179fb](https://github.com/hyperium/hyper/commit/8bc179fb517735a7c1d5cd1d7f5598bb82914dc6))
* **headers:** Implement Content-Language header field ([308880b4](https://github.com/hyperium/hyper/commit/308880b455df4dbb5d32817b5c0320c2a88139e3), closes [#475](https://github.com/hyperium/hyper/issues/475))
* **net:** add https_using_context for user-supplied SslContext ([1a076d1b](https://github.com/hyperium/hyper/commit/1a076d1bc7e8fb9c58904b0cec879dcf0fbce97b))
* **server:** allow consumer to supply an SslContext ([3a1a2427](https://github.com/hyperium/hyper/commit/3a1a24270dd13e22ef59120d66d327528949d5e0), closes [#471](https://github.com/hyperium/hyper/issues/471))


#### Breaking Changes

* This removes the trait `IntoBody`, and instead using
  `Into<Body>`, as it's more idiomatic. This will only have broken code
  that had custom implementations of `IntoBody`, and can be fixed by
  changing them to `Into<Body>`.

 ([a2aefd9a](https://github.com/hyperium/hyper/commit/a2aefd9a5689d4816f7c054bd6c32aa5c6fe3087))


### v0.3.14 (2015-04-18)


#### Bug Fixes

* **http:** Adjust httparse Request and Response lifetimes. ([76550fdb](https://github.com/hyperium/hyper/commit/76550fdb20bb812e92a1fc3f3a7eaaf4a689348b))


### v0.3.13 (2015-04-17)


#### Bug Fixes

* **server:** JoinHandle type parameter ([c694b138](https://github.com/hyperium/hyper/commit/c694b1385bd294e7c8e0398ee75e3a054ced5006))


#### Features

* **debug:** add Debug impls for StatusClass, Server, and Listening ([0fb92ee7](https://github.com/hyperium/hyper/commit/0fb92ee735136a07c832124df521b96a6779bd39))


### v0.3.12 (2015-04-15)


#### Bug Fixes

* **server:**
  * handle keep-alive closing ([d9187713](https://github.com/hyperium/hyper/commit/d9187713b2eaa628eb34f68c8a7201a6cf8e010d), closes [#437](https://github.com/hyperium/hyper/issues/437))
  * join on thread when Listening drops ([68d4d63c](https://github.com/hyperium/hyper/commit/68d4d63c2a0289b72ec1442d13e1212a0479c50b), closes [#447](https://github.com/hyperium/hyper/issues/447))
  * Use thread::spawn instead of thread::scoped. ([e8649567](https://github.com/hyperium/hyper/commit/e864956734af72bab07a3e01c9665bc1b7c96e5e))


#### Features

* **http:** Implement Debug for HttpReader/Writer. ([2f606c88](https://github.com/hyperium/hyper/commit/2f606c88bd91e5e36dee4c6db00c3117b1adf067))
* **log:** clean up logging ([4f09b002](https://github.com/hyperium/hyper/commit/4f09b002ffb2d076fc8fb01d9b9e0464216b2b41))
* **net:** make HttpStream implement Debug ([7b7f9c25](https://github.com/hyperium/hyper/commit/7b7f9c257d0e2d515bf336c567f12a625471e477))


### v0.3.11 (2015-04-15)


#### Bug Fixes

* **headers:** Content-Encoding needs a hyphen. ([ca2815ef](https://github.com/hyperium/hyper/commit/ca2815effda2a5b27f781b7bc35105aa81121bae))


#### Features

* **client:** remove generic parameter for Connector ([139a51f1](https://github.com/hyperium/hyper/commit/139a51f1c31b80cdddf643e984bbbfbb3d3e8c96), closes [#379](https://github.com/hyperium/hyper/issues/379))


#### Breaking Changes

* `AccessControlAllowHeaders` and `AccessControlRequestHeaders` values
are case insensitive now. `AccessControlAllowOrigin` variants are now `Any` and
`Value` to match the other headers.

 ([94f38950](https://github.com/hyperium/hyper/commit/94f38950ddf9a97fdc4f44e42aada4ed8f4d9b43))
* `If-Match`, `If-None-Match` and `Vary` item variant name changed to `Items`

 ([38d297b1](https://github.com/hyperium/hyper/commit/38d297b16e5d14d533947988f770f03b49d47a17))
* `Etag` header field is now `ETag` header field

 ([4434ea6a](https://github.com/hyperium/hyper/commit/4434ea6a7d57d367c0a541c82f6289ffbda5fb6c))
* For people using the default HttpConnector and Client,
    everything should continue to just work. If the Client has been
    used with a generic parameter, it should be removed.

    However, there were some breaking changes to the internals of
    NetworkConnectors. Specifically, they no longer return a
    NetworkStream, but instead a Into<Box<NetworkStream + Send>>. All
    implementations of NetworkStream should continue to just work,
    however.

    Possible breakages could come from the stricter usage of Send
    throughout the Client API.

 ([139a51f1](https://github.com/hyperium/hyper/commit/139a51f1c31b80cdddf643e984bbbfbb3d3e8c96))


### v0.3.10 (2015-04-06)


#### Bug Fixes

* **README:** Update to compile example against Rust beta ([341f19d3](https://github.com/hyperium/hyper/commit/341f19d3266c6de9a9a90c94f718124792766630))


### v0.3.9 (2015-04-03)


#### Bug Fixes

* **headers:** Add CowStr as a temporary hack to build on beta. ([8e065563](https://github.com/hyperium/hyper/commit/8e0655637e80c5377c01da4dbca6fb627e6d4225))


### v0.3.8 (2015-04-02)


#### Bug Fixes

* **rustup:** update to rust beta ([0f5858f3](https://github.com/hyperium/hyper/commit/0f5858f37974731243d47710364776fdd73376fe))


#### Breaking Changes

* Removed impl_header!() and impl_list_header!() macros,
use new header!() macro.

 ([262c450f](https://github.com/hyperium/hyper/commit/262c450f908dbf27754daff0784f0f20145036dd))


### v0.3.7 (2015-03-31)


#### Bug Fixes

* **buffer:** zero out new capacity when buffer grows ([cfdabd70](https://github.com/hyperium/hyper/commit/cfdabd70ecc3f5290ae1e6f7e5dfd50310d8658d))


#### Features

* **entitytag:** Add EntityTag comparison, make EntityTag safe to use ([9c21f7f9](https://github.com/hyperium/hyper/commit/9c21f7f953a5163792e71fb186cab391c45d1bb4))


### v0.3.6 (2015-03-30)


#### Bug Fixes

* **buffer:** get_buf to not return consumed part of buffer ([04e3b565](https://github.com/hyperium/hyper/commit/04e3b5651561f087fee7c0345fe77d217d3ad35a), closes [#406](https://github.com/hyperium/hyper/issues/406))
* **rustup:** get rid of slice pattern, add `Reflect` bounds ([c9f2c841](https://github.com/hyperium/hyper/commit/c9f2c841ff0e68dead38e762ed5f8c0f42255bc4))


### v0.3.5 (2015-03-28)


#### Bug Fixes

* **http:** read more before triggering TooLargeError ([cb59f609](https://github.com/hyperium/hyper/commit/cb59f609c61a097d5d9fa728b9df33d79922573b), closes [#389](https://github.com/hyperium/hyper/issues/389))


### v0.3.4 (2015-03-26)


#### Bug Fixes

* **rustup:** static bounds required on Type definition, trivial_casts ([eee7a85d](https://github.com/hyperium/hyper/commit/eee7a85d3c3a3f51a1c3c12496c0e45ea312524e))


### v0.3.3 (2015-03-25)


#### Bug Fixes

* **rustup:**
  * rustc 1.0.0-nightly (123a754cb 2015-03-24) ([3e456f00](https://github.com/hyperium/hyper/commit/3e456f00f9991b1c723a232fc9c76fe8c0539858))
  * 1.0.0-nightly (e2fa53e59 2015-03-20) ([f547080d](https://github.com/hyperium/hyper/commit/f547080df53076711b52a016b990c5be56f42ede))


#### Features

* **headers:** Implementing content-encoding header ([2983e8de](https://github.com/hyperium/hyper/commit/2983e8dea21f02a31012a25b0a302a128768030a), closes [#391](https://github.com/hyperium/hyper/issues/391))


### v0.3.2 (2015-03-20)


#### Bug Fixes

* **benches:** removed unused features ([104d4903](https://github.com/hyperium/hyper/commit/104d49036ff40c730ec8bef8012f19ccbee4aaae))
* **rustup:**
  * rustc 1.0.0-nightly (ea8b82e90) ([8181de25](https://github.com/hyperium/hyper/commit/8181de253aecfe81123e166a141ebfc8430ec4a4))
  * adapt to current rustc ([1f0bc951](https://github.com/hyperium/hyper/commit/1f0bc951c9ee40cab622a72d614d4c45d889ccd3), closes [#381](https://github.com/hyperium/hyper/issues/381))


#### Features

* **server:** use SocketAddrs instead of Ipv4Addrs ([5d7be77e](https://github.com/hyperium/hyper/commit/5d7be77e4ac0d5c1d852c1208abc77a913c4f4d1))


### v0.3.1 (2015-03-18)


#### Bug Fixes

* **header:** Fix charset parsing bug. ([5a6e176f](https://github.com/hyperium/hyper/commit/5a6e176f50fe667fbdc4c933c81d2db5ba5c571d))
* **headers:** Fix overflow with empty cookies ([99baaa10](https://github.com/hyperium/hyper/commit/99baaa10157f6c69ef1795a97e0db8bd794011f6))
* **rustup:** update to latest rustc ([4fd8a6a9](https://github.com/hyperium/hyper/commit/4fd8a6a9dc0dc969b36f3d3ad51cee177545f883))


#### Features

* **server:** add Expect 100-continue support ([0b716943](https://github.com/hyperium/hyper/commit/0b7169432b5f51efe5c167be418c2c50220e46a5), closes [#369](https://github.com/hyperium/hyper/issues/369))


#### Breaking Changes

* Several public functions and types in the `http` module
  have been removed. They have been replaced with 2 methods that handle
  all of the http1 parsing.

 ([b87bb20f](https://github.com/hyperium/hyper/commit/b87bb20f0c25891c30ef2399da2721596fbc1fcf))


## v0.3.0 (2015-03-03)


#### Features

* **headers:**
  * add enum for Charset ([180d9a92](https://github.com/hyperium/hyper/commit/180d9a92d92541aa415c918a2265bd6b33d39655))
  * add AcceptCharset header ([235089a1](https://github.com/hyperium/hyper/commit/235089a1034dc93ca62f47dcab0a93f1d49c72dd))
  * add q function to ease creating Quality values ([d68773c7](https://github.com/hyperium/hyper/commit/d68773c79f998813bbd1bf50a0dbc2bc01ee0470))
  * adds re-parsing ability when getting typed headers ([df756871](https://github.com/hyperium/hyper/commit/df756871edf4143135644c211106c5a8f8f5adb0))
* **hyper:** switch to std::io, std::net, and std::path. ([0fd6fcd7](https://github.com/hyperium/hyper/commit/0fd6fcd7c7f30c4317678a3b0968cc08ae9c0a71), closes [#347](https://github.com/hyperium/hyper/issues/347))


#### Breaking Changes

* added requirement that all HeaderFormat implementations
  must also be fmt::Debug. This likely as easy as slapping
  #[derive(Debug)] on to any custom headers.

 ([df756871](https://github.com/hyperium/hyper/commit/df756871edf4143135644c211106c5a8f8f5adb0))
* Check the docs. Everything was touched.

 ([0fd6fcd7](https://github.com/hyperium/hyper/commit/0fd6fcd7c7f30c4317678a3b0968cc08ae9c0a71))


### v0.2.1 (2015-02-27)


#### Bug Fixes

* **rustup:** str.split and associated type changes ([1b6e6a04](https://github.com/hyperium/hyper/commit/1b6e6a040fa26a8b3855ac46ccbcd5ee78065c71))


#### Features

* **headers:** add remove_raw method and corresponding test ([4f576780](https://github.com/hyperium/hyper/commit/4f576780c24ff3f943d5f821730ba65f4cdf8d4a), closes [#326](https://github.com/hyperium/hyper/issues/326))


## v0.2.0 (2015-02-21)


#### Bug Fixes

* **headers:** use $crate when referring to hyper modules on macros ([e246c3ac](https://github.com/hyperium/hyper/commit/e246c3ace8395cb5d281b841a416c503db1054ee), closes [#323](https://github.com/hyperium/hyper/issues/323))
* **rustup:**
  * Send changes ([4f5b97fe](https://github.com/hyperium/hyper/commit/4f5b97fefcea650214ca26c1aa197cd73683742f))
  * CowString is gone ([98b8c4b1](https://github.com/hyperium/hyper/commit/98b8c4b13723d8fa1b4f1ba42a06bb533bf13694))
  * Extend now takes an IntoIterator ([598d8f93](https://github.com/hyperium/hyper/commit/598d8f93e4a79dcc5ff58fbdc27e6b1a859786d1))
  * Add PhantomData markers to phantom type users ([1904c456](https://github.com/hyperium/hyper/commit/1904c4561f00a345714beadfa077016306b2c05d))
  * Remove uses of the obsolete &a[] syntax ([039e984f](https://github.com/hyperium/hyper/commit/039e984f6878d724d47f7e9fe7db765495ae2f10))
  * Fix signature of IntoCow ([234fcdc3](https://github.com/hyperium/hyper/commit/234fcdc3a25deb06240848d601be9e68930a73e6))
  * update feature flags ([b47f9365](https://github.com/hyperium/hyper/commit/b47f936525dde91b3456078ecf8d0c11917cc6b7))
  * use module-level thread functions ([fc2076cd](https://github.com/hyperium/hyper/commit/fc2076cd53c37ea244a0b89d7dd4b1eb8aeeb1d3))
  * update lifetime bounds ([f4a66b38](https://github.com/hyperium/hyper/commit/f4a66b38cb9e35bfec0bbc3c97e5298fc8ad8409))


#### Features

* **server:** make AcceptorPool::accept() block and allow non'-static data ([b0a72d80](https://github.com/hyperium/hyper/commit/b0a72d80d0e894220da6aa5ea29d71b278df596d))


### v0.1.13 (2015-02-17)


#### Bug Fixes

* **server:** Drain requests on drop. ([3d0f423e](https://github.com/hyperium/hyper/commit/3d0f423eb26c4f14aaf9f8a909b307f661a3c5d6), closes [#197](https://github.com/hyperium/hyper/issues/197), [#309](https://github.com/hyperium/hyper/issues/309))


#### Features

* **header:** Support arbitrary status codes ([73978531](https://github.com/hyperium/hyper/commit/7397853148b8221c0eb8315ae2e5f195ad2e642c))
* **headers:**
  * Implement PartialOrd for QualityItem ([2859d7ef](https://github.com/hyperium/hyper/commit/2859d7ef4ecadc3927fa46292ebbb225da597690), closes [#314](https://github.com/hyperium/hyper/issues/314))
  * add AcceptLanguage header ([20a585e3](https://github.com/hyperium/hyper/commit/20a585e30bbb060a91839de7e95fd75a95d03d93))
  * add IfMatch header ([5df06d44](https://github.com/hyperium/hyper/commit/5df06d4465fae01ef08b926f1f3be9f32a0f5c80))
* **server:** Rewrite the accept loop into a custom thread pool. ([3528fb9b](https://github.com/hyperium/hyper/commit/3528fb9b015a0959268452d5b42d5544c7b98a6a))


#### Breaking Changes

* This removes unregistered status codes from the enum. Use
`FromPrimitive` methods to create them now. StatusCode and StatusClass can no
longer be casted to `u16`, use `ToPrimitive` methods now.
For example `status.to_u16().unwrap()` to get the status code number.

 ([73978531](https://github.com/hyperium/hyper/commit/7397853148b8221c0eb8315ae2e5f195ad2e642c))


### v0.1.12 (2015-02-13)


#### Bug Fixes

* **net:** don't stop the server when an SSL handshake fails with EOF ([55f12660](https://github.com/hyperium/hyper/commit/55f12660891812d13a59e799b0ab5b185926479a))


#### Features

* **headers:** Add `If-None-Match` header field ([318b067a](https://github.com/hyperium/hyper/commit/318b067a06ecb42f0fba51928675d3b4291c7643), closes [#238](https://github.com/hyperium/hyper/issues/238))


### v0.1.11 (2015-02-06)


#### Bug Fixes

* **readme:** Make the README client example work ([9b5d6aab](https://github.com/hyperium/hyper/commit/9b5d6aab7e68cf776618151e9e69e34fd66aba6c))


#### Features

* **headers:** add IfUnmodifiedSince header ([b5543b67](https://github.com/hyperium/hyper/commit/b5543b67525e3d6ebc655d7e1736c8ade5b6dbb0))


#### Breaking Changes

* for any consumers of the Etag header, since the entity
tag is now in a tuple.

 ([28fd5c81](https://github.com/hyperium/hyper/commit/28fd5c81f54bb0ea3eda43a4014c736d00b4b07d))


### v0.1.10 (2015-02-03)


#### Bug Fixes

* **headers:** add limit to maximum header size that should be parsed ([f18a8fb7](https://github.com/hyperium/hyper/commit/f18a8fb76f15f36dec329683abb66be203ab2e7e), closes [#256](https://github.com/hyperium/hyper/issues/256))
* **rustup:**
  * update FromStr ([742081c8](https://github.com/hyperium/hyper/commit/742081c8cfeeb59908a653316a6377d05ffaa55c))
  * fix unused_feature warning in example server ([05a3a6b7](https://github.com/hyperium/hyper/commit/05a3a6b70badc28da33ff65e8c15003f87738e07))
  * switch to unstable features ([3af8b687](https://github.com/hyperium/hyper/commit/3af8b687d4a6ef462eb74b1f5a1cbb8f191902fd))


### v0.1.9 (2015-01-28)


#### Bug Fixes

* **headers:** Don't display q if q=1 in quality item. ([91df2441](https://github.com/hyperium/hyper/commit/91df2441a0bb8c032b6fc5ccff50ed0eb98f2194), closes [#281](https://github.com/hyperium/hyper/issues/281))
* **rustup:** update io import, Writer::write ([f606b603](https://github.com/hyperium/hyper/commit/f606b6039d15a0b6e46f5154a9c5482866497a0c))


#### Features

* **status:** add is_<status_class>() methods to StatusCodes ([2d55a22e](https://github.com/hyperium/hyper/commit/2d55a22e738fb7f37a271be4fc3cf2ebdb9b5345))


### v0.1.8 (2015-01-27)


#### Bug Fixes

* **headers:**
  * make ConnectionHeader unicase ([e06e7d9a](https://github.com/hyperium/hyper/commit/e06e7d9a7ece9588b673b06df6aec4663595df30))
  * make Protocol search websocket unicase ([65c70180](https://github.com/hyperium/hyper/commit/65c7018046eb556085ca47a28c980ec901980643))
* **log:** update to new logging levels ([b002b6c3](https://github.com/hyperium/hyper/commit/b002b6c3f09775e5d6759bbd07dacdee318c2915))


#### Features

* **headers:** Add `Pragma` header field ([767c95d2](https://github.com/hyperium/hyper/commit/767c95d2b9709b496b35d0d691ff7a1f6d35cbed), closes [#237](https://github.com/hyperium/hyper/issues/237))


#### Breaking Changes

* Change header `Cookie` to `Cookie`

 ([92f43cf8](https://github.com/hyperium/hyper/commit/92f43cf873ddceca9518195af6dad1ff6ac79e11))


### v0.1.7 (2015-01-27)


#### Bug Fixes

* **rustup:** update to newest fmt trait names and slice syntax ([9e3c94d7](https://github.com/hyperium/hyper/commit/9e3c94d764522f900731fdbdee857639901037fe))


#### Breaking Changes

* Implementations of Header will need to adjust the
    header_name method. It no longer takes any arguments.

 ([8215889e](https://github.com/hyperium/hyper/commit/8215889eda537d09da82a7ed12a1766bf4fd3bfe))


### v0.1.6 (2015-01-27)


#### Bug Fixes

* **headers:** make Schemes, Basic, Protocol public ([e43c35c1](https://github.com/hyperium/hyper/commit/e43c35c1ca86c0ff1278ccfe3d2cff43222627b2))


### v0.1.5 (2015-01-27)


### v0.1.4 (2015-01-27)


#### Bug Fixes

* **imports:** Update TypeID import location to "any" ([dd2534a6](https://github.com/hyperium/hyper/commit/dd2534a6863f8b3940d2776e6b6a8e48988b9b88))


### v0.1.3 (2015-01-27)


#### Features

* **server:** add a deconstruct method to Request. ([1014855f](https://github.com/hyperium/hyper/commit/1014855faec62ba00acdff6263c86e7dfa5fb047))


### v0.1.2 (2015-01-27)


#### Bug Fixes

* **server:** Increase MAX_HEADER_FIELD_LENGTH to 4k ([54238b28](https://github.com/hyperium/hyper/commit/54238b28e4899e76bb3d7c2dfd8d9bc6fd489b6c))


#### Features

* **net:**
  * Move SSL verification to unboxed closures ([bca9a53c](https://github.com/hyperium/hyper/commit/bca9a53c66c967affb8e245f26507494db39c35e))
  * Allow more generic SSL verification () ([af577851](https://github.com/hyperium/hyper/commit/af5778510d1d8422fcb04873f7c726a67f15f5eb), closes [#244](https://github.com/hyperium/hyper/issues/244))


### 0.1.1 (2015-01-13)

#### Features

* **server:**: Add TLS/SSL support serverside ([c6eef681](c6eef6812458e10de582530d7f2c5bce5156b73c), closes [#1](https://github.com/hyperium/hyper/issues/1))


#### Bug Fixes

* **headers:**
    * fix fmt_header outputs of several headers ([aa266653](https://github.com/hyperium/hyper/commit/aa26665367bde895ce02ad2a8e1a372f00719852), closes [#246](https://github.com/hyperium/hyper/issues/246))
    * don't use Show to write UserAgent header ([c8e334aa](https://github.com/hyperium/hyper/commit/c8e334aaebb5522a86d47f7e3c33836d2061cb65))

