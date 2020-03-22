# [Documentation](https://docs.rs/stm32h7-ethernet)

# stm32h7-ethernet

[![docs.rs](https://docs.rs/stm32h7-ethernet/badge.svg)](https://docs.rs/stm32h7-ethernet)
[![Travis](https://travis-ci.com/richardeoin/stm32h7-ethernet.svg?branch=master)](https://travis-ci.com/richardeoin/stm32h7-ethernet)
[![Crates.io](https://img.shields.io/crates/v/stm32h7-ethernet.svg)](https://crates.io/crates/stm32h7-ethernet)

This crate implements a [smoltcp][] device interface `phy::Device` for
the STM32H7 series of microcontrollers.

Multiple PHYs are supported:
- SMSC LAN8742a
- Micrel KSZ8081R

To build this crate, a device feature of [stm32h7xx-hal][] must be
selected. For example:

```
cargo build  --features stm32h7xx-hal/rt,stm32h7xx-hal/stm32h743
```

Applications using this crate should instead enable the correct
features of [stm32h7xx-hal][] themselves.

## Hardware Examples

### STM32H747I-DISCO

Targeting the STM32H747I-DISCO evaluation board from ST.

*Note:* Close solder jumper SB8!

### License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[stm32h7xx-hal]: https://github.com/stm32-rs/stm32h7xx-hal
[smoltcp]: https://github.com/smoltcp-rs/smoltcp
