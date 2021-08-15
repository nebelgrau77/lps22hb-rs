//! Various functions related to FIFO
//! 
//! TO DO: 
//! - add all the FIFO-related functions



use super::*; 

impl<T, E> LPS22HB<T> 
where
    T: Interface<Error = E>,
{   

    /*
    /// FIFO enable/disable
    pub fn fifo_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => {
                self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::FIFO_EN)
            }
            false => {
                self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::FIFO_EN)
            }
        }
    }

    /// Select FIFO operation mode (see Table 22 for details)        
    pub fn fifo_mode_config(&mut self, mode: FIFO_MODE) -> Result<(), T::Error> {
        let mut reg_data = [0u8];
        self.interface.read(Registers::FIFO_CTRL.addr(), &mut reg_data)?;
        let mut payload = reg_data[0];
        payload &= !Bitmasks::F_MODE_MASK;
        payload |= mode.value();
        self.interface.write(
            Registers::FIFO_CTRL.addr(),
            payload,
        )?;
        Ok(())
    }

    /// Select sample size for FIFO Mean mode running average (see Table 23 for details)        
    pub fn fifo_mean_config(&mut self, sample: FIFO_MEAN) -> Result<(), T::Error> {
        let mut reg_data = [0u8];
        self.interface.read(Registers::FIFO_CTRL.addr(), &mut reg_data)?;
        let mut payload = reg_data[0];
        payload &= !Bitmasks::WTM_POINT_MASK;
        payload |= sample.value();
        self.interface.write(
            Registers::FIFO_CTRL.addr(),
            payload,
        )?;
        Ok(())
    }

    /// FIFO empty flag on INT_DRDY pin
    pub fn fifo_empty_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => {
                self.set_register_bit_flag(Registers::CTRL_REG4, Bitmasks::F_EMPTY)
            }
            false => {
                self.clear_register_bit_flag(Registers::CTRL_REG4, Bitmasks::F_EMPTY)
            }
        }
    }

    /// FIFO filled up to threshold (watermark) level on INT_DRDY pin 
    pub fn fifo_filled_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => {
                self.set_register_bit_flag(Registers::CTRL_REG4, Bitmasks::F_FTH)
            }
            false => {
                self.clear_register_bit_flag(Registers::CTRL_REG4, Bitmasks::F_FTH)
            }
        }
    }

    /// FIFO overrun interrupt on INT_DRDY pin 
    pub fn fifo_overrun_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => {
                self.set_register_bit_flag(Registers::CTRL_REG4, Bitmasks::F_OVR)
            }
            false => {
                self.clear_register_bit_flag(Registers::CTRL_REG4, Bitmasks::F_OVR)
            }
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

    /// Is FIFO empty?
    pub fn fifo_empty_status(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::FIFO_STATUS, Bitmasks::EMPTY_FIFO)
    }

    */
}