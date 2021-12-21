# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.17] 2021-12-16

- Bump poem to `1.2.2`.

## [3.0.16] 2021-12-16

- Bump poem to `1.2.1`.

## [3.0.15] 2021-12-12

- Bump actix-web from `4.0.0-beta.11` to `4.0.0-beta.14`.

## [3.0.14] 2021-12-06

- [async-graphql-axum] bump axum from `0.3` to `0.4`.

## [3.0.13] 2021-12-06

- No longer assumes that a subscription stream that failed to resolve has ended. [#744](https://github.com/async-graphql/async-graphql/issues/744)
- Rework to implement `InputType` and `OutputType` for `HashMap` and `BTreeMap`.

## [3.0.12] 2021-12-05

- Fix possible deadlock in dataloader. [#555](https://github.com/async-graphql/async-graphql/issues/555)
- Add some helper methods for `BatchRequest`.
  - BatchRequest::iter
  - BatchRequest::iter_mut
  - BatchRequest::variables
  - BatchRequest::data
  - BatchRequest::disable_introspection
- Fix implicit interfaces not being exposed via the __schema introspection. [#741](https://github.com/async-graphql/async-graphql/pull/741)

## [3.0.11] 2021-12-02

- Fix panic on f32-64::INFINITE/f32-64::NEG_INFINITE/f32-64::NAN output. [#735](https://github.com/async-graphql/async-graphql/issues/735)

## [3.0.10] 2021-11-30

- Fix the custom validator cannot work on `Option<Vec<T>>`.

## [3.0.9] 2021-11-30

- Fix the validator cannot work on `Option<Vec<T>>`.

## [3.0.8] 2021-11-30

- `#[graphql(validator(list))]` no longer applies to `max_items` and `min_items`.
- Implement `InputValue`/`OutputValue` for `serde_json::Value`.
- Add support for `SmolStr` via a feature. [#730](https://github.com/async-graphql/async-graphql/pull/730)

## [3.0.7] 2021-11-23

- Fix error extensions cause stack overflow. [#719](https://github.com/async-graphql/async-graphql/issues/719)

## [3.0.6] 2021-11-19

- Custom directives. [Book](https://async-graphql.github.io/async-graphql/en/custom_directive.html)

## [3.0.5] 2021-11-19

- Remove skipped fields from the document before executing the query.
- Add `isRepeatable` field to `__Directive` - [GraphQL - October 2021]

## [3.0.4] 2021-11-18

- Remove `OutputJson` because `Json` can replace it.
- Allowed use validators on wrapper types, for example: `Option<T>`, `MaybeUnefined<T>`.

## [3.0.3] 2021-11-18

- [integrations] Make `GraphQLWebSocket::new` use generic stream.
- [integrations] Add `GraphQLWebSocket::new_with_pair` method.

## [3.0.2] 2021-11-16

- Add `url`, `regex` and `ip` validators.

## [3.0.1] 2021-11-17

- Remove the `ctx` parameter of `CustomValidator::check`. [#710](https://github.com/async-graphql/async-graphql/issues/710)

## [3.0.0-alpha.2] 2021-11-16

- Change the signature of the `connection::query` function to allow the callback to use any type that implements `Into<Error>`.
- Remove `ResolverError` and use `Error::new_with_source` instead.
- Add `ErrorExtensionValues::unset` method.
- Use the `SimpleObject` macro and the `InputObject` macro at the same time.
- Types that are not referenced will be hidden in introspection.
- Make the API of integrations is more consistent.
- Remove `async-graphql-tide`.
- Rework validators. [Book](https://async-graphql.github.io/async-graphql/en/input_value_validators.html)
- Rework guards. [Book](https://async-graphql.github.io/async-graphql/en/field_guard.html)

## [2.11.3] 2021-11-13

- Implemented CursorType for i32/i64. [#701](https://github.com/async-graphql/async-graphql/pull/701)
- An error is returned when the number fails to parse. [#704](https://github.com/async-graphql/async-graphql/issues/704)
- Fix Federation entity union is empty during schema introspection. [#700](https://github.com/async-graphql/async-graphql/issues/700)

## [2.11.2] 2021-11-11

- Fix the problem that `EmptyMutation` may cause when used in `MergedObject`. [#694](https://github.com/async-graphql/async-graphql/issues/694)
- If a GraphQL name conflict is detected when creating schema, it will cause panic. [#499](https://github.com/async-graphql/async-graphql/issues/499)

## [2.11.1] 2021-11-07

- Add `chrono::Duration` custom scalar. [#689](https://github.com/async-graphql/async-graphql/pull/689)
- Implement `From<Option<Option<T>>>` for `MaybeUndefined<T>`.
- Add `MaybeUndefined::as_opt_ref`, `MaybeUndefined::as_opt_deref`, `MaybeUndefined::map`, `MaybeUndefined::map_value`, `MaybeUndefined::contains`, `MaybeUndefined::contains_value`, and `MaybeUndefined::transpose` methods.
- Made `MaybeUndefined::is_undefined`, `MaybeUndefined::is_null`, `MaybeUndefined::is_value`, `MaybeUndefined::value` and `MaybeUndefined::as_opt_ref` const.
- Add `ResolverError` type. [#671](https://github.com/async-graphql/async-graphql/issues/671)
- [async-graphql-axum] Bump axum from `0.2.5` to `0.3`.
- [async-graphql-poem] Export the HTTP headers in the `Context`.

## [2.11.0] 2021-11-03

- Use Rust `2021` edition.
- Subscription typename - [GraphQL - October 2021] [#681](https://github.com/async-graphql/async-graphql/issues/681)
- Allow directive on variable definition - [GraphQL - October 2021] [#678](https://github.com/async-graphql/async-graphql/issues/678)
- Specified By - [GraphQL - October 2021] [#677](https://github.com/async-graphql/async-graphql/issues/677)
- Add `specified_by_url` for `Tz`, `DateTime<Tz>`, `Url`, `Uuid` and `Upload` scalars.
- Number value literal lookahead restrictions - [GraphQL - October 2021] [#685](https://github.com/async-graphql/async-graphql/issues/685)

## [2.10.8] 2021-10-26

- [async-graphql-poem] Bump poem to `1.0.13`. 

## [2.10.6] 2021-10-26

- Add derived for object & simple object & complex object. [#667](https://github.com/async-graphql/async-graphql/pull/667) [#670](https://github.com/async-graphql/async-graphql/pull/670)
- Respect query object field order. [#612](https://github.com/async-graphql/async-graphql/issues/612)

## [2.10.5] 2021-10-22

- Bump poem from `0.6.6` to `1.0.7`. 

## [2.10.4] 2021-10-22

- Implement `Default` for ID #659.
- Implement `ScalarType` for `bson::Bson` and `bson::Document`. [#661](https://github.com/async-graphql/async-graphql/pull/661)
- Add `CharsMinLength` and `CharsMaxLength` validators. [#656](https://github.com/async-graphql/async-graphql/pull/656)
- Fix the `Subscription` macro to work on Rust 2021. [#665](https://github.com/async-graphql/async-graphql/pull/665)

## [2.10.3] 2021-10-12

- Add `visible` macro argument for union type. [#655](https://github.com/async-graphql/async-graphql/pull/655)

## [2.10.2] 2021-09-29

- Add concrete names support for `Object` macro. [#633](https://github.com/async-graphql/async-graphql/issues/633)
- Add `Lookahead::selection_fields` method. [#643](https://github.com/async-graphql/async-graphql/pull/643)

## [2.10.1] 2021-09-24

- Add `DataLoader::enable_all_cache` and `DataLoader::enable_cache` methods. [#642](https://github.com/async-graphql/async-graphql/issues/642)
- Change the execution order of `chain` and `race` guards. [#614](https://github.com/async-graphql/async-graphql/issues/614)
- Change log level from `error` to `info`. [#518](https://github.com/async-graphql/async-graphql/issues/518)

## [2.10.0] 2021-09-17

- Add support for `graphql-ws` pings. [#635](https://github.com/async-graphql/async-graphql/issues/635)
- Add feature gate `websocket` for async-graphql-tide. [#636](https://github.com/async-graphql/async-graphql/issues/636)
- Implement GraphQL enum to `Value` conversion. [#617](https://github.com/async-graphql/async-graphql/issues/617)
- Implement `ScalarType` for `HashMap`/`BTreeMap` to use `ToString`/`FromStr`. [#585](https://github.com/async-graphql/async-graphql/issues/585)

## [2.9.15] 2021-09-10

- Added Axum error handling. [#629](https://github.com/async-graphql/async-graphql/pull/629)
- Bump bson from `2.0.0-beta.1` to `2.0.0`. [#628](https://github.com/async-graphql/async-graphql/pull/628)

## [2.9.14] 2021-09-03

- Add support for [serde_cbor](https://crates.io/crates/serde_cbor). [#619](https://github.com/async-graphql/async-graphql/pull/619)

## [2.9.13] 2021-09-01

- Released [`Axum`](https://github.com/tokio-rs/axum) integration. [`async-graphql-axum`](https://crates.io/crates/async-graphql-axum)

## [2.9.12] 2021-08-24

- Add integration for [`Poem`](https://github.com/poem-web/poem).
- Ignore items flagged `@skip` in `SelectionField` and `Lookahead`. [#605](https://github.com/async-graphql/async-graphql/pull/605)

## [2.9.11] 2021-08-22

- Implement `From<MaybeUndefined<T>> for Option<Option<T>>`. [#599](https://github.com/async-graphql/async-graphql/issues/599)
- Add human readable for serializer. [#604](https://github.com/async-graphql/async-graphql/pull/604)

## [2.9.10] 2021-08-05

- Change `GraphQLPlaygroundConfig::with_setting` to accept `impl Into<Value>` [#583](https://github.com/async-graphql/async-graphql/issues/583)
- Remove unnecessary unwrap in multipart handler [#594](https://github.com/async-graphql/async-graphql/pull/594)

## [2.9.9] 2021-07-20

- Add binary types to `ConstValue` and `Value`. [#569](https://github.com/async-graphql/async-graphql/issues/569)
- Implemented `OutputType` for [Bytes](https://docs.rs/bytes/1.0.1/bytes/struct.Bytes.html).
- Changed Lookahead to support multiple fields. [#574](https://github.com/async-graphql/async-graphql/issues/574)
- Implement `TryFrom<&[SelectionField<'a>]>` for `Lookahead<'a>`. [#575](https://github.com/async-graphql/async-graphql/issues/575)
- Attach custom HTTP headers to the response when an error occurs. [#572](https://github.com/async-graphql/async-graphql/issues/572)
- Allow field visible to support paths. [#578](https://github.com/async-graphql/async-graphql/pull/578)
- Add `list` attribute to the input value validator. [#579](https://github.com/async-graphql/async-graphql/issues/579)

## [2.9.8] 2021-07-12

- Add Extensions in Error of `InputValueValidator`. [#564](https://github.com/async-graphql/async-graphql/pull/564)

- Fix SDL print is not stable. [#547](https://github.com/async-graphql/async-graphql/issues/547)

## [2.9.7] 2021-07-04

- Add support for generic `ComplexObject`. [#562](https://github.com/async-graphql/async-graphql/pull/562)

## [2.9.6] 2021-07-02

- Implement `From<SelectionField>` for `Lookahead`. [#557](https://github.com/async-graphql/async-graphql/issues/557)
  
- Add Decimal scalar (from `rust_decimal` crate) [#559](https://github.com/async-graphql/async-graphql/pull/559)

## [2.9.5] 2021-06-29

- Allows to get the actual field name and alias in `ResolveInfo`. [#551](https://github.com/async-graphql/async-graphql/issues/551)

## [2.9.4] 2021-06-21

- Fix the bug that `MergedObject` may cause panic. [#539](https://github.com/async-graphql/async-graphql/issues/539#issuecomment-862209442)

## [2.9.3] 2021-06-17

- Bump upstream crate `bson` from `v1.2.0` to `v2.0.0-beta.1`. [#516](https://github.com/async-graphql/async-graphql/pull/516)

- Add `serial` attribute for `Object`, `SimpleObject` and `MergedObject` macros. [#539](https://github.com/async-graphql/async-graphql/issues/539)

- Remove the `static` constraint of the `receive_body` and `receive_batch_body` functions. [#544](https://github.com/async-graphql/async-graphql/issues/544)

- Implement `InputType` and `OutputType` for `[T; N]` array.

## [2.9.2] 2021-06-10

- Allow field guards to support paths. [#536](https://github.com/async-graphql/async-graphql/issues/536)
  
- Add the `operation_name` to `Extension::execute` method. [#538](https://github.com/async-graphql/async-graphql/issues/538)

## [2.9.1] 2021-06-08

- Rework error propagation. [#531](https://github.com/async-graphql/async-graphql/issues/531)

## [2.9.0] 2021-06-07

- Add support for returning multiple resolver errors. [#531](https://github.com/async-graphql/async-graphql/issues/531)

- Bump upstream crate `multer` from `v1.2.2` to `v2.0.0`.

- Aligned NaiveDateTime formatting with DateTime. [#535](https://github.com/async-graphql/async-graphql/pull/535)

## [2.8.6] 2021-06-01

- Allow the ability to set GraphQL Playground settings. [#508](https://github.com/async-graphql/async-graphql/pull/508)

- WebSocket is now generic in graphql_subscription_upgrade functions. [#530](https://github.com/async-graphql/async-graphql/pull/530)

- Removed `Copy` trait from initializer in `graphql_subscription_with_data`. [#530](https://github.com/async-graphql/async-graphql/pull/530)

## [2.8.5] 2021-05-11

- If `InputObject` contains an unnamed field, the correct error message will be given. [#498](https://github.com/async-graphql/async-graphql/issues/498)

- Added `Websocket::with_message_stream` for client message customization. [#501](https://github.com/async-graphql/async-graphql/pull/501)

- Added the `Secret` type using [secrecy](https://crates.io/crates/secrecy) crate.

## [2.8.4] 2021-04-23

- Fix the problem that the `ComplexObject` macro cannot work due to the `secret` attribute.

## [2.8.3] 2021-04-12

- Fixed an error in exporting Federation SDL.

## [2.8.2] 2021-04-09

- Now when the resolver returns the `Result` type, `E` can be all types that implement `async_graphql::Into<Error>`.

## [2.8.1] 2021-04-08

### Fixed

- Fix stack overflow during Registry::create_type for recursive type while running Schema::build. [#474](https://github.com/async-graphql/async-graphql/issues/474)

### Added

- Add `secret` attribute for arguments, they will not appear in the log.

```rust
#[Object]
impl Query {
    async fn login(&self, username:String, #[graphql(secret)] password: String) -> i32 {
        todo!()
    }
}
```

## [2.8.0] 2021-04-05

### Changed

- Rework `Extension`, now fully supports asynchronous, better to use than before, and can achieve more features, it contains a lot of changes. _(if you don't have a custom extension, it will not cause the existing code to fail to compile)_
  
### Added

- Add `async_graphql_warp::graphql_protocol`, `async_graphql_warp::graphql_subscription_upgrade` and `async_graphql_warp::graphql_subscription_upgrade_with_data` to control WebSocket subscription more finely. 

## [2.7.4] 2021-04-02

- Add the `BuildHasher` generic parameter to `dataloader::HashMapCache` to allow custom hashing algorithms. [#455](https://github.com/async-graphql/async-graphql/issues/455)

## [2.7.3] 2021-04-02
 
## Added 

- Add cache support for DataLoader. [#455](https://github.com/async-graphql/async-graphql/issues/455)
- Implements `ScalarType` for `serde_json::Value`.
- Add `SelectionField::alias` and `SelectionField::arguments` methods.

## Fixed  

- Prevent Warp WS Close, Ping, and Pong messages from being parsed as GraphQL [#459](https://github.com/async-graphql/async-graphql/pull/459)
- Fix Schema::sdl() does not include subscription definitions. [#464](https://github.com/async-graphql/async-graphql/issues/464)

## [2.7.2] 2021-04-01

## Removed

- Remove `SchemaBuilder::override_name` method. [#437](https://github.com/async-graphql/async-graphql/issues/437)
  
## Added

- Add `name` and `visible` attributes for `Newtype` macro for define a new scalar. [#437](https://github.com/async-graphql/async-graphql/issues/437)
- `NewType` macro now also implements `From<InnerType>` and `Into<InnerType>`.

## [2.7.1] 2021-03-31

- Add `Request::disable_introspection` method. [#456](https://github.com/async-graphql/async-graphql/issues/456)

## [2.7.0] 2021-03-27

## Fixed

- Fix chrono-tz integration. [#452](https://github.com/async-graphql/async-graphql/pull/452)

## Changed

- Rework Extension & TracingExtension & OpenTelemetryExtension

## [2.6.5] - 2021-03-24

- In websocket, if the client sends `start` before `connection_init`, the connection will be immediately disconnected and return `1011` error. [#451](https://github.com/async-graphql/async-graphql/issues/451)

## [2.6.4] - 2021-03-22

- Fix docs.

## [2.6.3] - 2021-03-22

### Added

- Add `extension::OpenTelemetry`.

### Removed

- Remove `TracingConfig`, now Request span always takes the current span as the parent, so this option is no longer needed.
- Remove `multipart` feature.

### Changed

- Now all features are not activated by default.

## [2.6.2] - 2021-03-20

- Add `SchemaBuilder::enable_subscription_in_federation` method.  [#449](https://github.com/async-graphql/async-graphql/issues/449)

## [2.6.1] - 2021-03-19

- Fix tracing extension doesn't work with async code. [#448](https://github.com/async-graphql/async-graphql/issues/448)

## [2.6.0] - 2021-03-18

- Add [ComplexObject](https://docs.rs/async-graphql/2.6.0/async_graphql/attr.ComplexObject.html) macro.

## [2.5.14] - 2021-03-14

- Add `DataLoader::loader` method. [#441](https://github.com/async-graphql/async-graphql/issues/441)
- Fix the validation does not work on some inline fragments.

## [2.5.13] - 2021-03-09

- Support generics in Subscription types. [#438](https://github.com/async-graphql/async-graphql/pull/438)

## [2.5.12] - 2021-03-09

- Remove unnecessary Box from WebSocket messages.
- Export subscription type to Federation SDL. (for [GraphGate](https://github.com/async-graphql/graphgate) 😁)
- Add `extends` attribute for derive macros Subscription and MergedSubscription.
- Add `SchemaBuilder::override_name` method. [#437](https://github.com/async-graphql/async-graphql/issues/437)

## [2.5.11] - 2021-03-07

- Execute `_entity` requests in parallel. [#431](https://github.com/async-graphql/async-graphql/issues/431)

## [2.5.10] - 2021-03-06

- Add descriptions for the exported Federation SDL.

## [2.5.9] - 2021-02-28

### Changed

- Moved `Variables` from `async_graphql::context::Variables` to `async_graphql::Variables`.

## [2.5.8] - 2021-02-27

### Added

- Allow the `deprecation` attribute to have no reason.

    ```rust
    #[derive(SimpleObject)]
    struct MyObject {
        #[graphql(deprecation)]
        a: i32,
    
        #[graphql(deprecation = true)]
        b: i32,
    
        #[graphql(deprecation = false)]
        c: i32,
    
        #[graphql(deprecation = "reason")]
        d: i32,
    }
    ```

## [2.5.7] - 2021-02-23

### Fixed

- Fix the problem that the borrowing lifetime returned by the `Context::data` function is too small.

## [2.5.6] - 2021-02-23

### Changed

- When introspection is disabled, introspection related types are no longer registered.

## [2.5.5] - 2021-02-22

### Added

- Add support for Federation [nested keys](https://www.apollographql.com/docs/federation/entities/#defining-a-compound-primary-key).

## [2.5.4] - 2021-02-15

### Fixed

- Fixed the error that the directive locations `FIELD_DEFINITION` and `ENUM_VALUE` cannot be parsed.

## [2.5.3] - 2021-02-13

### Fixed

- Fixed [#409](https://github.com/async-graphql/async-graphql/issues/409)

## [2.5.2] - 2021-02-06

### Added

- Add subscription support for tide with [tide-websockets](https://crates.io/crates/tide-websockets).

### Fixed

- Fixed the bug that can accept subscription requests during the initialization of WebSocket.
- Fixed GraphQL over WebSocket Protocol does not support ConnectionError events. [#406](https://github.com/async-graphql/async-graphql/issues/406)
