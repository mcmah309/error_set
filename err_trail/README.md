# err_trail

Convience methods on `Result` and `Option` for logging when an `Err` or `None` is ecountered. Similar to [anyhow](https://github.com/dtolnay/anyhow)
but for logging.

## Feature Flags

**tracing** / **log** / **defmt** :
Enables support for the `tracing` or `log` or `defmt` crates. Methods are added to `Result` and are executed when the `Result` is an `Err` for logging purposes. They work similarly to `anyhow`'s `.context(..)` method. e.g.
```rust
let result: Result<(), &str> = Err("operation failed");

let value: Result<(), &str> = result.error_context("If `Err`, this message is logged as error via tracing/log/defmt");
let value: Result<(), &str> = result.warn_context("If `Err`, this message is logged as warn via tracing/log/defmt");
let value: Result<(), &str> = result.with_error_context(|err| format!("If `Err`, this message is logged as error via tracing/log/defmt: {}", err));
let value: Option<()> = result.consume_as_error(); // If `Err`, the `Err` is logged as error via tracing/log/defmt
let value: Option<()> = result.consume_with_warn(|err| format!("If `Err`, this message is logged as warn via tracing/log/defmt: {}", err));
// ...etc.
```
This is useful tracing context around errors. e.g.
```rust
let val = func().warn_context("`func` failed, here is some extra context like variable values")?;
let val = func().consume_as_warn();
```
rather than
```rust
let val = func().inspect_err(|err| tracing::warn!("`func` failed, here is some extra context like variable values"))?;
let val = func().inspect_err(|err| tracing::warn!("{}", err)).ok();
```
> Note: a `stub` feature flag also exists to be used by libraries. This allows the api's to be used in libraries
> while a downstream binary can ultimately decide the implementation. If no implementations is selected, since all the above
> methods are inlined, the code will be optimized away during compilation.

## Guide

**warn** - Represents a bad state in which the current process _can_ continue.

**error** - Represents a bad state in which the current process _cannot_ continue.

- If returning a `Result` to the calling function, context should usually be `warn`
- If consuming a `Result`, context should usually be `error`