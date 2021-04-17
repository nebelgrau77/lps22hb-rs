//! Register mapping

/// LPS22HB Registers
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Registers {
    
    /// Interrupt control.
    INTERRUPT_CFG   = 0x0B,
    /// Pressure threshold low.
    THS_P_L         = 0x0C,
    /// Pressure threshold high.
    THS_P_H         = 0x0D,
    /// Who Am I (identifies the chip).
    WHO_AM_I        = 0x0F,    
    /// Control register 1.
    CTRL_REG1       = 0x10,
    /// Control register 2.
    CTRL_REG2       = 0x11,
    /// Control register 3.
    CTRL_REG3       = 0x12,
    /// FIFO configuration register.
    FIFO_CTRL       = 0x14,
    /// Reference pressure register.
    REF_P_XL        = 0x15,
    /// Reference pressure register.
    REF_P_L         = 0x16,
    /// Reference pressure register.
    REF_P_H         = 0x17,
    /// Pressure offset register.
    RPDS_L          = 0x18,
    /// Pressure offset register.
    RPDS_H          = 0x19,
    /// Resolution configuration.
    RES_CONF        = 0x1A,
    /// Interrupt configuration.
    INT_SOURCE      = 0x25,
    /// FIFO status register.
    FIFO_STATUS     = 0x26,
    /// Status register.
    STATUS          = 0x27,
    /// Pressure output register.
    PRESS_OUT_XL    = 0x28,
    /// Pressure output register.
    PRESS_OUT_L     = 0x29,
    /// Pressure output register.
    PRESS_OUT_H     = 0x2A,
    /// Temperature output register.
    TEMP_OUT_L      = 0x2B,
    /// Temperature output register.
    TEMP_OUT_H      = 0x2C,
    /// Filter reset register.
    LPFP_RES        = 0x33,    
    
}

impl Registers {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

/// LPS22HB Bit masks
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Bitmasks {
    
    /// Enable single shot to acquire a new dataset
    ONE_SHOT    = 0b0000_0001,
}

impl Bitmasks {
    pub fn bitmask(self) -> u8 {
        self as u8
    }
}