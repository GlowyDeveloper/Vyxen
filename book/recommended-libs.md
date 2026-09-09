# Recommended Libraries

This list comprises of libraries that are recommended to use when working with Vyxen.

## [log](https://crates.io/crates/log)

A logging facade for Rust. Vyxen uses this to report warnings and errors.

To pipe the logs into the output, look at [`env_logger`](#env_logger) and [`console_log`](#console_log).

 - `log!()`
 - `debug!()`
 - `warn!()`
 - `error!()`

## [env_logger](https://crates.io/crates/env_logger)

Pipes the logs from `log` into the output. This does not work on wasm32, for this you should use `console_log`.

```rust
env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
// Initilizes with info level

env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
// Initilizes with debug level
```

## [console_log](https://crates.io/crates/console_log)

Pipes the logs from `log` into the output for wasm32.

```rust
console_log::init_with_level(log::Level::Info).unwrap();
// Initilizes with info level

console_log::init_with_level(log::Level::Debug).unwrap();
// Initilizes with debug level
```

## [console_error_panic_hook](https://crates.io/crates/console_error_panic_hook)

Panic hook that logs the panic message and the stack trace to the console for wasm32.

```rust
console_error_panic_hook::init();
```