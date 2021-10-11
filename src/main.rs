extern crate rppal;

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use rppal::gpio::Gpio;
use rppal::gpio::InputPin;
use rppal::gpio::OutputPin;
use std::ptr::read_volatile;
use std::ptr::write_volatile;
use std::thread;
use std::time::Duration;

use itertools::Itertools;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {

    let mut led_pin = Gpio::new()?.get(18)?.into_output();
    let sensor_pin = Gpio::new()?.get(17)?.into_input();

    loop {
        if sensor_pin.is_high() {
            println!("turn on LED");
            led_pin.set_high();
        } else {
            println!("turn off LED");
            led_pin.set_low();
        }

        thread::sleep(Duration::from_secs(1));
    }
}
