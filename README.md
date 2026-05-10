# async-serial

Lightweight async serial port adapter — bridges [`serialport`] with [`async-io`].

[![Crates.io](https://img.shields.io/crates/v/async-serial)](https://crates.io/crates/async-serial)
[![License](https://img.shields.io/crates/l/async-serial)](https://github.com/ZXY595/async-serial#license)

## Quickstart

```rust
use async_serial::AsyncSerialPortBuilder;

let mut port = serialport::new("/dev/ttyUSB0", 9_600)
    .open_async()?;

// Async read — won't block the executor
let mut buf = [0u8; 256];
let n = port.read(&mut buf).await?;
```

## How It Works

`serialport::TTYPort` / `serialport::COMPort` implement `Read` + `Write` but those are blocking calls. `async-io` provides an `Async<T>` wrapper that makes any `IoSafe + AsFd` type non-blocking via the reactor.

This crate provides:

- **`IoSafeAdapter<T>`** — a newtype that supplies the missing `IoSafe` and `AsFd` impls
- **`AsyncSerialPort`** — a type alias for `Async<IoSafeAdapter<platform port>>`
- **`AsyncSerialPortBuilder`** — an extension trait that adds `.open_async()` to `SerialPortBuilder`

## API

- **`AsyncSerialPortBuilder::open_async()`** — opens a port via builder and wraps it for async IO
- **`AsyncSerialPort`** — type alias: `Async<IoSafeAdapter<TTYPort>>` (unix) or `Async<IoSafeAdapter<COMPort>>` (windows)
- **`IoSafeAdapter<T>`** — `Deref<Target = T>`, all serial methods accessible transparently
- **Re-exports**: `Async`, `SerialPortBuilder`, `new`, `Error`, `Result`

## Platform Support

| Platform | Backend type   |
|----------|---------------|
| Unix     | `TTYPort`     |
| Windows  | `COMPort`     |

## License

MIT OR Apache-2.0
