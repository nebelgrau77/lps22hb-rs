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
//! __NOTE__: Only I2C interface is supported at the moment.
//!  //!
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
//!//!
//! lps22.one_shot().unwrap();
//!
//! let pressure = lps22.read_pressure().unwrap();
//! let temperature = lps22.read_temperature().unwrap();
//! ```
//!

#![no_std]
//#![deny(warnings, missing_docs)]

pub mod config;
pub mod fifo;
pub mod interface;
pub mod interrupt;
pub mod register;
pub mod sensor;
use interface::Interface;
use register::{Bitmasks, Registers};

/// Sensor's ID
//const WHOAMI: u8 = 0b10110001; // decimal value 177

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

    /// Read a byte from the given register.
    fn read_register(&mut self, address: Registers) -> Result<u8, T::Error> {
        let mut reg_data = [0u8];
        self.interface.read(address.addr(), &mut reg_data)?;
        Ok(reg_data[0])
    }

    /// Clear selected bits using a bitmask
    fn clear_register_bit_flag(&mut self, address: Registers, bitmask: u8) -> Result<(), T::Error> {
        let mut reg_data = [0u8; 1];
        self.interface.read(address.addr(), &mut reg_data)?;
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
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum Odr {
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

impl Odr {
    pub fn value(self) -> u8 {
        (self as u8) << 4
    }
}

/// SPI interface mode
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum SPIMode {
    /// 4-wire mode (default)
    _4wire,
    /// 3-wire mode
    _3wire,
}

/// FIFO mode selection. (Refer to Table 20)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FIFOMode {
    /// Bypass mode
    Bypass = 0b000,
    /// FIFO mode
    FIFO = 0b001,
    /// Stream mode
    Stream = 0b010,
    /// Stream-to-FIFO mode
    StreamToFIFO = 0b011,
    /// Bypass-to-stream mode
    BypassToStream = 0b100,
    /// Dynamic-stream mode
    DynamicStream = 0b110,
    /// Bypass-to-FIFO mode
    BypassToFIFO = 0b111,
}

impl FIFOMode {
    pub fn value(self) -> u8 {
        (self as u8) << 5 // shifted into the correct position, can be used directly
    }
}

/// INT_DRDY pin configuration. (Refer to Table 19)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum IntDrdy {
    /// Data signal (see CTRL_REG4)
    DataSignal = 0b00,
    /// Pressure high
    PHigh = 0b01,
    /// Pressure low
    PLow = 0b10,
    /// Pressure low or high
    PLowOrHigh = 0b011,
}

impl IntDrdy {
    pub fn value(self) -> u8 {
        self as u8 // no need to shift, bits 0:1 (INT_S)
    }
}

/// Interrupt active setting for the INT_DRDY pin: active high (default) or active low
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum IntActive {
    /// Active high
    High,
    /// Active low
    Low,
}

impl IntActive {
    pub fn value(self) -> u8 {
        match self {
            INT_ACTIVE::High => 0,
            INT_ACTIVE::Low => 1,
        }
    }
}

/// Interrupt pad setting for INT_DRDY pin: push-pull (default) or open-drain.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum IntPin {
    /// Push-pull
    PushPull,
    /// Open drain
    OpenDrain,
}

impl IntPin {
    pub fn value(self) -> u8 {
        match self {
            IntPin::PushPull => 0,
            IntPin::OpenDrain => 1,
        }
    }
}

/// Settings for various FIFO- and interrupt-related flags, Enabled or Disabled
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum Flag {
    /// Enabled (bit set)
    Enabled,
    /// Disabled (bit cleared)
    Disabled,
}

impl Flag {
    pub fn status(self) -> u8 {
        match self {
            Flag::Disabled => 0,
            Flag::Enabled => 1,
        }
    }
}

/// FIFO on/off
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum FIFOOn {
    /// Enabled (bit set)
    Enabled,
    /// Disabled (bit cleared)
    Disabled,
}

impl FIFOOn {
    pub fn value(self) -> u8 {
        match self {
            FIFOOn::Disabled => 0,
            FIFOOn::Enabled => 1,
        };
        status
    }
}
