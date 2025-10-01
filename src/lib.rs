//! This is a platform agnostic Rust driver for the ultra-compact, low-cost,
//! host-side fuel-gauge systems for lithium-ion (Li+) batteries in handheld
//! and portable equipment using the [`embedded-hal`] traits.
//!
//! It is compatible with MAX17043, MAX17044, MAX17048, MAX17049, MAX17058 and MAX17059.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Get state of charge. See: [`soc()`].
//! - Get battery voltage. See: [`voltage()`].
//! - Software reset. See: [`reset()`].
//! - Quickstart. See: [`quickstart()`].
//! - Get IC version. See: [`version()`].
//! - Only on MAX17048/MAX17049:
//!     - Get charge/discharge rate. See: [`charge_rate()`].
//! - Only on MAX17048/MAX17049/MAX17058/MAX17059:
//!     - Set table registers. See: [`set_table()`].
//!
//! [`soc()`]: struct.Max17043.html#method.soc
//! [`voltage()`]: struct.Max17043.html#method.voltage
//! [`reset()`]: struct.Max17043.html#method.reset
//! [`quickstart()`]: struct.Max17043.html#method.quickstart
//! [`version()`]: struct.Max17043.html#method.version
//! [`charge_rate()`]: struct.Max17048.html#method.charge_rate
//! [`set_table()`]: struct.Max17048.html#method.set_table
//!
//! <!-- TODO
//! [Introductory blog post]()
//! -->
//!
//! ## The devices
//! The devices are ultra-compact, low-cost, host-side fuel-gauge systems
//! for lithium-ion (Li+) batteries in handheld and portable equipment.
//! There are models configured to operate with a single or dual lithium
//! cell pack.
//!
//! The devices use a sophisticated Li+ battery-modeling scheme, called
//! ModelGauge(TM) to track the battery's relative state-of-charge (SOC)
//! continuously over a widely varying charge/discharge profile. Unlike
//! traditional fuel gauges, the ModelGauge algorithm eliminates the need
//! for battery relearn cycles and an external current-sense resistor.
//! Temperature compensation is possible in the application with minimal
//! interaction between a μC and the device.
//!
//! The communication is done through an I2C interface.
//!
//! Datasheets: [MAX17043/MAX17044](https://datasheets.maximintegrated.com/en/ds/MAX17043-MAX17044.pdf),
//! [MAX17048/MAX17049](https://datasheets.maximintegrated.com/en/ds/MAX17048-MAX17049.pdf),
//! [MAX17058/MAX17059](https://datasheets.maximintegrated.com/en/ds/MAX17058-MAX17059.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Read state of charge and cell voltage (sync)
//!
//! ```no_run
//! # #[cfg(not(feature = "async"))] {
//! # use embedded_hal_mock::eh1::i2c::Mock;
//! use max170xx::Max17043;
//!
//! # let mut sensor = Max17043::new(Mock::new(&[]));
//! let soc = sensor.soc().unwrap();
//! let voltage = sensor.voltage().unwrap();
//! println!("Charge: {:.2}%", soc);
//! println!("Voltage: {:.2}V", voltage);
//! # }
//! ```
//!
//! ### Read state of charge and cell voltage (async)
//!
//! ```no_run
//! # #[cfg(feature = "async")] {
//! # use embedded_hal_mock::eh1::i2c::Mock;
//! use max170xx::Max17043;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let mut sensor = Max17043::new(Mock::new(&[]));
//! let soc = sensor.soc().await.unwrap();
//! let voltage = sensor.voltage().await.unwrap();
//! println!("Charge: {:.2}%", soc);
//! println!("Voltage: {:.2}V", voltage);
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! ### Trigger software reset
//!
//! ```no_run
//! # #[cfg(not(feature = "async"))] {
//! # use embedded_hal_mock::eh1::i2c::Mock;
//! use max170xx::Max17043;
//!
//! # let mut sensor = Max17043::new(Mock::new(&[]));
//! sensor.reset().unwrap();
//! # }
//! # #[cfg(feature = "async")] {
//! # use embedded_hal_mock::eh1::i2c::Mock;
//! # use max170xx::Max17043;
//! # async fn example() {
//! # let mut sensor = Max17043::new(Mock::new(&[]));
//! sensor.reset().await.unwrap();
//! # }
//! # }
//! ```
//!
//! ### Quick start
//!
//! ```no_run
//! # #[cfg(not(feature = "async"))] {
//! # use embedded_hal_mock::eh1::i2c::Mock;
//! use max170xx::Max17043;
//!
//! # let mut sensor = Max17043::new(Mock::new(&[]));
//! // ... noisy power-up ...
//! sensor.quickstart().unwrap();
//! # }
//! # #[cfg(feature = "async")] {
//! # use embedded_hal_mock::eh1::i2c::Mock;
//! # use max170xx::Max17043;
//! # async fn example() {
//! # let mut sensor = Max17043::new(Mock::new(&[]));
//! // ... noisy power-up ...
//! sensor.quickstart().await.unwrap();
//! # }
//! # }
//! ```
//!
//! ### Read charge/discharge rate
//!
//! ```no_run
//! # #[cfg(not(feature = "async"))] {
//! # use embedded_hal_mock::eh1::i2c::Mock;
//! use max170xx::Max17048;
//!
//! # let mut sensor = Max17048::new(Mock::new(&[]));
//! let rate = sensor.charge_rate().unwrap();
//! println!("Charge rate: {:.2}%/h", rate);
//! # }
//! # #[cfg(feature = "async")] {
//! # use embedded_hal_mock::eh1::i2c::Mock;
//! # use max170xx::Max17048;
//! # async fn example() {
//! # let mut sensor = Max17048::new(Mock::new(&[]));
//! let rate = sensor.charge_rate().await.unwrap();
//! println!("Charge rate: {:.2}%/h", rate);
//! # }
//! # }
//! ```
//!

#![deny(unsafe_code, missing_docs)]
#![no_std]

mod types;
pub use crate::types::Error;
#[macro_use]
mod common;
#[macro_use]
mod register_access;
use crate::register_access::{Command, Register, ADDR};
mod max17043_44;
pub use crate::max17043_44::{Max17043, Max17044};
mod max170x8_x9;
pub use crate::max170x8_x9::{Max17048, Max17049, Max17058, Max17059};
