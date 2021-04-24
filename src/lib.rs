#![cfg_attr(not(test), no_std)]

mod bus;
mod common;
pub mod dev;
mod pin;

pub use bus::I2cBus;
pub use common::mode;
pub use pin::Pin;

pub(crate) use bus::I2cExt;
pub(crate) use common::Direction;
pub(crate) use common::PortDriver;
pub(crate) use common::PortDriverTotemPole;

pub use dev::pca9536::Pca9536;
pub use dev::pca9555::Pca9555;
pub use dev::pcf8574::Pcf8574;
pub use dev::pcf8574::Pcf8574a;
