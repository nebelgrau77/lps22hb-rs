//! I2C Interface
use super::Interface;
use embedded_hal::i2c::blocking::I2c;

/// Pressure sensor address for I2C communication
#[allow(non_camel_case_types)]
pub enum I2cAddress {
    /// SA0 pad tied to VCC
    SA0_VCC = 0b1011101,
    /// SA0 pad tied to ground
    SA0_GND = 0b1011100,
}

impl I2cAddress {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

/// This holds `I2C` and device address
pub struct I2cInterface<I2C> {
    i2c: I2C,
    dev_addr: u8,
}

impl<I2C> I2cInterface<I2C> {
    /// Initializes an Interface with `I2C` instance and device address
    /// # Arguments
    /// * `i2c` - I2C instance
    /// * `dev_addr` - `I2cAddress`: register address for the pressure sensor
    pub fn init(i2c: I2C, dev_addr: I2cAddress) -> Self {
        Self {
            i2c,
            dev_addr: dev_addr.addr(),
        }
    }
}

/// Implementation of `Interface`
impl<I2C> Interface for I2cInterface<I2C>
where
    I2C: I2c,
{
    type Error = I2C::Error;

    fn write(&mut self, addr: u8, value: u8) -> Result<(), Self::Error> {
        //let sensor_addr = self.dev_addr;
        self.i2c
            //.write(sensor_addr, &[addr, value])
            .write(self.dev_addr, &[addr, value])
    }

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        //let sensor_addr = self.dev_addr;
        self.i2c
            //.write_read(sensor_addr, &[addr], buffer)
            .write_read(self.dev_addr, &[addr], buffer)
    }
}
