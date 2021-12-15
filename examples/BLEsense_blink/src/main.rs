// Example for Arduino 33 BLE Sense with built-in LPS22HB sensor. 
//
// Reads pressure and temperature, 
// prints the readings to serial over USB.
//
// this seems to be working (@115200 bps)

#![no_main]
#![no_std]

use panic_halt as _;

use nrf52840_hal as hal;

use hal::{pac::{CorePeripherals, Peripherals},
        prelude::*,
        gpio::Level,
        delay::Delay,        
        Twim,        
        clocks::Clocks,
        };

use cortex_m_rt::entry;

use lps22hb::{interface::{I2cInterface,
                        i2c::I2cAddress}, interrupt::InterruptConfig, fifo::FIFOConfig};
use lps22hb::*;

const BOOT_DELAY_MS: u16 = 100; //small delay for the I2C to initiate correctly and start on boot without having to reset the board

#[entry]
fn main() -> ! {
    
    let p = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    let clocks = Clocks::new(p.CLOCK);
    let clocks = clocks.enable_ext_hfosc();

    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let port1 = hal::gpio::p1::Parts::new(p.P1);
    
    let mut led = port0.p0_13.into_push_pull_output(Level::Low);
    
    let _vdd_env = port0.p0_22.into_push_pull_output(Level::High); // powers the LPS22HB sensor, as per board schematics
    
    let _r_pullup = port1.p1_00.into_push_pull_output(Level::High); // necessary for SDA1 and SCL1 to work, as per board schematics
    
    let mut red = port0.p0_24.into_push_pull_output(Level::High);
    let mut green = port0.p0_16.into_push_pull_output(Level::High);    
    let mut blue = port0.p0_06.into_push_pull_output(Level::High);

    // set up delay provider
    let mut delay = Delay::new(core.SYST);
   
    // define I2C1 pins
    let scl1 = port0.p0_15.into_floating_input().degrade(); // clock
    let sda1 = port0.p0_14.into_floating_input().degrade(); // data

    let i2c1_pins = hal::twim::Pins{
        scl: scl1,
        sda: sda1
    };    

    // wait for just a moment
    delay.delay_ms(BOOT_DELAY_MS);
    
    // set up I2C1    
    let mut i2c1 = Twim::new(p.TWIM1, i2c1_pins, hal::twim::Frequency::K400);
    
    delay.delay_ms(1000_u32);

    led.set_high().unwrap();

    // configure I2C interface for the LPS22HB driver
    let i2c_interface = I2cInterface::init(i2c1, I2cAddress::SA0_GND);
       
    // create a new driver instance with the I2C interface    
    let mut lps22 = LPS22HB::new(i2c_interface);

    lps22.set_datarate(ODR::_1Hz).unwrap();

    let int_config = InterruptConfig{enable_low_event: true,
                                    data_signal_config: INT_DRDY::P_low,
                                ..Default::default()};

    lps22.enable_interrupts(true, int_config).unwrap();

    let fifo_config = FIFOConfig{enable_watermark: true,
        fifo_mode: FIFO_MODE::FIFO, 
        watermark_level: 8,
        ..Default::default()};

    //lps22.enable_fifo(true, fifo_config).unwrap();


    loop {       

        let int_status = lps22.get_int_status().unwrap();

        // toggle the LED
        if int_status.diff_press_low {
            if red.is_set_high().unwrap() {
                red.set_low().unwrap();
            } else {
                red.set_high().unwrap();
            }
        }


        let fifo_status = lps22.get_fifo_status().unwrap();

        // toggle the LED
        if fifo_status.fifo_thresh_reached {
            green.set_low().unwrap();
            }
        else {
            green.set_high().unwrap();
            }

        let data_status = lps22.get_data_status().unwrap();
        
        // toggle the LED
        if data_status.press_available {
            if blue.is_set_high().unwrap() {
                blue.set_low().unwrap();
                delay.delay_ms(50_u32);
            } else {
                blue.set_high().unwrap();
                delay.delay_ms(50_u32);
            }
        }

        /*
        // toggle the LED
        if led.is_set_high().unwrap() {
            led.set_low().unwrap();
        } else {
            led.set_high().unwrap();
        }
         */
    }    
}

