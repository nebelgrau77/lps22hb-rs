//! Various functions related to FIFO
//!
//! TO DO:
//! - add all the FIFO-related functions

use super::*;

impl<T, E> LPS22HB<T>
where
    T: Interface<Error = E>,
{
    // The FIFO buffer is enabled when the FIFO_EN bit in CTRL_REG2 (11h) is set to '1'
    // and each mode is selected by the FIFO_MODE[2:0] bits in FIFO_CTRL (14h).

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

    /// Is FIFO filling equal or higher than the threshold?
    pub fn fifo_threshold_status(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::FTH_FIFO)
    }

    /// Is FIFO full and at least one sample has been overwritten?
    pub fn fifo_overrun_status(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::OVR)
    }

    /// Read FIFO stored data level
    pub fn fifo_stored_data_level(&mut self) -> Result<u8, T::Error> {
        let mut data = [0u8; 1];
        self.interface
            .read(Registers::FIFO_STATUS.addr(), &mut data)?;
        let level = data[0] & Bitmasks::FSS_MASK;
        Ok(level)
    }

    /// Stop on FIFO watermark (enable FIFO watermark use)
    pub fn stop_on_fth(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::STOP_ON_FTH),
            false => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::STOP_ON_FTH),
        }
    }
}
