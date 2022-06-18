//! Various functions related to interrupts
//!
//! TO DO: check if all functions related to the interrupts are covered

use super::*;

/// Interrupt pin settings
#[derive(Debug)]
pub struct InterruptConfig {
    /// configure interrupt pin as active high or active low
    pub active_high_or_low: IntActive,
    /// configure interrupt pin as  push-pull or open drain
    pub pushpull_or_opendrain: IntPin,
    /// configure data signal on the interrupt pin
    pub data_signal_config: IntDrdy,
    /// enable FIFO full flag on interrupt pin
    pub enable_fifo_full: Flag,
    /// enable FIFO watermark flag on interrupt pin
    pub enable_fifo_fth: Flag,
    /// enable FIFO overrun flag on interrupt pin
    pub enable_fifo_overrun: Flag,
    /// enable data ready signal on interrupt pin
    pub enable_data_ready: Flag,
    /// enable latching interrupt request to INT_SOURCE register
    pub enable_latch_interrupt: Flag,
    /// enable low pressure event on interrupt pin
    pub enable_low_event: Flag,
    /// enable hihg pressure event on interrupt pin
    pub enable_high_event: Flag,
    /// enable computing of differential pressure output
    pub enable_differential: Flag,
}

impl Default for InterruptConfig {
    fn default() -> Self {
        InterruptConfig {
            active_high_or_low: IntActive::High, // active high (CTRL_REG3)
            pushpull_or_opendrain: IntPin::PushPull, // push-pull (CTRL_REG3)
            data_signal_config: IntDrdy::DataSignal, // data signal on INT_DRDY pin (CTRL_REG3)
            enable_fifo_full: Flag::Disabled,    // disabled (CTRL_REG3)
            enable_fifo_fth: Flag::Disabled,     // disabled (CTRL_REG3)
            enable_fifo_overrun: Flag::Disabled, // disabled (CTRL_REG3)
            enable_data_ready: Flag::Disabled,   // disabled (CTRL_REG3)
            enable_latch_interrupt: Flag::Disabled, // interrupt request not latched (INTERRUPT_CFG)
            enable_low_event: Flag::Disabled, // disable interrupt request on low pressure event (INTERRUPT_CFG)
            enable_high_event: Flag::Disabled, // disable interrupt request on low pressure event (INTERRUPT_CFG)
            enable_differential: Flag::Disabled, // disabled (CTRL_REG1)
        }
    }
}

impl InterruptConfig {
    /// Returns values to be written to CTRL_REG3, CTRL_REG4 and INTERRUPT_CFG:
    fn int_ctrl_reg3(&self) -> u8 {
        let mut data = 0u8;
        data |= self.active_high_or_low.value() << 7;
        data |= self.pushpull_or_opendrain.value() << 6;
        data |= self.enable_fifo_full.value() << 5;
        data |= self.enable_fifo_fth.value() << 4;
        data |= self.enable_fifo_overrun.value() << 3;
        data |= self.enable_data_ready.value() << 2;
        data |= self.data_signal_config.value();
        data
    }
    fn int_interrupt_cfg(&self) -> u8 {
        let mut data = 0u8;

        data |= self.enable_differential.value() << 3;
        data |= self.enable_latch_interrupt.value() << 2;
        data |= self.enable_low_event.value() << 1;
        data |= self.enable_high_event.value();
        data // this must be OR'ed with the content of the INTERRUPT_CFG
    }
}

#[derive(Debug)]
/// Contents of the INT_SOURCE register (interrupt active and differential pressure events flags)
pub struct IntStatus {
    pub interrupt_active: bool,
    pub diff_press_low: bool,
    pub diff_press_high: bool,
}

impl From<u8> for IntStatus {
    fn from(value: u8) -> Self {
        IntStatus {
            interrupt_active: match (value & Bitmasks::IA) >> 2 {
                0 => false,
                _ => true,
            },
            diff_press_low: match (value & Bitmasks::PL) >> 1 {
                0 => false,
                _ => true,
            },
            diff_press_high: match value & Bitmasks::PH {
                0 => false,
                _ => true,
            },
        }
    }
}

impl<T, E> LPS22HB<T>
where
    T: Interface<Error = E>,
{
    /// Enable interrupts and configure the interrupt pin
    pub fn configure_interrupts(
        &mut self,
        // flag: bool,
        config: InterruptConfig,
    ) -> Result<(), T::Error> {
        // write the whole CTRL_REG3 register
        self.interface
            .write(Registers::CTRL_REG3.addr(), config.int_ctrl_reg3())?;

        // get the contents of INTERRUPT_CFG and combine it with the bits to be set
        let reg_data = [0u8; 1];
        self.read_register(Registers::INTERRUPT_CFG)?;

        let interrupt_cfg = config.int_interrupt_cfg();

        let mut data: u8 = reg_data[0] & !0b00001111;

        data = data | interrupt_cfg;

        self.interface
            .write(Registers::INTERRUPT_CFG.addr(), data)?;

        Ok(())
    }

    /// Get all the flags from the INT_SOURCE register (NOTE: INT_SOURCE register is cleared by reading it)
    pub fn get_int_status(&mut self) -> Result<IntStatus, T::Error> {
        Ok(IntStatus::from(self.read_register(Registers::INT_SOURCE)?))
    }
}
