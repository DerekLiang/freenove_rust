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

fn get_key(col_pins: &mut Vec<OutputPin>, rows_pins: &mut Vec<InputPin>) -> char {
    let key_codes = vec![
        vec!['1', '2', '3', 'A'],
        vec!['4', '5', '6', 'B'],
        vec!['7', '8', '9', 'C'],
        vec!['*', '0', '#', 'D'],
    ];

    loop {
        let col_width = col_pins.len();

        for scan_col in 0..col_width {
            for (col, col_pin) in col_pins.iter_mut().enumerate() {
                if scan_col == col {
                    col_pin.set_low();
                } else {
                    col_pin.set_high();
                }
                
                for (row, row_pin) in rows_pins.iter_mut().enumerate() {
                    if row_pin.is_low() {

                        // for unknown reason, the reading is not consistent
                        // so we double check by raising the pin to high and read it again
                        col_pin.set_high();
                        if row_pin.is_high() {                            
                            let key = key_codes[row][col_width - 1 - scan_col];
                            // println!("scan col {}, col {}, row {}", scan_col, col, row);
                            return key;
                        } else {
                            // println!("phantom read!");
                        }
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let row_pin_numbers = vec![18, 23, 24, 25];
    let col_pin_numbers = vec![10, 22, 27, 17];

    let mut rows_pins = row_pin_numbers
        .into_iter()
        .map(|d| Ok(Gpio::new()?.get(d)?.into_input_pullup()) as Result<_, Box<dyn Error>>)
        .fold_ok(vec![], |mut acc, pin| {
            acc.push(pin);
            acc as Vec<InputPin>
        })?;

    let mut col_pins = col_pin_numbers
        .into_iter()
        .map(|d| Ok(Gpio::new()?.get(d)?.into_output()) as Result<_, Box<dyn Error>>)
        .fold_ok(vec![], |mut acc, pin| {
            acc.push(pin);
            acc as Vec<OutputPin>
        })?;

    loop {
        println!(" reading {}", get_key(&mut col_pins, &mut rows_pins));
        thread::sleep(Duration::from_secs(1));
    }
}
