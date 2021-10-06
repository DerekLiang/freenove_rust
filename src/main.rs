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

/// return true if the bit is set for a given u8
/// bit position start from right to left and staring from 0 to 7
#[inline]
fn is_bit_set(bit: u8, bit_pos: u8) -> bool {
    1 << bit_pos & bit > 0
}

/// set the q_bits to controller
/// 0-7 bits of q_bit representing Q0-Q7 output
fn shift_out(data_pin: &mut OutputPin, clock_pin: &mut OutputPin, q_bits: u8) {
    (0..8).into_iter().rev().for_each(|i| {
        clock_pin.set_low();
        if is_bit_set(q_bits, i) {
            data_pin.set_high();
        } else {
            data_pin.set_low();
        }
        thread::sleep(Duration::from_millis(10));
        clock_pin.set_high();
        thread::sleep(Duration::from_millis(10));
    })
}
/// display x as single digits
fn set_led(
    latch_pin: &mut OutputPin,
    data_pin: &mut OutputPin,
    clock_pin: &mut OutputPin,
    x: usize,
) {
    let numbers = vec![
        0xc0, 0xf9, 0xa4, 0xb0, 0x99, 0x92, 0x82, 0xf8, 0x80, 0x90, 0x88, 0x83, 0xc6, 0xa1, 0x86,
        0x8e,
    ];

    latch_pin.set_low();
    shift_out(data_pin, clock_pin, numbers[x]);
    latch_pin.set_high();
    println!("{}", x);
    thread::sleep(Duration::from_millis(200));
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut data_pin = Gpio::new()?.get(17)?.into_output();
    let mut latch_pin = Gpio::new()?.get(27)?.into_output();
    let mut clock_pin = Gpio::new()?.get(22)?.into_output();

    loop {
        (0..10).into_iter().for_each(|x| {
            set_led(&mut latch_pin, &mut data_pin, &mut clock_pin, x);
        });
        thread::sleep(Duration::from_millis(100));
    }
}
