// reads correctly in one shot mode
// reads correctly in continuous mode



#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, 
    prelude::*, 
    rcc::{Config},    
    serial,
    };

use lps22hb::*;

use lps22hb::interface::{I2cInterface,
    i2c::I2cAddress};

use core::fmt::Write;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    //configure the clock
    let mut rcc = dp.RCC.freeze(Config::hsi16());
    
    //acquire the GPIOA and GPIOB peripherals
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut gpiob = dp.GPIOB.split(&mut rcc);
  
    //get the delay provider
    let mut delay = cp.SYST.delay(rcc.clocks);

    //configure PB3 as output (green LED)
    let mut green = gpiob.pb3.into_push_pull_output(); // SPI CLOCK!

    // configure UART TX/RX pins
    let tx_pin = gpioa.pa2;
    let rx_pin = gpioa.pa3;
    
    // configure serial (default 9600 bps)
    let mut serial = dp.USART2.usart(tx_pin, rx_pin, serial::Config::default().baudrate(9600.Bd()), &mut rcc).unwrap();

    let (mut tx, mut _rx) = serial.split();

    // I2C pins
    let scl = gpioa.pa9.into_open_drain_output();
    let sda = gpioa.pa10.into_open_drain_output();
    
    // I2C configuration
    let i2c = dp.I2C1.i2c(sda, scl, 100_000.Hz(), &mut rcc); 
    
    let mut val: u8 = 0;

    // configure I2C interface for the LPS25HB driver
    let i2c_interface = I2cInterface::init(i2c, I2cAddress::SA0_VCC); // Pololu board

    // create a new driver instance with the I2C interface    
    let mut lps22hb = LPS22HB::new(i2c_interface);
    
    // set data rate to 1Hz
    //lps22hb.set_datarate(ODR::_1Hz).unwrap();

    loop {
        

        lps22hb.one_shot().unwrap();
        
        // read temperature and pressure
        
        let temp = lps22hb.read_temperature().unwrap();                    
        let press = lps22hb.read_pressure().unwrap();
        let id = lps22hb.get_device_id().unwrap();

        // print data to serial
        writeln!(tx, "temperature: {:.1} C, pressure: {:.1} hPa\r", temp, press).unwrap();
        //writeln!(tx, "temperature: {:.1} C\r", temp).unwrap();
        writeln!(tx, "my name is: {}\r", id).unwrap();
 
        green.set_high().unwrap();    
        delay.delay_ms(250_u16);

        // writeln!(tx, "blink! {}\r", val).unwrap();

        val += 1;

        green.set_low().unwrap();
    
        delay.delay_ms(500_u16);
    }


}



