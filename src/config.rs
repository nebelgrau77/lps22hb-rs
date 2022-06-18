//! Various functions related to configuration
//!
//! TO DO: check if all the functions are implemented

use super::*;

impl<T, E> LPS22HB<T>
where
    T: Interface<Error = E>,
{
    /// Set output data rate        
    pub fn set_datarate(&mut self, odr: Odr) -> Result<(), T::Error> {
        let mut reg_data = [0u8];
        self.interface
            .read(Registers::CTRL_REG1.addr(), &mut reg_data)?;
        let mut payload = reg_data[0];
        payload &= !Bitmasks::ODR_MASK;
        payload |= odr.value();
        self.interface.write(Registers::CTRL_REG1.addr(), payload)?;
        Ok(())
    }

    // === TO BE REFACTORED - use Flag ===

    /// Enable or disable block data update
    pub fn bdu_enable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::BDU),
            false => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::BDU),
        }
    }

    // === TO BE REFACTORED - use Flag ===

    /// AUTOZERO: when set to ‘1’, the measured pressure is used
    /// as the reference in REF_P (0x15, 0x16, 0x17).
    /// From that point on the output pressure registers are updated and the same value
    /// is also used for interrupt generation.
    /// The register content of REF_P is subtracted from the measured pressure.
    /// PRESS_OUT = measured pressure - REF_P
    /// P_DIFF_IN = measured pressure - REF_P    ///     
    pub fn autozero_config(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::AUTOZERO),
            false => self.clear_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::AUTOZERO),
        }
    }

    /// Resets the Autozero function. Self-cleared.
    pub fn autozero_reset(&mut self) -> Result<(), T::Error> {
        self.set_register_bit_flag(Registers::INTERRUPT_CFG, Bitmasks::RESET_AZ)
    }

    // === TO BE REFACTORED - use Flag ===

    /// Disables I2C interface (default 0, I2C enabled)
    pub fn i2c_disable(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::I2C_DIS),
            false => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::I2C_DIS),
        }
    }

    // === TO BE REFACTORED - use .value() ===

    /// Sets SPI Mode (default 4-wire)
    pub fn spi_config(&mut self, mode: SPIMode) -> Result<(), T::Error> {
        match mode {
            SPIMode::_3wire => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::SIM),
            SPIMode::_4wire => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::SIM),
        }
    }

    // === TO BE REFACTORED - use Flag ===

    /// Register address automatically incremented during a multiple byte access with a serial interface (I2C or SPI).
    /// Default value: enabled
    pub fn address_incrementing(&mut self, flag: bool) -> Result<(), T::Error> {
        match flag {
            true => self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::IF_ADD_INC),
            false => self.clear_register_bit_flag(Registers::CTRL_REG2, Bitmasks::IF_ADD_INC),
        }
    }

    /// Reboot. Refreshes the content of the internal registers stored in the Flash memory block.
    /// At device power-up the content of the Flash memory block is transferred to the internal registers
    /// related to the trimming functions to allow correct behavior of the device itself.
    /// If for any reason the content of the trimming registers is modified,
    /// it is sufficient to use this bit to restore the correct values.
    /// At the end of the boot process the BOOT bit is set again to ‘0’ by hardware.
    /// The BOOT bit takes effect after one ODR clock cycle.
    pub fn reboot(&mut self) -> Result<(), T::Error> {
        self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::BOOT)
    }

    /// Is reboot phase running?
    pub fn reboot_running(&mut self) -> Result<bool, T::Error> {
        self.is_register_bit_flag_high(Registers::INT_SOURCE, Bitmasks::BOOT_STATUS)
    }

    /// Run software reset (resets the device to the power-on configuration, takes 4 usec)
    pub fn software_reset(&mut self) -> Result<(), T::Error> {
        self.set_register_bit_flag(Registers::CTRL_REG2, Bitmasks::SWRESET)
    }

    // SWITCHING INTO POWER-DOWN COULD BE ADDED TO THIS FUNCTION
    /// Enable low-power mode (must be done only with the device in power-down mode)
    pub fn enable_low_power(&mut self) -> Result<(), T::Error> {
        self.set_register_bit_flag(Registers::RES_CONF, Bitmasks::LC_EN)
    }

    // LOWPASS FILTER ENABLING AND CONFIGURING COULD BE MOVED TOGETHER

    // === TO BE REFACTORED - use Flag for enable, maybe some enum for filter configure? ===

    /// Enable and configure low-pass filter on pressure data in Continuous mode
    pub fn lowpass_filter(&mut self, enable: bool, configure: bool) -> Result<(), T::Error> {
        match enable {
            true => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::EN_LPFP),
            false => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::EN_LPFP),
        }?;
        match configure {
            true => self.set_register_bit_flag(Registers::CTRL_REG1, Bitmasks::LPFP_CFG),
            false => self.clear_register_bit_flag(Registers::CTRL_REG1, Bitmasks::LPFP_CFG),
        }?;
        Ok(())
    }

    // === TO BE REFACTORED - Ok(self.read_register) ===

    /// Reset low-pass filter.  If the LPFP is active, in order to avoid the transitory phase,
    /// the filter can be reset by reading this register before generating pressure measurements.
    pub fn lowpass_filter_reset(&mut self) -> Result<(), T::Error> {
        let mut _data = [0u8; 1];
        self.interface
            .read(Registers::LPFP_RES.addr(), &mut _data)?;
        Ok(())
    }
}
