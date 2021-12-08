//! Functions related to sensor measurements: reading value or status, setting offset and reference
//!
//! TO DO: add reference pressure setting

use super::*;

#[derive(Debug)]
/// Contents of the STATUS register (pressure and temperature overrun and data availability flags)
pub struct DataStatus {
    pub temp_overrun: bool,
    pub press_overrun: bool,
    pub temp_available: bool,
    pub press_available: bool,
}

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

    /// Get all the flags from the STATUS_REG register
    pub fn get_data_status(&mut self) -> Result<DataStatus, T::Error> {
        let status = DataStatus {
            /// Has new temperature data overwritten the previous one?
            temp_overrun: self.is_register_bit_flag_high(Registers::STATUS, Bitmasks::T_OR)?,
            /// Has new pressure data overwritten the previous one?
            press_overrun: self.is_register_bit_flag_high(Registers::STATUS, Bitmasks::P_OR)?,
            /// Is new temperature data available?
            temp_available: self.is_register_bit_flag_high(Registers::STATUS, Bitmasks::T_DA)?,
            /// Is new pressure data available?            
            press_available: self.is_register_bit_flag_high(Registers::STATUS, Bitmasks::P_DA)?,
        };
        Ok(status)
    }

    /// Triggers the one-shot mode, and a new acquisition starts when it is required.
    /// Enabling this mode is possible only if the device was previously in power-down mode.
    /// Once the acquisition is completed and the output registers updated,
    /// the device automatically enters in power-down mode. ONE_SHOT bit self-clears itself.
    pub fn one_shot(&mut self) -> Result<(), T::Error> {
        self.set_datarate(ODR::PowerDown)?; // make sure that Power down/one shot mode is enabled
        self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::ONE_SHOT)?;
        Ok(())
    }

    // --- THESE FUNCTIONS CAN BE REMOVED ---

    /*

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

     */
}
