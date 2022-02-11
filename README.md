# lis3dh-spi
![build_workflow](https://github.com/robhany/lis3dh-spi/actions/workflows/rust.yml/badge.svg)
[![Crates.io Version][crates-io-badge]][crates-io]
[![Crates.io Downloads][crates-io-download-badge]][crates-io-download]
![No Std][no-std-badge]


This crate is a no_std driver for the LIS3DH accelerometer using SPI.

## Datasheet

https://www.st.com/resource/en/datasheet/lis3dh.pdf


## About this driver
This driver is not consuming the SPI bus the communication is running on.
Bus has to be passed as mutable reference when interacting with the LIS3DH.  
## Usage
Add this to your Cargo.toml:

```toml
[dependencies]
lis3dh-spi = "0.0.3"
```

And this to your main.rs

```rust

let mut accelerometer = lis3dh_spi::Lis3dh::default();

accelerometer.set_l_p_en(LPEn::HighResolutionNormalMode);
accelerometer.set_output_data_rate(ODR::Hz400);
accelerometer.write_all_settings(&mut chip_select_pin, &mut spi_bus).ok();
let angle_and_gravity_offset = accelerometer.get_angle_and_gravity_offset(&mut chip_select_pin, &mut spi_bus).ok();

```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

<!-- Badges -->
[crates-io]: https://crates.io/crates/lis3dh-spi
[crates-io-badge]: https://img.shields.io/crates/v/lis3dh-spi.svg?maxAge=3600
[crates-io-download]: https://crates.io/crates/lis3dh-spi
[crates-io-download-badge]: https://img.shields.io/crates/d/lis3dh-spi.svg?maxAge=3600
[no-std-badge]: https://img.shields.io/badge/no__std-yes-blue

