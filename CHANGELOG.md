# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2022-01-28

### Removed
- SPI mode temporarily disabled due to multibyte read problems

### Fixed
- bug in the `interrupt.rs` configuration settings

## [0.1.2] - 2022-01-24

### Changed
- `enable_fifo` now called `configure_fifo`
- `configure_interrupts` does not have the `flag` argument anymore
- enabling of differential pressure events moved to the interrupt configuration struct

### Fixed
- upper bits of `INTERRUPT_CFG` register not overwritten anymore

## [0.1.1] - 2022-01-23

### Added
- FLAG enum
- FIFO_ON enum
- threshold and offset reading and setting
- SPI mode

### Changed
- pressure and temperature read separately
- data status read in one go
- enable_interrupts becomes configure_interrupts, 
- enable_differential becomes one of the flags of the configuration
- IntSource becomes IntStatus, and is read in one go

## [0.1.0] - 2021-12-08

### Added
- FIFO watermark level setting
- Pressure and temperature data availability and overrun flags read into a new DataStatus struct
- Oneshot() function setting the power-down mode and enabling the ONE_SHOT bit in one go
- enable_fifo() function that enables and configures FIFO, using a struct for configuration
- get_fifo_status() (reads FIFO status into a struct)
- enable_interrupts() function that enables interupts and configures the interrupt pin, using a struct for configuration
- get_int_status function that reads the interrupt status into a struct
- lowpass_filter function replacing two separate ..enable and ..configure functions

### Removed
- Separate functions for pressure and temperature availability and overrun flag reading
- Separare fifo_enable, fifo_mode_config, stop_on_fth and set_watermark level functions 
- Separate fifo_threshold_status and fifo_overrun_status
- Separate int_generation_enable, int_latch_enable, data_signal_drdy_enable, interrupt_pin_active, interrupt_pin_config, int_drdy_config functions
- Separate interrupt_active, low_pressure_event_occurred and high_pressure_event_occurred functions
- Separate lowpass_filter_enable and lowpass_filter_configure functions

### Changed
- fifo_stored_data_level renamed read_fifo_level and turned private


## [0.0.3] - 2021-12-04

### Added
- Control functions: reboot, software reset
- Configuration functions: BDU, autozero, I2C disable, SPI mode
- Interrupt configuration
- Reading status of interrupts
- Reading data availability and overrun status
- FIFO control, configuration and reading 
- Interrupt pin configuration
- Low-pass filter configuration

[0.0.3]: https://github.com/nebelgrau77/lps22hb-rs/releases/tag/v0.0.3

## [0.0.2] - 2021-08-15

### Added
- data rate setting
- Adruino example with serial over USB
- `new` function
- `get_device_id` function

### Changed
- driver instance created with the `new` function, no settings
- organization in modules following the LPS25HB example: pressure and temperature reading functions moved to `sensor.rs`
  

[0.0.2]: https://github.com/nebelgrau77/lps22hb-rs/releases/tag/v0.0.2

## [0.0.1] - 2021-04-17

### Added
- Checking if sensor is reachable (WHOAMI)
- Enabling single-shot data acquisition
- Reading pressure and temperature

[0.0.1]: https://github.com/nebelgrau77/lps22hb-rs/releases/tag/v0.0.1
