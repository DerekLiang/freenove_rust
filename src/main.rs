use core::f32::consts::PI;
use rand::Rng;
use rppal::gpio::OutputPin;
use rppal::i2c::I2c;
use std::error::Error;
use std::slice::Iter;
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
        thread::sleep(Duration::from_micros(1));
        clock_pin.set_high();
        thread::sleep(Duration::from_micros(1));
    })
}
/// display x as single digit number
/// if x is out of range (0-9), it will clear the display
fn set_led(
    latch_pin: &mut OutputPin,
    data_pin: &mut OutputPin,
    clock_pin: &mut OutputPin,
    x: usize,
) {
    let numbers = vec![0xc0, 0xf9, 0xa4, 0xb0, 0x99, 0x92, 0x82, 0xf8, 0x80, 0x90];

    latch_pin.set_low();
    shift_out(data_pin, clock_pin, if x <= 9 { numbers[x] } else { 0xff });
    latch_pin.set_high();
    // println!("{}", x);
    thread::sleep(Duration::from_millis(1));
}

/// convert number into digit array representation,
/// for example 123 convert to [0,1,2,3]
fn get_digits(number: u16) -> Vec<u8> {
    let mut number = number;
    let mut result = vec![];

    while number > 0 {
        result.push((number % 10) as u8);
        number /= 10;
    }

    (0..4).into_iter().for_each(|_| {
        result.push(0);
    });

    result.into_iter().take(4).rev().collect()
}

fn display(
    digit_pins: &mut Vec<OutputPin>,
    latch_pin: &mut OutputPin,
    data_pin: &mut OutputPin,
    clock_pin: &mut OutputPin,
    number: u16,
) {
    // println!("\n@@@ start print {}", number);
    get_digits(number)
        .into_iter()
        .enumerate()
        .for_each(|(d_pos, d)| {
            set_led(latch_pin, data_pin, clock_pin, 0xff); // clear led
            // println!("--- digit {} at {}", d, d_pos);
            digit_pins
                .into_iter()
                .enumerate()
                .for_each(|(pin_pos, pin)| {
                    // Note: the ref implementation do the opposite, 
                    // in my hardware, it seems that the following is correct.
                    if d_pos == pin_pos { 
                        pin.set_low();
                        // println!("setting pin {} low for {}", pin_pos, d);
                    } else {
                        pin.set_high();
                        // println!("setting pin {} high for {}", pin_pos, d);
                    }
                });
            set_led(latch_pin, data_pin, clock_pin, d as usize);
            thread::sleep(Duration::from_millis(1));
        })
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut data_pin = Gpio::new()?.get(24)?.into_output();
    let mut latch_pin = Gpio::new()?.get(23)?.into_output();
    let mut clock_pin = Gpio::new()?.get(18)?.into_output();

    let mut digit_pins = vec![
        Gpio::new()?.get(17)?.into_output(),
        Gpio::new()?.get(27)?.into_output(),
        Gpio::new()?.get(22)?.into_output(),
        Gpio::new()?.get(10)?.into_output(),
    ];

    let counter = Arc::new(Mutex::<u16>::new(0));
    let counter_ref = counter.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000));
            *counter_ref.lock().unwrap() += 1;      
            *counter_ref.lock().unwrap() %= 10000;  // prevent overflow u16
        }
    });

    loop {
        (0..u16::MAX).for_each(|d| {
            display(
                &mut digit_pins,
                &mut latch_pin,
                &mut data_pin,
                &mut clock_pin,
                // d,
                *counter.lock().unwrap(),
            );
            // thread::sleep(Duration::from_millis(1));
        });
        break;
    }

    Ok(())
}
