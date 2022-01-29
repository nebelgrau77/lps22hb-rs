// reads correct values in one shot mode, but the one shot must be enabled every time
// all values incorrect in continuous mode


#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, 
    prelude::*, 
    rcc::{Config},
    spi,
    serial,
    };

use lps22hb::*;
use lps22hb::interface::{SpiInterface};
    
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
    // let mut green = gpiob.pb3.into_push_pull_output(); // SPI CLOCK!

    // configure UART TX/RX pins
    let tx_pin = gpioa.pa2;
    let rx_pin = gpioa.pa3;
    
    // configure serial (default 9600 bps)
    let mut serial = dp.USART2.usart(tx_pin, rx_pin, serial::Config::default().baudrate(9600.Bd()), &mut rcc).unwrap();

    let (mut tx, mut _rx) = serial.split();

    // SPI pins
    let sck = gpiob.pb3;
    let miso = gpiob.pb4;
    let mosi = gpiob.pb5;
    let mut cs = gpioa.pa11.into_push_pull_output();

    // initialize SPI
    let mut spi = dp.SPI1.spi((sck, miso, mosi), spi::MODE_0, 100_000.Hz(), &mut rcc);
    
    //let mut val: u8 = 0;

    // configure SPI interface for the LPS25HB driver
    let spi_interface = SpiInterface::init(spi, cs); // Pololu board

    let mut lps22hb = LPS22HB::new(spi_interface);
    
    // set data rate to 1Hz
    // lps25hb.set_datarate(ODR::_1Hz).unwrap();

    loop {
        
        lps22hb.one_shot().unwrap();
        
        // read temperature and pressure
        
        let temp = lps22hb.read_temperature().unwrap();            
        
        
        
        // let press = lps22hb.read_pressure().unwrap();

        

        let id = lps22hb.get_device_id().unwrap();
        // print data to serial
        // writeln!(tx, "temperature: {:.1} C, pressure: {:.1} hPa\r", temp, press).unwrap();
        writeln!(tx, "temperature: {:.1} C\r", temp).unwrap();
        // writeln!(tx, "pressure: {:.1} hPa\r", press).unwrap();
        writeln!(tx, "my name is: {}\r", id).unwrap();

        //green.set_high().unwrap();    
        // delay.delay_ms(250_u16);

        // writeln!(tx, "blink! {}\r", val).unwrap();

        //val += 1;

        //green.set_low().unwrap();
    
        delay.delay_ms(500_u16);
    }


}



