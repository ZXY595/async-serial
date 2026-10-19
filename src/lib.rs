use std::{
    io,
    ops::{Deref, DerefMut},
};

#[cfg(any(target_family = "unix", target_os = "wasi"))]
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, RawFd};

pub use async_io::Async;
use async_io::IoSafe;
pub use serialport::{SerialPortBuilder, new};
pub mod error {
    pub use serialport::{Error, Result};
}

pub type AsyncSerialPort = Async<
    IoSafeAdapter<
        cfg_select! {
            unix => serialport::TTYPort,
            windows => serialport::COMPort,
        },
    >,
>;
pub trait AsyncSerialPortBuilder {
    fn open_async(self) -> serialport::Result<AsyncSerialPort>;
}

impl AsyncSerialPortBuilder for serialport::SerialPortBuilder {
    fn open_async(self) -> serialport::Result<AsyncSerialPort> {
        self.open_native()
            .map(IoSafeAdapter)
            .and_then(|io| Async::new(io).map_err(From::from))
    }
}

/// An [`IoSafe`] newtype for [`serialport::SerialPort`] implementors, useful for [`Async`] wrapper.
#[derive(Debug)]
pub struct IoSafeAdapter<T>(T);

impl<T> Deref for IoSafeAdapter<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for IoSafeAdapter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: io::Read> io::Read for IoSafeAdapter<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<T: io::Write> io::Write for IoSafeAdapter<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

/// # Safety:
/// The implementation of [`io::Read`] and [`io::Write`] for the wrapped serial port is [`IoSafe`]
unsafe impl<T: io::Read + io::Write> IoSafe for IoSafeAdapter<T> {}

#[cfg(any(target_family = "unix", target_os = "wasi"))]
impl<T: AsRawFd> AsRawFd for IoSafeAdapter<T> {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

#[cfg(any(target_family = "unix", target_os = "wasi"))]
impl<T> AsFd for IoSafeAdapter<T>
where
    Self: AsRawFd,
{
    fn as_fd(&self) -> BorrowedFd<'_> {
        // Safety: This fd lifespan is tied to the `T`
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}
