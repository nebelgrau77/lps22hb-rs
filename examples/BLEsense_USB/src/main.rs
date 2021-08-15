// Example for Arduino 33 BLE Sense with built-in LPS22HB sensor. 
//
// Reads pressure and temperature every second in single shot mode, 
// prints the readings to serial.
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
        uarte::{Uarte,Parity,Baudrate}, 
        clocks::Clocks,
        usbd::{UsbPeripheral, Usbd}        
        };

use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
use usb_device::test_class::TestClass;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
        

use cortex_m_rt::entry;

use arrayvec::ArrayString;
use core::fmt;
use core::fmt::Write;

use lps22hb::interface::{I2cInterface,
                        i2c::I2cAddress};
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
    
    // set up delay provider
    let mut delay = Delay::new(core.SYST);
   
    let usb_bus = Usbd::new(UsbPeripheral::new(p.USBD, &clocks));

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
                                        .manufacturer("Fake company")
                                        .product("Serial port")
                                        .serial_number("TEST")
                                        .device_class(USB_CLASS_CDC)
                                        .max_packet_size_0(64) // makes control transfers 8x faster
                                        .build();



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


    /*
    // define pins for UART
    // using A6 for CTS and A7 for RTS
    let rx_pin = port1.p1_10.into_floating_input().degrade();
    let tx_pin = port1.p1_03.into_push_pull_output(Level::Low).degrade();
    let ct_pin = port0.p0_28.into_floating_input().degrade(); // CTS: not used but necessary for configuration, pin may vary
    let rt_pin = port0.p0_03.into_push_pull_output(Level::Low).degrade(); // RTS: not used but necessary for configuration, pin may vary
        
    let uart_pins = hal::uarte::Pins{
            rxd: rx_pin,
            txd: tx_pin,
            cts: Some(ct_pin),
            rts: Some(rt_pin),
            };
    
    // set up UART
    let mut serial = Uarte::new(p.UARTE0, uart_pins, Parity::EXCLUDED, Baudrate::BAUD9600);
    */
    // configure I2C interface for the LPS22HB driver
    let i2c_interface = I2cInterface::init(i2c1, I2cAddress::SA0_GND);
       
    // create a new driver instance with the I2C interface    
    let mut lps22 = LPS22HB::new(i2c_interface);

    lps22.set_datarate(ODR::_50Hz).unwrap();


    loop {       

        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut text_buf = ArrayString::<[u8; 32]>::new();

        //lps22.enable_one_shot().unwrap();

        let mut buf = ArrayString::<[u8; 32]>::new();

        //let temp = lps22.read_temperature().unwrap();            
        //let press = lps22.read_pressure().unwrap();
        let id = lps22.get_device_id().unwrap();

        //format_reading(&mut buf, press, temp);
        format_simple(&mut buf, id);

        //serial.write_str(buf.as_str()).unwrap();

        serial.write(buf.as_bytes());

        // toggle the LED
        if led.is_set_high().unwrap() {
            led.set_low().unwrap();
            }
        else {
            led.set_high().unwrap();
            }

        //delay.delay_ms(1000_u32);
    }    
}


/// Simple formatter to pretty print the sensor values
fn format_simple(buf: &mut ArrayString<[u8; 32]>, sensor_id: u8) {
    fmt::write(buf, format_args!("my name is: {}\r\n", sensor_id)).unwrap();
}


/// Simple formatter to pretty print the sensor values
fn format_reading(buf: &mut ArrayString<[u8; 32]>, press: f32, temp: f32) {
    fmt::write(buf, format_args!("P: {:.02}hPA, T: {:.02}C\r\n", press, temp)).unwrap();
}