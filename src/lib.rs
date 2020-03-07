//! Ethernet for stm32h7xx
//!
//! Implments smoltcp device interface `phy::Device` for STM32H7
//! series microcontrollers.
//!
//! To build this crate, a specific device feature of `stm32h7xx-hal`
//! mube be selected. For example:
//!
//! ```
//! cargo build  --features stm32h7xx-hal/rt,stm32h7xx-hal/stm32h743
//! ```
//!
//! Applications using this crate should instead enable the correct
//! features of stm32h7xx-hal themselves.
//!
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
