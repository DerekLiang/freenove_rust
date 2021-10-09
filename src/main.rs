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
        // Note: the ref implementation has delays which cause flickering
        // thread::sleep(Duration::from_micros(10));
        clock_pin.set_high();
        // thread::sleep(Duration::from_micros(10));
    })
}

fn set_display(
    latch_pin: &mut OutputPin,
    data_pin: &mut OutputPin,
    clock_pin: &mut OutputPin,
    data: &Vec<u8>,
) {
    (0..8).for_each(|p| {
        latch_pin.set_low();
        // first shift data of line information to the first stage 74HC959
        shift_out(data_pin, clock_pin, data[p]);
        // then shift data of column information to the second stage 74HC959
        shift_out(data_pin, clock_pin, !(0x80 >> p));
        // Output data of two stage 74HC595 at the same time
        latch_pin.set_high();

        thread::sleep(Duration::from_millis(1));
    });

    thread::sleep(Duration::from_millis(1));
}
// /// display x as single digit number
// /// if x is out of range (0-9), it will clear the display
// fn set_led(
//     latch_pin: &mut OutputPin,
//     data_pin: &mut OutputPin,
//     clock_pin: &mut OutputPin,
//     x: usize,
// ) {
//     let numbers = vec![0xc0, 0xf9, 0xa4, 0xb0, 0x99, 0x92, 0x82, 0xf8, 0x80, 0x90];

//     latch_pin.set_low();
//     shift_out(data_pin, clock_pin, if x <= 9 { numbers[x] } else { 0xff });
//     latch_pin.set_high();
//     // println!("{}", x);
//     thread::sleep(Duration::from_millis(1));
// }

// // /// convert number into digit array representation,
// // /// for example 123 convert to [0,1,2,3]
// // fn get_digits(number: u16) -> Vec<u8> {
// //     let mut number = number;
// //     let mut result = vec![];

// //     while number > 0 {
// //         result.push((number % 10) as u8);
// //         number /= 10;
// //     }

// //     (0..4).into_iter().for_each(|_| {
// //         result.push(0);
// //     });

// //     result.into_iter().take(4).rev().collect()
// // }

// // fn display(
// //     digit_pins: &mut Vec<OutputPin>,
// //     latch_pin: &mut OutputPin,
// //     data_pin: &mut OutputPin,
// //     clock_pin: &mut OutputPin,
// //     number: u16,
// // ) {
// //     // println!("\n@@@ start print {}", number);
// //     get_digits(number)
// //         .into_iter()
// //         .enumerate()
// //         .for_each(|(d_pos, d)| {
// //             set_led(latch_pin, data_pin, clock_pin, 0xff); // clear led
// //             // println!("--- digit {} at {}", d, d_pos);
// //             digit_pins
// //                 .into_iter()
// //                 .enumerate()
// //                 .for_each(|(pin_pos, pin)| {
// //                     // Note: the ref implementation do the opposite,
// //                     // in my hardware, it seems that the following is correct.
// //                     if d_pos == pin_pos {
// //                         pin.set_low();
// //                         // println!("setting pin {} low for {}", pin_pos, d);
// //                     } else {
// //                         pin.set_high();
// //                         // println!("setting pin {} high for {}", pin_pos, d);
// //                     }
// //                 });
// //             set_led(latch_pin, data_pin, clock_pin, d as usize);
// //             thread::sleep(Duration::from_millis(1));
// //         })
// // }

fn main() -> Result<(), Box<dyn Error>> {
    let mut data_pin = Gpio::new()?.get(17)?.into_output();
    let mut latch_pin = Gpio::new()?.get(27)?.into_output();
    let mut clock_pin = Gpio::new()?.get(22)?.into_output();

    let smiley_data = vec![0x1c, 0x22, 0x51, 0x45, 0x45, 0x51, 0x22, 0x1c];

    let digits_data = vec![
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // " "
        vec![0x00, 0x00, 0x3E, 0x41, 0x41, 0x3E, 0x00, 0x00], // "0"
        vec![0x00, 0x00, 0x21, 0x7F, 0x01, 0x00, 0x00, 0x00], // "1"
        vec![0x00, 0x00, 0x23, 0x45, 0x49, 0x31, 0x00, 0x00], // "2"
        vec![0x00, 0x00, 0x22, 0x49, 0x49, 0x36, 0x00, 0x00], // "3"
        vec![0x00, 0x00, 0x0E, 0x32, 0x7F, 0x02, 0x00, 0x00], // "4"
        vec![0x00, 0x00, 0x79, 0x49, 0x49, 0x46, 0x00, 0x00], // "5"
        vec![0x00, 0x00, 0x3E, 0x49, 0x49, 0x26, 0x00, 0x00], // "6"
        vec![0x00, 0x00, 0x60, 0x47, 0x48, 0x70, 0x00, 0x00], // "7"
        vec![0x00, 0x00, 0x36, 0x49, 0x49, 0x36, 0x00, 0x00], // "8"
        vec![0x00, 0x00, 0x32, 0x49, 0x49, 0x3E, 0x00, 0x00], // "9"
        vec![0x00, 0x00, 0x3F, 0x44, 0x44, 0x3F, 0x00, 0x00], // "A"
        vec![0x00, 0x00, 0x7F, 0x49, 0x49, 0x36, 0x00, 0x00], // "B"
        vec![0x00, 0x00, 0x3E, 0x41, 0x41, 0x22, 0x00, 0x00], // "C"
        vec![0x00, 0x00, 0x7F, 0x41, 0x41, 0x3E, 0x00, 0x00], // "D"
        vec![0x00, 0x00, 0x7F, 0x49, 0x49, 0x41, 0x00, 0x00], // "E"
        vec![0x00, 0x00, 0x7F, 0x48, 0x48, 0x40, 0x00, 0x00], // "F"
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // " "
    ];


    loop {
        (0..20).for_each(|_| {
            set_display(&mut latch_pin,&mut  data_pin,&mut  clock_pin, &smiley_data);
        });
        
        (0..digits_data.len()).for_each(|p| {
            (0..20).for_each(|_| {
                set_display(&mut latch_pin,&mut  data_pin,&mut  clock_pin, &digits_data[p]);
            })
        })

    }

}
