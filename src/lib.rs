//! A platform agnostic driver to interface with LPS22HB pressure sensor module.
//!
//! This driver allows you to:
//! - read atmospheric pressure in hPa, see [`read_pressure()`]
//! - read temperature in degrees Celsius, see [`read_temperature()`]
//! - enable single-shot data acquisition, see [`enable_one_shot()`]
//! - set data rate, see [`set_datarate()`]
//!
//! [`read_pressure()`]: struct.LPS22HB.html#method.read_pressure
//! [`read_temperature()`]: struct.LPS22HB.html#method.read_temperature
//! [`enable_one_shot()`]: struct.LPS22HB.html#method.enable_one_shot
//! [`set_datarate()`]: struct.LPS22HB.html#method.set_datarate
//!
//! __NOTE__: This is a very early version of the crate. Only I2C interface is supported at the moment.
//!  
//!
//! ### Datasheet: [LPS22HB](https://www.st.com/resource/en/datasheet/lps22hb.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! Please find additional examples using hardware in this repository: [examples]
//!
//! [examples]: https://github.com/nebelgrau77/lps22hb-rs/examples
//!
//! ### Read pressure and temperature
//!
//! ```rust
//!
//! use lps22hb::interface::{I2cInterface, i2c::I2cAddress};
//! use lps22hb::*;
//!
//! let mut lps22 = LPS22HB.new(i2c_interface);
//!
//!
//! lps22.enable_one_shot().unwrap();
//!
//! let pressure = lps22.read_pressure().unwrap();
//! let temperature = lps22.read_temperature().unwrap();
//! ```
//!

// TO DO:
// - add other enums (FIFO settings, interrupts, etc.)

#![no_std]
//#![deny(warnings, missing_docs)]

pub mod sensor;
use sensor::*;

pub mod config;
use config::*;

pub mod fifo;
use fifo::*;

pub mod interrupt;
use interrupt::*;

pub mod register;
use register::{Bitmasks, Registers};

pub mod interface;
use interface::Interface;

/// Sensor's ID
const WHOAMI: u8 = 0b10110001; // decimal value 177

/// The output of the temperature sensor must be divided by 100, see p. 10 of the datasheet.
const TEMP_SCALE: f32 = 100.0;
/// The output of the pressure sensor must be divided by 4096, see p. 10 of the datasheet.
const PRESS_SCALE: f32 = 4096.0;

/// Holds the driver instance with the selected interface
pub struct LPS22HB<T> {
    interface: T,
}

impl<T, E> LPS22HB<T>
where
    T: Interface<Error = E>,
{
    /// Create a new instance of the LPS25HB driver.
    pub fn new(interface: T) -> Self {
        LPS22HB { interface }
    }

    /// Destroy driver instance, return interface instance.
    pub fn destroy(self) -> T {
        self.interface
    }

    /*
    /// Verifies communication with WHO_AM_I register
    pub fn sensor_is_reachable(&mut self) -> Result<bool, T::Error> {
        let mut bytes = [0u8; 1];
        let (who_am_i, register) = (WHOAMI, Registers::WHO_AM_I.addr());
        self.interface.read(register, &mut bytes)?;
        Ok(bytes[0] == who_am_i)
    }



    /// Initializes the sensor with selected settings
    pub fn begin_sensor(&mut self) -> Result <(), T::Error> {
        self.interface.write(
            Registers::CTRL_REG1.addr(),
            self.sensor.ctrl_reg1(),
        )?;
        self.interface.write(
            Registers::CTRL_REG2.addr(),
            self.sensor.ctrl_reg2(),
        )?;
        Ok(())
    }
    */

    /// Clear selected bits using a bitmask
    fn clear_register_bit_flag(&mut self, address: Registers, bitmask: u8) -> Result<(), T::Error> {
        let mut reg_data = [0u8; 1];
        self.interface.read(address.addr(), &mut reg_data)?;
        //let bitmask = bitmask.bitmask();
        let payload: u8 = reg_data[0] & !bitmask;
        self.interface.write(address.addr(), payload)?;
        Ok(())
    }

    /// Set selected bits using a bitmask
    fn set_register_bit_flag(&mut self, address: Registers, bitmask: u8) -> Result<(), T::Error> {
        let mut reg_data = [0u8; 1];
        self.interface.read(address.addr(), &mut reg_data)?;
        let payload: u8 = reg_data[0] | bitmask;
        self.interface.write(address.addr(), payload)?;
        Ok(())
    }

    /// Read a byte from the given register.
    fn read_register(&mut self, address: Registers) -> Result<u8, T::Error> {
        let mut reg_data = [0u8];
        self.interface.read(address.addr(), &mut reg_data)?;
        Ok(reg_data[0])
    }

    /// Check if specific bits are set.
    fn is_register_bit_flag_high(
        &mut self,
        address: Registers,
        bitmask: u8,
    ) -> Result<bool, T::Error> {
        let data = self.read_register(address)?;
        Ok((data & bitmask) != 0)
    }
}

/// Output data rate and power mode selection (ODR). (Refer to Table 17)
#[derive(Debug, Clone, Copy)]
pub enum ODR {
    /// Power-down / One-shot mode enabled
    PowerDown = 0b000,
    /// 1 Hz
    _1Hz = 0b001,
    /// 10 Hz
    _10Hz = 0b010,
    /// 25 Hz
    _25Hz = 0b011,
    /// 50 Hz
    _50Hz = 0b100,
    /// 75 Hz
    _75Hz = 0b101,
}

impl ODR {
    pub fn value(self) -> u8 {
        (self as u8) << 4
    }
}

/// SPI interface mode
#[derive(Debug, Clone, Copy)]
pub enum SPI_Mode {
    /// 4-wire mode (default)
    _4wire,
    /// 3-wire mode
    _3wire,
}

/// INT_DRDY pin configuration. (Refer to Table 19)
#[derive(Debug, Clone, Copy)]
pub enum INT_DRDY {
    /// Data signal (see CTRL_REG4)
    DataSignal = 0b00,
    /// Pressure high
    P_high = 0b01,
    /// Pressure low
    P_low = 0b10,
    /// Pressure low or high
    P_low_or_high = 0b011,
}

impl INT_DRDY {
    pub fn value(self) -> u8 {
        self as u8 // no need to shift, bits 0:1 (INT_S)
    }
}

/// FIFO mode selection. (Refer to Table 20)
#[derive(Debug, Clone, Copy)]
pub enum FIFO_MODE {
    /// Bypass mode
    Bypass = 0b000,
    /// FIFO mode
    FIFO = 0b001,
    /// Stream mode
    Stream = 0b010,
    /// Stream-to-FIFO mode
    Stream_to_FIFO = 0b011,
    /// Bypass-to-stream mode
    Bypass_to_stream = 0b100,
    /// Dynamic-stream mode
    Dynamic_Stream = 0b110,
    /// Bypass-to-FIFO mode
    Bypass_to_FIFO = 0b111,
}

impl FIFO_MODE {
    pub fn value(self) -> u8 {
        (self as u8) << 5 // shifted into the right position, can be used directly
    }
}

/// Interrupt active setting for the INT_DRDY pin: active high (default) or active low
#[derive(Debug, Clone, Copy)]
pub enum INT_ACTIVE {
    /// Active high
    High,
    /// Active low
    Low,
}

/// Interrupt pad setting for INT_DRDY pin: push-pull (default) or open-drain.
#[derive(Debug, Clone, Copy)]
pub enum INT_PIN {
    /// Push-pull
    PushPull,
    /// Open drain
    OpenDrain,
}
