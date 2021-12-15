//! Various functions related to FIFO
//!
//! TO DO:
//! - add all the FIFO-related functions

use super::*;

/// FIFO settings
#[derive(Debug)]
pub struct FIFOConfig {
    /// Stop on FIFO watermark (enable FIFO watermark use)
    pub enable_watermark: FLAG, // default disabled
    /// Select FIFO operation mode (see Table 22 for details)        
    pub fifo_mode: FIFO_MODE, // default Bypass
    /// Set the watermark level
    pub watermark_level: u8, // default 0
}

impl Default for FIFOConfig {
    fn default() -> Self {
        FIFOConfig {
            enable_watermark: FLAG::Disabled,      // disabled
            fifo_mode: FIFO_MODE::Bypass, // Bypass mode
            watermark_level: 32u8,        // 0 does not make sense as a default value
        }
    }
}

impl FIFOConfig {
    /// Returns values to be written to CTRL_REG2 and FIFO_CTRL:
    fn f_ctrl_reg2(&self) -> u8 {
        let mut data = 0u8;
        // THIS RESULT MUST THEN BE COMBINED WITH THE OTHER BIT SETTINGS
        if self.enable_watermark.status() {
            data |= 1 << 5;
        }
        data
    }
    fn f_fifo_ctrl(&self) -> u8 {
        let mut data = 0u8;
        data |= self.fifo_mode.value();
        data |= self.watermark_level;
        data
    }
}

#[derive(Debug)]
/// Contents of the FIFO_STATUS register (threshold reached, overrun, empty, stored data level)
pub struct FifoStatus {
    pub fifo_thresh_reached: bool,
    pub fifo_overrun: bool,
    pub fifo_empty: bool,
    pub fifo_level: u8,
}

impl<T, E> LPS22HB<T>
where
    T: Interface<Error = E>,
{
    // The FIFO buffer is enabled when the FIFO_EN bit in CTRL_REG2 (11h) is set to '1'
    // and each mode is selected by the FIFO_MODE[2:0] bits in FIFO_CTRL (14h).

    /// Enable and configure FIFO
    pub fn enable_fifo(&mut self, flag: FIFO_ON, config: FIFOConfig) -> Result<(), T::Error> {
        match flag {
            FIFO_ON::Enabled => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::FIFO_EN),
            FIFO_ON::Disabled => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::FIFO_EN),
        }?;

        match config.enable_watermark {
            FLAG::Enabled => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::STOP_ON_FTH),
            FLAG::Disabled => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::STOP_ON_FTH),
        }?;

        /*
        let mut reg_data = [0u8];
        self.interface
            .read(Registers::CTRL_REG2.addr(), &mut reg_data)?;
        reg_data[0] |= config.f_ctrl_reg2();
        self.interface
            .write(Registers::CTRL_REG2.addr(), reg_data[0])?;
        */
        self.interface
            .write(Registers::FIFO_CTRL.addr(), config.f_fifo_ctrl())?;

        Ok(())
    }

    // --- THIS FUNCTIONS COULD BE REMOVED ---

    /*

    /// FIFO enable/disable
    pub fn fifo_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::FIFO_EN),
            false => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::FIFO_EN),
        }
    }

    /// Select FIFO operation mode (see Table 20 for details)
    pub fn fifo_mode_config(&mut self, mode: FIFO_MODE) -> Result<(), T::Error> {
        let mut reg_data = [0u8];
        self.interface
            .read(Registers::FIFO_CTRL.addr(), &mut reg_data)?;
        let mut payload = reg_data[0];
        payload &= !Bitmasks::F_MODE_MASK;
        payload |= mode.value();
        self.interface.write(Registers::FIFO_CTRL.addr(), payload)?;
        Ok(())
    }

     */

    // -- THESE FUNCTIONS CAN BE REPLACED WITH INTERRUPT CONFIGURATION ---

    /*

    /// FIFO full flag on INT_DRDY pin
    pub fn fifo_full_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG3, Bitmasks::F_FSS5),
            false => self.clear_register_bit_flag(Registers::CTRL_REG3, Bitmasks::F_FSS5),
        }
    }

    /// FIFO filled up to threshold (watermark) level on INT_DRDY pin
    pub fn fifo_fth_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG3, Bitmasks::F_FTH),
            false => self.clear_register_bit_flag(Registers::CTRL_REG3, Bitmasks::F_FTH),
        }
    }

    /// FIFO overrun interrupt on INT_DRDY pin
    pub fn fifo_overrun_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG3, Bitmasks::F_OVR),
            false => self.clear_register_bit_flag(Registers::CTRL_REG3, Bitmasks::F_OVR),
        }
    }

     */

    // --- END OF THE BLOCK

    /// Get flags and FIFO level from the FIFO_STATUS register
    pub fn get_fifo_status(&mut self) -> Result<FifoStatus, T::Error> {
        
        let reg_value = self.read_register(Registers::FIFO_STATUS)?;
        

        let fifo_level_value = self.read_fifo_level()?;

        let status = FifoStatus {

            fifo_thresh_reached: match reg_value & Bitmasks::FTH_FIFO {
                0 => false,
                _ => true,
            },
            /// Is FIFO full and at least one sample has been overwritten?
            fifo_overrun: match reg_value & Bitmasks::OVR {
                0 => false,
                _ => true,
            },
            /*
            /// Is FIFO filling equal or higher than the threshold?
            fifo_thresh_reached: self
                .is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::FTH_FIFO)?,
            /// Is FIFO full and at least one sample has been overwritten?
            fifo_overrun: self.is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::OVR)?,
            */
            /// Is FIFO empty?
            /// 
            
            fifo_empty: match fifo_level_value {
                0 => true,
                _ => false,
            },
             
            /// Read FIFO stored data level
            fifo_level: fifo_level_value,
        };
        Ok(status)
    }

    // --- THESE FUNCTIONS COULD BE REMOVED ---

    /*

    /// Is FIFO filling equal or higher than the threshold?
    pub fn fifo_threshold_status(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::FTH_FIFO)
    }

    /// Is FIFO full and at least one sample has been overwritten?
    pub fn fifo_overrun_status(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::OVR)
    }

     */

    /// Read FIFO stored data level
    // pub fn read_fifo_level(&mut self) -> Result<u8, T::Error> {
    fn read_fifo_level(&mut self) -> Result<u8, T::Error> {
        let mut data = [0u8; 1];
        self.interface
            .read(Registers::FIFO_STATUS.addr(), &mut data)?;
        let level = data[0] & Bitmasks::FSS_MASK;
        Ok(level)
    }

    // --- THESE FUNCTIONS COULD BE REMOVED

    /*

    /// Stop on FIFO watermark (enable FIFO watermark use)
    pub fn stop_on_fth(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::STOP_ON_FTH),
            false => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::STOP_ON_FTH),
        }
    }

    /// Set the watermark level
    pub fn set_watermark_level(&mut self, level: u8) -> Result<(), T::Error> {
        let wtm: u8 = match level {
            // if the input value exceeds the capacity, default to maximum
            l if l < 33 => l,
            _ => 32,
        };
        let mut reg_data = [0u8];
        self.interface
            .read(Registers::FIFO_CTRL.addr(), &mut reg_data)?;
        let mut payload = reg_data[0];
        payload &= !Bitmasks::WTM_MASK;
        payload |= mode.value();
        self.interface.write(Registers::FIFO_CTRL.addr(), payload)?;
        Ok(())
    }
    */
}
