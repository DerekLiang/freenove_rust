use core::f32::consts::PI;
use rand::Rng;
use rppal::gpio::OutputPin;
use rppal::i2c::I2c;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::Trigger;
use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::system::DeviceInfo;

// Q0-Q7 q_bit representing 0-7 bit
fn shift_out(data_pin: &mut OutputPin, clock_pin: &mut OutputPin, q_bit: u8) {
    (0..8).into_iter().rev().for_each(|i| {
        clock_pin.set_low();
        if i == q_bit {
            data_pin.set_high();
        } else {
            data_pin.set_low();
        }
        thread::sleep(Duration::from_millis(10));
        clock_pin.set_high();
        thread::sleep(Duration::from_millis(10));
    })
}

fn set_led(latch_pin: &mut OutputPin, data_pin: &mut OutputPin, clock_pin: &mut OutputPin, x: u8) {
    latch_pin.set_low();
    shift_out(data_pin, clock_pin, x);
    latch_pin.set_high();
    println!("{}",x);
    thread::sleep(Duration::from_millis(2000));
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut data_pin = Gpio::new()?.get(17)?.into_output();
    let mut latch_pin = Gpio::new()?.get(27)?.into_output();
    let mut clock_pin = Gpio::new()?.get(22)?.into_output();
    
    loop {
        (0..8).into_iter().for_each(|x| {
            set_led(&mut latch_pin,&mut data_pin,&mut clock_pin, x);
        });
        thread::sleep(Duration::from_millis(100));

        (0..8).into_iter().rev().for_each(|x| {
            set_led(&mut latch_pin,&mut data_pin,&mut clock_pin, x);
        });

        thread::sleep(Duration::from_millis(100));
    }
}
