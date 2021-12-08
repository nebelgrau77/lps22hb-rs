//! Register mapping and bitmasks
//!

/// LPS22HB Registers
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Registers {
    /// Interrupt control.
    INTERRUPT_CFG = 0x0B,
    /// Pressure threshold low.
    THS_P_L = 0x0C,
    /// Pressure threshold high.
    THS_P_H = 0x0D,
    /// Who Am I (identifies the chip).
    WHO_AM_I = 0x0F,
    /// Control register 1.
    CTRL_REG1 = 0x10,
    /// Control register 2.
    CTRL_REG2 = 0x11,
    /// Control register 3.
    CTRL_REG3 = 0x12,
    /// FIFO configuration register.
    FIFO_CTRL = 0x14,
    /// Reference pressure register.
    REF_P_XL = 0x15,
    /// Reference pressure register.
    REF_P_L = 0x16,
    /// Reference pressure register.
    REF_P_H = 0x17,
    /// Pressure offset register.
    RPDS_L = 0x18,
    /// Pressure offset register.
    RPDS_H = 0x19,
    /// Resolution configuration.
    RES_CONF = 0x1A,
    /// Interrupt configuration.
    INT_SOURCE = 0x25,
    /// FIFO status register.
    FIFO_STATUS = 0x26,
    /// Status register.
    STATUS = 0x27,
    /// Pressure output register.
    PRESS_OUT_XL = 0x28,
    /// Pressure output register.
    PRESS_OUT_L = 0x29,
    /// Pressure output register.
    PRESS_OUT_H = 0x2A,
    /// Temperature output register.
    TEMP_OUT_L = 0x2B,
    /// Temperature output register.
    TEMP_OUT_H = 0x2C,
    /// Filter reset register. If the LPFP is active, in order to avoid the transitory phase, 
    /// the filter can be reset by reading this register before generating pressure measurements.
    LPFP_RES = 0x33,
}

impl Registers {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

/// LPS22HB Bit masks
#[allow(non_camel_case_types)]
pub struct Bitmasks;

impl Bitmasks {
    // === INTERRUPT_CFG (0x0B) ===
    pub const AUTORIFP: u8 = 0b1000_0000;
    pub const RESET_ARP: u8 = 0b0100_0000;
    pub const AUTOZERO: u8 = 0b0010_0000;
    pub const RESET_AZ: u8 = 0b0001_0000;
    /// Enable interrupt generation
    pub const DIFF_EN: u8 = 0b0000_1000;
    /// Latch Interrupt Request
    pub const LIR: u8 = 0b0000_0100;
    /// Enable interrupt generation on Low Pressure Event
    pub const PLE: u8 = 0b0000_0010;
    /// Enable interrupt generation on High Pressure Event
    pub const PHE: u8 = 0b0000_0001;

    // === CTRL_REG1 (0x10) ===
    /// Output data rate selection
    pub const ODR_MASK: u8 = 0b0111_0000;
    /// Low pass filter on pressure data in Continuous mode
    pub const EN_LPFP: u8 = 0b0000_1000;
    pub const LPFP_CFG: u8 = 0b0000_0100;
    /// Block data update
    pub const BDU: u8 = 0b0000_0010;
    /// SPI Interface Mode Selection
    pub const SIM: u8 = 0b0000_0001;

    // === CTRL_REG2 (0x11) ===
    pub const BOOT: u8 = 0b1000_0000;
    pub const FIFO_EN: u8 = 0b0100_0000;
    pub const STOP_ON_FTH: u8 = 0b0010_0000;
    /// Increment address during multiple byte read (I2C/SPI), default 1 (enabled)
    pub const IF_ADD_INC: u8 = 0b0001_0000;
    pub const I2C_DIS: u8 = 0b0000_1000;
    pub const SWRESET: u8 = 0b0000_0100;
    /// Enable single shot to acquire a new dataset
    pub const ONE_SHOT: u8 = 0b0000_0001;

    // === CTRL_REG3 (0x12) ===
    pub const INT_H_L: u8 = 0b1000_0000;
    pub const PP_OD: u8 = 0b0100_0000;
    /// FIFO full flag on INT_DRDY pin
    pub const F_FSS5: u8 = 0b0010_0000;
    /// FIFO watermark status on INT_DRDY pin
    pub const F_FTH: u8 = 0b0001_0000;
    /// FIFO watermark status on INT_DRDY pin
    pub const F_OVR: u8 = 0b0000_1000;
    /// Data-ready signal on INT_DRDY pin
    pub const DRDY: u8 = 0b0000_0100;
    /// Data signal on INT_DRDY pin control bits
    pub const INT_S_MASK: u8 = 0b0000_0011;

    // === FIFO_CTRL (0x14) ===
    /// FIFO mode selection
    pub const F_MODE_MASK: u8 = 0b1110_0000;
    /// FIFO watermark level selection
    pub const WTM_MASK: u8 = 0b0001_1111;

    // === RES_CONF (0x1A) ===
    /// Low current mode enable; must be changed in power-down mode
    pub const LC_EN: u8 = 0b0000_0001;

    // === INT_SOURCE (0x25) ===
    /// Reboot phase status (1 - running)
    pub const BOOT_STATUS: u8 = 0b1000_0000;
    /// Interrupt active
    pub const IA: u8 = 0b0000_0100;
    /// Differential pressure low
    pub const PL: u8 = 0b0000_0010;
    /// Differential pressure high
    pub const PH: u8 = 0b0000_0001;

    // === FIFO_STATUS (0x26) ===
    /// FIFO watermark status
    pub const FTH_FIFO: u8 = 0b1000_0000;
    /// FIFO overrun status
    pub const OVR: u8 = 0b0100_0000;
    /// FIFO stored data level
    pub const FSS_MASK: u8 = 0b0011_1111;

    // === STATUS (0x27) ===
    /// Temperature data overrun
    pub const T_OR: u8 = 0b0010_0000;
    /// Pressure data overrun
    pub const P_OR: u8 = 0b0001_0000;
    /// Temperature data available
    pub const T_DA: u8 = 0b0000_0010;
    /// Pressure data available
    pub const P_DA: u8 = 0b0000_0001;
}
