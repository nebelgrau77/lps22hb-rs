//! Pressure sensor settings, types
#![allow(dead_code, non_camel_case_types)]

/// Pressure sensor settings. Use this struct to configure the sensor.
#[derive(Debug)]
pub struct SensorSettings {    
    /// Output data rate & power mode selection
    pub sample_rate: ODR,    
    pub auto_addr_inc: bool,

}

impl Default for SensorSettings {
    fn default() -> Self {
        SensorSettings {            
            sample_rate: ODR::PowerDown,            
            auto_addr_inc: true,
        }
    }
}

impl SensorSettings {
    
    /// Returns `u8` to write to CTRL_REG1 (0x10)
    /// # CTRL_REG1: [0][ODR2][ODR1][ODR0][EN_LPFP][LPFP_CFG][BDU][SIM]
    /// - ODR[2:0] - Output data rate & power mode selection
    /// - EN_LPFP - Enable low-pass filter on pressure data when Continuous mode is used
    /// - LPFP_CFG - Low-pass configuration register
    /// - BDU - Block data update
    /// - SIM - SPI Serial Interface Mode selection
    
    pub fn ctrl_reg1(&self) -> u8 {
        self.sample_rate.value()
    }
    
    /// Returns `u8` to write to CTRL_REG2 (0x11)
    /// # CTRL_REG2: [BOOT][FIFO_EN][STOP_ON_FTH][IF_ADD_INC][I2C_DIS][SWRESET][ONE_SHOT]    
    /// - BOOT - Reboot memory content
    /// - FIFO_EN - FIFO enable
    /// - STOP_ON_FTH - Stop on FIFO watermark. Enable FIFO watermark level use
    /// - IF_ADD_INC - Register address automatically incremented during a multiple byte access with a
    /// serial interface (I2C or SPI)
    /// - I2C_DIS - Disable I2C interface
    /// - SWRESET - Software reset
    /// - ONE_SHOT - One-shot enable    
    pub fn ctrl_reg2(&self) -> u8 {
        let mut result = 0_u8;
        if self.auto_addr_inc {
            result |= 1 << 4;
        }
        result        
    }
}

/// Output data rate and power mode selection (ODR_XL). (Refer to Table 68)
#[derive(Debug, Clone, Copy)]
pub enum ODR {
    /// Power-down / One-shot mode enabled
    PowerDown = 0b000,
    /// 1 Hz
    _1Hz = 0b001,
    /// 10 Hz
    _10Hz = 0b010,
    /// 25 Hz
    _25Hz = 0b011,
    /// 50 Hz
    _50Hz = 0b100,
    /// 75 Hz
    _75Hz = 0b101,    
}

impl ODR {
    pub fn value(self) -> u8 {
        (self as u8) << 4
    }
}



#[test]
fn sensor_init_values() {
    let settings = SensorSettings::default();
    //assert_eq!(settings.ctrl_reg5_xl(), 0b0011_1000); // [DEC_1][DEC_0][Zen_XL][Yen_XL][Zen_XL][0][0][0]
    //assert_eq!(settings.ctrl_reg6_xl(), 0b0110_0000); // [ODR_XL2][ODR_XL1][ODR_XL0][FS1_XL][FS0_XL][BW_SCAL_ODR][BW_XL1][BW_XL0]
    //assert_eq!(settings.ctrl_reg7_xl(), 0b0000_0000); // [HR][DCF1][DCF0][0][0][FDS][0][HPIS1]
}
