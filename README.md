# async-serial

Lightweight async serial port adapter — bridges [`serial`] with [`async-io`].

[![Crates.io](https://img.shields.io/crates/v/async-serial)](https://crates.io/crates/async-serial)
[![License](https://img.shields.io/crates/l/async-serial)](https://github.com/ZXY595/async-serial#license)

## Quickstart

```rust
use async_serial::{open, SerialPort, SerialPortSettings}

let mut port = open("/dev/ttyUSB0")?;
port.reconfigure(&|settings| {
    settings.set_baud_rate(serial::Baud9600)?;
    Ok(())
})?;

// Async read — won't block the executor
let mut buf = [0u8; 256];
let n = port.read(&mut buf).await?;
```

## How It Works

`serial::SystemPort` implements `Read` + `Write` but those are blocking calls. `async-io` provides an `Async<T>` wrapper that makes any `IoSafe + AsFd` type non-blocking via the reactor. This crate provides the missing `IoSafe` and `AsFd` impls in a small newtype.

## API

- **`open(path)`** — opens a serial port and returns `Async<SystemPort>`
- **`SystemPort::from_serial(port)`** — wraps an existing `serial::SystemPort`
- **`Deref<Target = serial::SystemPort>`** — transparent access to all serial methods

## License

MIT OR Apache-2.0
