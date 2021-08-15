//! Various functions related to configuration
//! 
//! TO DO:
//! - add all the other settings here


use super::*; 

impl<T, E> LPS22HB<T> 
where
    T: Interface<Error = E>,
{   

    /// Enable single shot data acquisition (self cleared by hardware)
    pub fn enable_one_shot(&mut self) -> Result<(), T::Error> {
        self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::ONE_SHOT)?;
        Ok(())
    }

    /// Set output data rate        
    pub fn set_datarate(&mut self, odr: ODR) -> Result<(), T::Error> {
        let mut reg_data = [0u8];
        self.interface.read(Registers::CTRL_REG1.addr(), &mut reg_data)?;
        let mut payload = reg_data[0];
        payload &= !Bitmasks::ODR_MASK;
        payload |= odr.value();
        self.interface.write(
            Registers::CTRL_REG1.addr(),
            payload,
        )?;
        Ok(())
    }

     
    /*

    /// Temperature internal average configuration (default 64).
    pub fn temperature_resolution(&mut self, resolution: TEMP_RES) -> Result<(), T::Error> {
        let mut reg_data = [0u8];
        self.interface.read(Registers::RES_CONF.addr(), &mut reg_data)?;
        let mut payload = reg_data[0];
        payload &= !Bitmasks::AVGT_MASK;
        payload |= resolution.value();
        self.interface.write(
            Registers::RES_CONF.addr(),
            payload,
        )?;
        Ok(())
    }

    /// Pressure internal average configuration (default 512).
    pub fn pressure_resolution(&mut self, resolution: PRESS_RES) -> Result<(), T::Error> {
        let mut reg_data = [0u8];
        self.interface.read(Registers::RES_CONF.addr(), &mut reg_data)?;
        let mut payload = reg_data[0];
        payload &= !Bitmasks::AVGP_MASK;
        payload |= resolution.value();
        self.interface.write(
            Registers::RES_CONF.addr(),
            payload,
        )?;
        Ok(())
    }

 


    /// Enable single shot data acquisition (self cleared by hardware)
    pub fn enable_one_shot(&mut self) -> Result<(), T::Error> {
        self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::ONE_SHOT)?;
        Ok(())
    }


    /// Enable or disable block data update
    pub fn bdu_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => {
                self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::BDU)
            }
            false => {
                self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::BDU)
            }
        }
    }

    /// Configuration of the interrupt generation (enabled/disable)
    pub fn int_generation_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => {
                self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::DIFF_EN)
            }
            false => {
                self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::DIFF_EN)
            }
        }
    }

    /// Resets the Autozero function. Self-cleared.
    pub fn autozero_reset(&mut self) -> Result<(), T::Error> {
        self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::RESET_AZ)
    }

    /// AUTOZERO: when set to ‘1’, the actual pressure output value is copied in
    /// REF_P_H (0Ah), REF_P_L (09h) and REF_P_XL (08h).
    /// When this bit is enabled, the register content of REF_P is subtracted from the pressure output value.
    pub fn autozero_config(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => {
                self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::AUTOZERO)
            }
            false => {
                self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::AUTOZERO)
            }
        }
    }

    /// Disables I2C interface (default 0, I2C enabled)
    pub fn i2c_disable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => {
                self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::I2C_DIS)
            }
            false => {
                self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::I2C_DIS)
            }
        }
    }

    /// Sets SPI Mode (default 4-wire)
    pub fn spi_config(&mut self, mode: SPI_Mode) -> Result<(), T::Error> {
        match mode {
            SPI_Mode::_3wire => {
                self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::SIM)
            }
            SPI_Mode::_4wire => {
                self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::SIM)
            }
        } 
    }


    /// Interrupt request latching to INT_SOURCE
    pub fn int_latch_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => {
                self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::LIR)
            }
            false => {
                self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::LIR)
            }
        }
    }

    /// Enable interrupt on differential pressure low event
    pub fn diff_press_low_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => {
                self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PL_E)
            }
            false => {
                self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PL_E)
            }
        }
    }

    /// Enable interrupt on differential pressure high event
    pub fn diff_press_high_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => {
                self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PH_E)
            }
            false => {
                self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PH_E)
            }
        }
    }   

    /// Data-ready signal on INT_DRDY pin 
    pub fn data_signal_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => {
                self.set_register_bit_flag(Registers::CTRL_REG4, Bitmasks::DRDY)
            }
            false => {
                self.clear_register_bit_flag(Registers::CTRL_REG4, Bitmasks::DRDY)
            }
        }
    }
     */

}