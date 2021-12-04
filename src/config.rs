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
        self.interface
            .read(Registers::CTRL_REG1.addr(), &mut reg_data)?;
        let mut payload = reg_data[0];
        payload &= !Bitmasks::ODR_MASK;
        payload |= odr.value();
        self.interface.write(Registers::CTRL_REG1.addr(), payload)?;
        Ok(())
    }

    /// Enable or disable block data update
    pub fn bdu_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::BDU),
            false => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::BDU),
        }
    }

    /// Configuration of the interrupt generation (enabled/disable)
    pub fn int_generation_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::DIFF_EN),
            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::DIFF_EN),
        }
    }

    /// Resets the Autozero function. Self-cleared.
    pub fn autozero_reset(&mut self) -> Result<(), T::Error> {
        self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::RESET_AZ)
    }

    /// AUTOZERO: when set to ‘1’, the measured pressure is used
    /// as the reference in REF_P (0x15, 0x16, 0x17).
    /// From that point on the output pressure registers are updated and the same value
    /// is also used for interrupt generation.
    /// The register content of REF_P is subtracted from the measured pressure.
    /// PRESS_OUT = measured pressure - REF_P
    /// P_DIFF_IN = measured pressure - REF_P
    ///     
    pub fn autozero_config(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::AUTOZERO),
            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::AUTOZERO),
        }
    }

    /// Disables I2C interface (default 0, I2C enabled)
    pub fn i2c_disable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::I2C_DIS),
            false => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::I2C_DIS),
        }
    }

    /// Sets SPI Mode (default 4-wire)
    pub fn spi_config(&mut self, mode: SPI_Mode) -> Result<(), T::Error> {
        match mode {
            SPI_Mode::_3wire => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::SIM),
            SPI_Mode::_4wire => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::SIM),
        }
    }

    /// Enable interrupt on differential pressure low event
    pub fn diff_press_low_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PLE),
            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PLE),
        }
    }

    /// Enable interrupt on differential pressure high event
    pub fn diff_press_high_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PHE),
            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PHE),
        }
    }

    /// Interrupt request latching to INT_SOURCE
    pub fn int_latch_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::LIR),
            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::LIR),
        }
    }

    /// Data-ready signal on INT_DRDY pin
    pub fn data_signal_drdy_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG3, Bitmasks::DRDY),
            false => self.clear_register_bit_flag(Registers::CTRL_REG3, Bitmasks::DRDY),
        }
    }

 


}
