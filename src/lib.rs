//! A platform agnostic driver to interface with LPS22HB pressure sensor module.
//! 
//! This driver allows you to:
//! - read atmospheric pressure in hPa, see [`read_pressure()`]
//! - read temperature in degrees Celsius, see [`read_temperature()`]
//! - enable single-shot data acquisition, see ['enable_one_shot()`]
//! 
//! [`read_pressure()`]: struct.LPS22HB.html#method.read_pressure
//! [`read_temperature()`]: struct.LPS22HB.html#method.read_temperature
//! ['enable_one_shot()`]: struct.LPS22HB.html#method.enable_one_shot
//! 
//! __NOTE__: This is a very early version of the crate. Only I2C interface is supported at the moment.
//! Only single shot mode is supported. 
//! 
//! 
//! ### Datasheets
//! - [LPS22HB](https://www.st.com/resource/en/datasheet/lps22hb.pdf)
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
//! use lps22hb::{sensor, LPS22HBInit};
//! 
//! let mut lps22 = LPS22HBInit {
//!         ..Default::default()
//!         }.with_interface(i2c_interface);
//! 
//! lps22.begin_sensor().unwrap();
//! 
//! lps22.enable_one_shot().unwrap();
//! 
//! let pressure = lps22.read_pressure().unwrap();
//! let temperature = lps22.read_temperature().unwrap();
//! ```
//!

#![no_std]
//#![deny(warnings, missing_docs)]

pub mod sensor;
use sensor::SensorSettings;

pub mod register;
use register::{Registers, Bitmasks};

pub mod interface;
use interface::{Interface};

/// Sensor's ID 
const WHOAMI: u8 = 0b10110001; // decimal value 177

/// The output of the temperature sensor must be divided by 100, see p. 10 of the datasheet.
const TEMP_SCALE: f32 = 100.0;
/// The output of the pressure sensor must be divided by 4096, see p. 10 of the datasheet.
const PRESS_SCALE: f32 = 4096.0;

/// LPS22HB init struct.
/// Use this struct to configure sensors and init LPS22HB with an interface of your choice.
pub struct LPS22HBInit {
    pub sensor: SensorSettings,    
}

impl Default for LPS22HBInit {
    fn default() -> Self {
        Self {
            sensor: SensorSettings::default(),            
        }
    }
}

impl LPS22HBInit {
    /// Constructs a new LPS22HB driver instance with a I2C or SPI peripheral.
    ///
    /// # Arguments
    /// * `interface` - `SpiInterface` or `I2cInterface`
    pub fn with_interface<T>(self, interface: T) -> LPS22HB<T>
    where
        T: Interface,
    {
        LPS22HB {
            interface,
            sensor: self.sensor,            
        }
    }
}

/// LPS22HB sensor
pub struct LPS22HB<T>
where
    T: Interface,
{
    interface: T,
    sensor: SensorSettings,    
}

impl<T> LPS22HB<T>
where
    T: Interface,
{   
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

    /// Raw sensor reading (3 bytes of pressure data and 2 bytes of temperature data)
    fn read_sensor_raw(&mut self) -> Result<(i32, i32), T::Error> {
        let mut data = [0u8;5];
        self.interface.read(Registers::PRESS_OUT_XL.addr(), &mut data)?;
        let p: i32 = (data[2] as i32) << 16 | (data[1] as i32) << 8 | (data[0] as i32);
        let t: i32 = (data[4] as i32) << 8 | (data[3] as i32);
        Ok((p, t))
    }

    /// Calculated pressure reading in hPa
    pub fn read_pressure(&mut self) -> Result<f32, T::Error> {
        let (p,_t) = self.read_sensor_raw()?;
        let pressure: f32 = (p as f32) / PRESS_SCALE;
        Ok(pressure)
    }

    /// Calculated temperaure reading in degrees Celsius 
    pub fn read_temperature(&mut self) -> Result<f32, T::Error> {
        let (_p,t) = self.read_sensor_raw()?;
        let temperature: f32 = (t as f32) / TEMP_SCALE;
        Ok(temperature)
    }
    
    /// Clear selected bits using a bitmask
    fn clear_register_bit_flag(&mut self, address: Registers, bitmask: Bitmasks) -> Result<(), T::Error> {
        let mut reg_data = [0u8;1];
        self.interface.read(address.addr(), &mut reg_data)?;
        let bitmask = bitmask.bitmask();
        let payload: u8 = reg_data[0] & !bitmask;
        self.interface.write(            
            address.addr(),
            payload,
        )?;
        Ok(())
    }    

    /// Set selected bits using a bitmask
    fn set_register_bit_flag(&mut self, address: Registers, bitmask: Bitmasks) -> Result<(), T::Error> {
        let mut reg_data = [0u8;1];
        self.interface.read(address.addr(), &mut reg_data)?;
        let payload: u8 = reg_data[0] | bitmask.bitmask();
        self.interface.write(            
            address.addr(),
            payload,
        )?;
        Ok(())
    }

    /// Enable single shot data acquisition (self cleared by hardware)
    pub fn enable_one_shot(&mut self) -> Result<(), T::Error> {
        self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::ONE_SHOT)?;
        Ok(())
    }
    

}
