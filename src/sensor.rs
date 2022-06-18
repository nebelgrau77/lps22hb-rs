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

    /// Calculated pressure reading in hPa
    pub fn read_pressure(&mut self) -> Result<f32, T::Error> {
        let mut data = [0u8; 3];
        self.interface
            .read(Registers::PRESS_OUT_XL.addr(), &mut data)?;
        let p: i32 = (data[2] as i32) << 16 | (data[1] as i32) << 8 | (data[0] as i32);
        let pressure = (p as f32) / PRESS_SCALE; // no need to take care of negative values
        Ok(pressure)
    }

    /// Calculated temperaure reading in degrees Celsius
    pub fn read_temperature(&mut self) -> Result<f32, T::Error> {
        let mut data = [0u8; 2];
        self.interface
            .read(Registers::TEMP_OUT_L.addr(), &mut data)?;
        let t: i16 = (data[1] as i16) << 8 | (data[0] as i16);
        let temperature = (t as f32) / TEMP_SCALE;
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

    /// Read threshold value for pressure interrupt generation
    pub fn read_threshold(&mut self) -> Result<i16, T::Error> {
        let mut data = [0u8; 2];
        self.interface.read(Registers::THS_P_L.addr(), &mut data)?;
        let o: i16 = (data[1] as i16) << 8 | (data[0] as i16);
        Ok(o)
    }

    /// Set the pressure offset value (VALUE IN hPA!)
    pub fn set_threshold(&mut self, threshold: u16) -> Result<(), T::Error> {
        let mut payload = [0u8; 2];
        let threshold = threshold * 16;

        payload[0] = (threshold & 0xff) as u8; // lower byte
        payload[1] = (threshold >> 8) as u8; // upper byte

        self.interface
            .write(Registers::THS_P_L.addr(), payload[0])?;
        self.interface
            .write(Registers::THS_P_H.addr(), payload[1])?;

        Ok(())
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

    /// Set the reference pressure (value in hPA)
    pub fn set_reference_pressure(&mut self, pressure: u16) -> Result<(), T::Error> {
        /*
        self.interface.read(Registers::REF_P_XL.addr(), &mut data)?;
        let p: i32 = (data[2] as i32) << 16 | (data[1] as i32) << 8 | (data[0] as i32);
        let pressure: f32 = (p as f32) / PRESS_SCALE;


        let pressure = pressure * PRESS_SCALE;

        let mut payload = [0u8; 3];

        // value must be split into three bytes

        payload[0] = (pressure & 0xff) as u8; // XL byte
        payload[1] = (pressure >> 8) as u8; // L byte
        payload[2] = (pressure >> 16) as u8; // H byte

        */

        /*

                /**
                 * @brief  pressure_ref:   The Reference pressure value is a 24-bit data
                *         expressed as 2’s complement. The value is used when AUTOZERO
                *         or AUTORIFP function is enabled.[set]
                *
                * @param  ctx    Read / write interface definitions
                * @param  buff   Buffer that contains data to write
                * @retval        Interface status (MANDATORY: return 0 -> no Error).
                *
                */
                int32_t lps22hb_pressure_ref_set(stmdev_ctx_t *ctx, int32_t val)
                {
                uint8_t buff[3];
                int32_t ret;

                buff[2] = (uint8_t)((uint32_t)val / 65536U);
                buff[1] = (uint8_t)((uint32_t)val - (buff[2] * 65536U)) / 256U;
                buff[0] = (uint8_t)((uint32_t)val - (buff[2] * 65536U) -
                                    (buff[1] * 256U));
                ret =  lps22hb_write_reg(ctx, LPS22HB_REF_P_XL, buff, 3);

                return ret;
                }

                /**
                 * @brief  pressure_ref:   The Reference pressure value is a 24-bit data
                *         expressed as 2’s complement. The value is used when AUTOZERO
                *         or AUTORIFP function is enabled.[get]
                *
                * @param  ctx    Read / write interface definitions
                * @param  buff   Buffer that stores data read
                * @retval        Interface status (MANDATORY: return 0 -> no Error).
                *
                */
                int32_t lps22hb_pressure_ref_get(stmdev_ctx_t *ctx, int32_t *val)
                {
                uint8_t buff[3];
                int32_t ret;

                ret =  lps22hb_read_reg(ctx, LPS22HB_REF_P_XL, buff, 3);
                *val = (int32_t)buff[2];
                *val = (*val * 256) + (int32_t)buff[1];
                *val = (*val * 256) + (int32_t)buff[0];

                return ret;
                }

        */

        Ok(())
    }

    /// Get all the flags from the STATUS_REG register
    pub fn get_data_status(&mut self) -> Result<DataStatus, T::Error> {
        // TO DO: use this value for reading all the bitflags in one go
        // use bitmasks
        let reg_value = self.read_register(Registers::STATUS)?;

        let status = DataStatus {
            /// Has new pressure data overwritten the previous one?
            press_overrun: match reg_value & Bitmasks::P_OR {
                0 => false,
                _ => true,
            },
            /// Has new temperature data overwritten the previous one?
            temp_overrun: match reg_value & Bitmasks::T_OR {
                0 => false,
                _ => true,
            },
            /// Is new pressure data available?
            press_available: match reg_value & Bitmasks::P_DA {
                0 => false,
                _ => true,
            },
            /// Is new temperature data available?
            temp_available: match reg_value & Bitmasks::T_DA {
                0 => false,
                _ => true,
            },
        };

        Ok(status)
    }

    /// Triggers the one-shot mode, and a new acquisition starts when it is required.
    /// Enabling this mode is possible only if the device was previously in power-down mode.
    /// Once the acquisition is completed and the output registers updated,
    /// the device automatically enters in power-down mode. ONE_SHOT bit self-clears itself.
    pub fn one_shot(&mut self) -> Result<(), T::Error> {
        self.set_datarate(Odr::PowerDown)?; // make sure that Power down/one shot mode is enabled
        self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::ONE_SHOT)?;
        Ok(())
    }
}
