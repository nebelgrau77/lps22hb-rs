# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

...


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
