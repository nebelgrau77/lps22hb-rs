//! Functions related to sensor measurements: reading value or status, setting offset and reference
//!
//! TO DO: add reference pressure setting

use super::*;

impl<T, E> LPS22HB<T>
where
    T: Interface<Error = E>,
{
    /// Read the device ID ("who am I")
    pub fn get_device_id(&mut self) -> Result<u8, T::Error> {
        let mut data = [0u8; 1];
        self.interface.read(Registers::WHO_AM_I.addr(), &mut data)?;
        let whoami = data[0];
        Ok(whoami)
    }

    /// Raw sensor reading (3 bytes of pressure data and 2 bytes of temperature data)
    fn read_sensor_raw(&mut self) -> Result<(i32, i32), T::Error> {
        let mut data = [0u8; 5];
        self.interface
            .read(Registers::PRESS_OUT_XL.addr(), &mut data)?;
        let p: i32 = (data[2] as i32) << 16 | (data[1] as i32) << 8 | (data[0] as i32);
        let t: i32 = (data[4] as i32) << 8 | (data[3] as i32);
        Ok((p, t))
    }

    /// Calculated pressure reading in hPa
    pub fn read_pressure(&mut self) -> Result<f32, T::Error> {
        let (p, _t) = self.read_sensor_raw()?;
        let pressure: f32 = (p as f32) / PRESS_SCALE;
        Ok(pressure)
    }

    /// Calculated temperaure reading in degrees Celsius
    pub fn read_temperature(&mut self) -> Result<f32, T::Error> {
        let (_p, t) = self.read_sensor_raw()?;
        let temperature: f32 = (t as f32) / TEMP_SCALE;
        Ok(temperature)
    }

    /// Calculated reference pressure reading in hPa
    pub fn read_reference_pressure(&mut self) -> Result<f32, T::Error> {
        let mut data = [0u8; 3];
        self.interface.read(Registers::REF_P_XL.addr(), &mut data)?;
        let p: i32 = (data[2] as i32) << 16 | (data[1] as i32) << 8 | (data[0] as i32);
        let pressure: f32 = (p as f32) / PRESS_SCALE;
        Ok(pressure)
    }

    /// Read pressure offset value, 16-bit data that can be used to implement One-Point Calibration (OPC) after soldering.
    pub fn read_pressure_offset(&mut self) -> Result<i16, T::Error> {
        let mut data = [0u8; 2];
        self.interface.read(Registers::RPDS_L.addr(), &mut data)?;
        let o: i16 = (data[1] as i16) << 8 | (data[0] as i16);
        Ok(o)
    }

    /// Set the pressure offset value (VALUE IN hPA!)
    pub fn set_pressure_offset(&mut self, offset: u16) -> Result<(), T::Error> {
        let mut payload = [0u8; 2];
        let offset = offset * 16;

        payload[0] = (offset & 0xff) as u8; // lower byte
        payload[1] = (offset >> 8) as u8; // upper byte

        self.interface.write(Registers::RPDS_L.addr(), payload[0])?;
        self.interface.write(Registers::RPDS_H.addr(), payload[1])?;

        Ok(())
    }

    /// Has new pressure data overwritten the previous one?
    pub fn pressure_data_overrun(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::STATUS, Bitmasks::P_OR)
    }

    /// Has new temperature data overwritten the previous one?
    pub fn temperature_data_overrun(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::STATUS, Bitmasks::T_OR)
    }

    /// Is new pressure data available?
    pub fn pressure_data_available(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::STATUS, Bitmasks::P_DA)
    }

    /// Is new temperature data available?
    pub fn temperature_data_available(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::STATUS, Bitmasks::T_DA)
    }
}
