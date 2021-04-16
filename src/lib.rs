//! A platform agnostic driver to interface with LPS22HB pressure sensor module.
//!
//! ### Datasheets
//! - [LPS22HB](https://www.st.com/resource/en/datasheet/lps22hb.pdf)
//!
#![no_std]
// #![deny(warnings, missing_docs)]

pub mod pressure;

pub mod register;

use pressure::PressureSettings;

pub mod interface;
use interface::{Interface, Sensor};

/// Sensor's ID
const WHO_AM_I      : u8 = 0x0F;    

/// The output of the temperature sensor must be divided by 100, see p. 10 of the datasheet.
const TEMP_SCALE: f32 = 100.0;
/// The output of the pressure sensor must be divided by 4096, see p. 10 of the datasheet.
const PRESS_SCALE: f32 = 4096.0;

/// LPS22HB init struct.
/// Use this struct to configure sensors and init LPS22HB with an interface of your choice.
pub struct LPS22HBInit {
    pub pressure: PressureSettings,
    
}

impl Default for LPS22HBInit {
    fn default() -> Self {
        Self {
            pressure: PressureSettings::default(),            
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
            pressure: self.pressure,            
        }
    }
}

/// LPS22HB sensor
pub struct LPS22HB<T>
where
    T: Interface,
{
    interface: T,
    pressure: PressureSettings,    
}

impl<T> LPS22HB<T>
where
    T: Interface,
{
    fn reachable(&mut self, sensor: Sensor) -> Result<bool, T::Error> {
        use Sensor::*;
        let mut bytes = [0u8; 1];
        let (who_am_i, register) = match sensor {
            // how does this work?
            Pressure | Temperature => (WHO_AM_I, register::LPS::WHO_AM_I.addr()),            
        };

        self.interface.read(sensor, register, &mut bytes)?;
        Ok(bytes[0] == who_am_i)
    }

    // I don't need this - single sensor only, can use function below

    /// Verifies communication with WHO_AM_I register
    pub fn accel_is_reacheable(&mut self) -> Result<bool, T::Error> {
        self.reachable(Sensor::Pressure)
    }
    

    // need to figure out what has to be set up

    /// Initializes Accelerometer with sensor settings.
    pub fn begin_accel(&mut self) -> Result<(), T::Error> {
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::CTRL_REG5_XL.addr(),
            self.accel.ctrl_reg5_xl(),
        )?;
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::CTRL_REG6_XL.addr(),
            self.accel.ctrl_reg6_xl(),
        )?;
        self.interface.write(
            Sensor::Accelerometer,
            register::AG::CTRL_REG7_XL.addr(),
            self.accel.ctrl_reg7_xl(),
        )?;
        Ok(())
    }
    
    // single sensor, only needs single press_data_available function

    fn data_available(&mut self, sensor: Sensor) -> Result<u8, T::Error> {
        use Sensor::*;
        let register = match sensor {
            Accelerometer | Gyro | Temperature => register::AG::STATUS_REG_1.addr(),
            Magnetometer => register::Mag::STATUS_REG_M.addr(),
        };
        let mut bytes = [0u8; 1];
        self.interface.read(sensor, register, &mut bytes)?;
        Ok(bytes[0])
    }
    /// Sees if new Accelerometer data is available
    pub fn accel_data_available(&mut self) -> Result<bool, T::Error> {
        match self.data_available(Sensor::Accelerometer)? {
            x if x & 0x01 > 0 => Ok(true),
            _ => Ok(false),
        }
    }
    


    /// raw sensor reading for x, y, z axis
    fn read_sensor_raw(&mut self, sensor: Sensor, addr: u8) -> Result<(i16, i16, i16), T::Error> {
        let mut bytes = [0u8; 6];
        self.interface.read(sensor, addr, &mut bytes)?;
        let x: i16 = (bytes[1] as i16) << 8 | bytes[0] as i16;
        let y: i16 = (bytes[3] as i16) << 8 | bytes[2] as i16;
        let z: i16 = (bytes[5] as i16) << 8 | bytes[4] as i16;
        Ok((x, y, z))
    }
    /// raw accelerometer readings
    pub fn read_accel_raw(&mut self) -> Result<(i16, i16, i16), T::Error> {
        self.read_sensor_raw(Sensor::Accelerometer, register::AG::OUT_X_L_XL.addr())
    }
    /// calculated accelerometer readings (x, y, z)
    pub fn read_accel(&mut self) -> Result<(f32, f32, f32), T::Error> {
        let (x, y, z) = self.read_accel_raw()?;
        let sensitivity = self.accel.scale.sensitivity();
        Ok((
            x as f32 * sensitivity,
            y as f32 * sensitivity,
            z as f32 * sensitivity,
        ))
    }
    
    // doesn't really need separate temperature reading, better use a struct with
    // pressure and temperature values

    /* 
    /// Reads calculated temperature in Celsius
    pub fn read_temp(&mut self) -> Result<f32, T::Error> {
        let mut bytes = [0u8; 2];
        self.interface.read(
            Sensor::Accelerometer,
            register::AG::OUT_TEMP_L.addr(),
            &mut bytes,
        )?;
        let result: i16 = (bytes[1] as i16) << 8 | bytes[0] as i16;
        Ok((result as f32) / TEMP_SCALE + TEMP_BIAS)
    }
    */
}