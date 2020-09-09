//! ## This crate has been replaced by [Ethernet support within
//! stm32h7xx-hal](https://docs.rs/stm32h7xx-hal/latest/stm32h7xx_hal/ethernet/index.html).
//!
//! ## License
//!
//! Licensed under either of
//!
//! - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//! - MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! [stm32h7xx-hal]: https://github.com/stm32-rs/stm32h7xx-hal
//! [smoltcp]: https://github.com/smoltcp-rs/smoltcp
#![no_std]

#[macro_use]
extern crate log;

mod ethernet;

#[cfg(feature = "phy_ksz8081r")]
mod ksz8081r;
#[cfg(feature = "phy_lan8742a")]
mod lan8742a;
#[cfg(not(any(feature = "phy_ksz8081r", feature = "phy_lan8742a")))]
compile_error!(
    "A least one PHY device must be enabled. Use a feature gate to
enable."
);
#[cfg(all(feature = "phy_ksz8081r", feature = "phy_lan8742a"))]
compile_error!(
    "Cannot enable multiple PHY devices. Try setting
`default-features = false`."
);

/// PHY address
pub const ETH_PHY_ADDR: u8 = 0;

/// Station Management Interface (SMI) on an ethernet PHY
pub trait StationManagement {
    /// Read a register over SMI.
    fn smi_read(&mut self, reg: u8) -> u16;
    /// Write a register over SMI.
    fn smi_write(&mut self, reg: u8, val: u16);
}

/// Traits for an Ethernet PHY
trait PHY {
    /// Reset PHY and wait for it to come out of reset.
    fn phy_reset(&mut self);
    /// PHY initialisation.
    fn phy_init(&mut self);
}

pub use ethernet::{enable_interrupt, ethernet_init, interrupt_handler};
pub use ethernet::{DesRing, EthernetDMA, EthernetMAC};
