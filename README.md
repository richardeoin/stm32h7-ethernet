# stm32h7-ethernet

`stm32h7-ethernet` implements ethernet for the STM32H7 series of
microcontrollers.

Multiple PHYs are supported
    * SMSC LAN8742a
    * Micrel KSZ8081R

# Examples

## `stm32h747i-disco`

Targeting the STM32H747I-DISCO evaluation board from ST.

*Note:* Close solder jumper SB8 on the underside of the PCB!

`$ cargo run --example stm32h747i-disco`

This will start `arm-none-eabi-gdb`.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
