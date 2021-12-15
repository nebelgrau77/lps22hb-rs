//! Various functions related to interrupts
//!
//! TO DO: 
//! - "enable" flag in the interrupt enable function must be moved to the config, called "enable_differential"
//! - interrupt status reading must be in one go, as with latching on it would clear the register by reading the first bit the way it's configured now
//! 

use super::*;

/// Interrupt pin settings
#[derive(Debug)]
pub struct InterruptConfig {
    /// configure interrupt pin as active high or active low 
    pub active_high_or_low: bool, 
    /// configure interrupt pin as  push-pull or open drain
    pub pushpull_or_opendrain: bool,
    /// configure data signal on the interrupt pin
    pub data_signal_config: INT_DRDY,
    /// enable FIFO full flag on interrupt pin
    pub enable_fifo_full: bool, 
    /// enable FIFO watermark flag on interrupt pin
    pub enable_fifo_fth: bool, 
    /// enable FIFO overrun flag on interrupt pin
    pub enable_fifo_overrun: bool,
    /// enable data ready signal on interrupt pin
    pub enable_data_ready: bool,
    /// enable latching interrupt request to INT_SOURCE register
    pub enable_latch_interrupt: bool,
    /// enable low pressure event on interrupt pin
    pub enable_low_event: bool,
    /// enable hihg pressure event on interrupt pin
    pub enable_high_event: bool,
}

impl Default for InterruptConfig {
    fn default() -> Self {
        InterruptConfig {
            active_high_or_low: false,                  // active high (CTRL_REG3)
            pushpull_or_opendrain: false,               // push-pull (CTRL_REG3)
            data_signal_config: INT_DRDY::DataSignal,   // data signal on INT_DRDY pin (CTRL_REG3)
            enable_fifo_full: false,                    // disabled (CTRL_REG3)
            enable_fifo_fth: false,                     // disabled (CTRL_REG3)
            enable_fifo_overrun: false,                 // disabled (CTRL_REG3)
            enable_data_ready: false,                   // disabled (CTRL_REG3)
            enable_latch_interrupt: false,              // interrupt request not latched (INTERRUPT_CFG)
            enable_low_event: false,                    // disable interrupt request on low pressure event (INTERRUPT_CFG)
            enable_high_event: false,                   // disable interrupt request on low pressure event (INTERRUPT_CFG)
        }
    }
}

impl InterruptConfig {
    /// Returns values to be written to CTRL_REG3, CTRL_REG4 and INTERRUPT_CFG:
    fn int_ctrl_reg3(&self) -> u8 {
        let mut data = 0u8;
        if self.active_high_or_low {
            data |= 1 << 7;
        }
        if self.pushpull_or_opendrain {
            data |= 1 << 6;
        }
        if self.enable_fifo_full {
            data |= 1 << 5;
        }
        if self.enable_fifo_fth {
            data |= 1 << 4;
        }
        if self.enable_fifo_overrun {
            data |= 1 << 3;
        }
        if self.enable_data_ready {
            data |= 1 << 2;
        }        
        data |= self.data_signal_config.value();
        data
    }    
    fn int_interrupt_cfg(&self) -> u8 {
        
        let mut data = 0u8;

        if self.enable_latch_interrupt {
            data |= 1 << 2;
        }
        if self.enable_low_event {
            data |= 1 << 1;
        }
        if self.enable_high_event {
            data |= 1;
        }
        data // this must be OR'ed with the content of the INTERRUPT_CFG
    }
}

#[derive(Debug)]
/// Contents of the INT_SOURCE register (interrupt active and differential pressure events flags)
pub struct IntSource {
    pub interrupt_active: bool,
    pub diff_press_low: bool,
    pub diff_press_high: bool,    
}

impl<T, E> LPS22HB<T>
where
    T: Interface<Error = E>,
{
    /// Enable interrupts and configure the interrupt pin
    pub fn enable_interrupts(&mut self, flag: bool, config: InterruptConfig,) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::DIFF_EN),
            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::DIFF_EN),
        }?;
        self.interface
            .write(Registers::CTRL_REG3.addr(), config.int_ctrl_reg3())?;        
        
        let mut buffer = [0u8;1];
        self.read_register(Registers::INTERRUPT_CFG)?;        
        let mut interrupt_cfg = 0u8;
        interrupt_cfg |= config.int_interrupt_cfg();
       
        self.interface
            .write(Registers::INTERRUPT_CFG.addr(), interrupt_cfg)?;
        Ok(())
    }
    
    // --- THE FOLLOWING SECTION COULD BE REMOVED --- 

    /*
    
    
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
    pub fn interrupt_pin_active(&mut self, setting: INT_ACTIVE) -> Result<(), T::Error> {
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
        payload |= config.value();
        self.interface.write(Registers::CTRL_REG3.addr(), payload)?;
        Ok(())
    }

    */

    /// Get all the flags from the INT_SOURCE register (NOTE: INT_SOURCE register is cleared by reading it)
    pub fn get_int_status(&mut self) -> Result<IntSource, T::Error> {        
        let status = IntSource {
            /// Has any interrupt event been generated? (self clearing)
            interrupt_active: self.is_register_bit_flag_high(Registers::INT_SOURCE, Bitmasks::IA)?,
            /// Has low differential pressure event been generated? (self clearing)
            diff_press_low: self.is_register_bit_flag_high(Registers::INT_SOURCE, Bitmasks::PL)?,
            /// Has high differential pressure event been generated? (self clearing)
            diff_press_high: self.is_register_bit_flag_high(Registers::INT_SOURCE, Bitmasks::PH)?,
        };
        Ok(status)
    }

    // --- THESE FUNCTIONS COULD BE REMOVED ---

    /*

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

     */
}
