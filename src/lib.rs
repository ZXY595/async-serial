use std::{
    ffi::OsStr,
    fmt, io,
    ops::{Deref, DerefMut},
    os::fd::{AsFd, AsRawFd, BorrowedFd, RawFd},
};

pub use async_io::Async;
use async_io::IoSafe;
pub use serial::{SerialPort, SerialPortSettings};
pub mod error {
    pub use serial::{Error, ErrorKind, Result};
}

pub fn open(port: &(impl AsRef<OsStr> + ?Sized)) -> serial::Result<Async<SystemPort>> {
    let port = serial::open(port)?;
    SystemPort::from_serial(port).map_err(From::from)
}

/// An [`IoSafe`] newtype for [`serial::SystemPort`], useful for [`Async`] wrapper.
pub struct SystemPort(serial::SystemPort);

impl SystemPort {
    pub fn from_serial(port: serial::SystemPort) -> io::Result<Async<Self>> {
        Async::new(Self(port))
    }
}

impl Deref for SystemPort {
    type Target = serial::SystemPort;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SystemPort {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl io::Read for SystemPort {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl io::Write for SystemPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

/// # Safety:
/// The implementation of [`io::Read`] and [`io::Write`] for [`serial::SystemPort`] is [`IoSafe`]
unsafe impl IoSafe for SystemPort {}

impl AsRawFd for SystemPort {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl AsFd for SystemPort {
    fn as_fd(&self) -> BorrowedFd<'_> {
        // Safety: This fd lifespan is tied to the `serial::SystemPort`
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}

impl fmt::Debug for SystemPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SystemPort")
            .field(&self.as_raw_fd())
            .finish()
    }
}
