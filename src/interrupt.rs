//! Various functions related to interrupts
//!

use super::*;

impl<T, E> LPS22HB<T>
where
    T: Interface<Error = E>,
{
    /// Configuration of the interrupt generation (enabled/disable)
    pub fn int_generation_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::DIFF_EN),
            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::DIFF_EN),
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

    /// Interrupt active high/low (default active high)
    pub fn interrupt_active(&mut self, setting: INT_ACTIVE) -> Result<(), T::Error> {
        match setting {
            INT_ACTIVE::High => {
                self.clear_register_bit_flag(Registers::CTRL_REG3, Bitmasks::INT_H_L)
            }
            INT_ACTIVE::Low => self.set_register_bit_flag(Registers::CTRL_REG3, Bitmasks::INT_H_L),
        }
    }

    /// Interrupt pin configuration: push-pull (default) or open drain
    pub fn interrupt_pin_config(&mut self, setting: INT_PIN) -> Result<(), T::Error> {
        match setting {
            INT_PIN::PushPull => {
                self.clear_register_bit_flag(Registers::CTRL_REG3, Bitmasks::PP_OD)
            }
            INT_PIN::OpenDrain => self.set_register_bit_flag(Registers::CTRL_REG3, Bitmasks::PP_OD),
        }
    }

    /// Configure INT_DRDY pin
    pub fn int_drdy_config(&mut self, config: INT_DRDY) -> Result<(), T::Error> {
        let mut reg_data = [0u8];
        self.interface
            .read(Registers::CTRL_REG3.addr(), &mut reg_data)?;
        let mut payload = reg_data[0];
        payload &= !Bitmasks::INT_S_MASK;
        payload |= mode.value();
        self.interface.write(Registers::CTRL_REG3.addr(), payload)?;
        Ok(())
    }

    /// Has any interrupt event been generated? (self clearing)
    pub fn interrupt_active(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::INT_SOURCE, Bitmasks::IA)
    }

    /// Has low differential pressure event been generated? (self clearing)
    pub fn low_pressure_event_occurred(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::INT_SOURCE, Bitmasks::PL)
    }

    /// Has high differential pressure event been generated? (self clearing)
    pub fn high_pressure_event_occurred(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::INT_SOURCE, Bitmasks::PH)
    }

    /// Enable interrupt on differential pressure high event
    pub fn diff_press_high_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PHE),
            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::PHE),
        }
    }
}
