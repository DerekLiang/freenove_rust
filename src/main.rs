extern crate rppal;

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use rppal::gpio::Gpio;
use rppal::gpio::InputPin;
use rppal::gpio::OutputPin;
use std::ptr::read_volatile;
use std::ptr::write_volatile;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use itertools::Itertools;
use rppal::gpio::Level;
use rppal::gpio::Trigger;
use std::error::Error;
use std::time::Instant;

fn pulse_in(echo_pin: &InputPin, timeout: u16) -> f32 {
    0.0
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut trigger_pin = Gpio::new()?.get(23)?.into_output();
    let mut echo_pin = Gpio::new()?.get(24)?.into_input_pulldown();

    let MAX_DISTANCE = 2; // 2 meter
    let timeout_in_micro_second = MAX_DISTANCE * 2 * 1000_000 / 340; // t=S*2/V

    loop {
        // start the plus
        trigger_pin.set_high();
        thread::sleep(Duration::from_micros(10));
        trigger_pin.set_low();

        // wait for rising edge
        echo_pin.set_interrupt(Trigger::RisingEdge)?;
        match echo_pin.poll_interrupt(true, Some(Duration::from_micros(timeout_in_micro_second))) {
            Err(_) | Ok(None) => {
                // println!("timeout on waiting for rising edge");
                continue;
            }
            Ok(_) => {}
        };
        let start_time = Instant::now();

        // wait for falling edge
        echo_pin.set_interrupt(Trigger::FallingEdge)?;
        match echo_pin.poll_interrupt(true, Some(Duration::from_micros(timeout_in_micro_second))) {
            Err(_) | Ok(None) => {
                // println!("timeout on waiting for falling edge");
                continue;
            }
            _ => {}
        }
        let end_time = Instant::now();

        let ping_time = end_time - start_time;

        //calculate distance with sound speed 340m/s
        let distance = ping_time.as_micros() as f32 * 340.0 / 2.0 / 1000_000.0;

        println!(
            "distance is: {:.3} meter, {}micro-second",
            distance,
            ping_time.as_micros()
        );
        thread::sleep(Duration::from_secs(1));
    }
}
