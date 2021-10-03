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

// run the following command and find the address
// > i2cdetect -y 1
const i2cAddress: u16 = 0x4b;

fn map(value: i16, from_low: i16, from_high: i16, to_low: i16, to_high: i16) -> i16 {
    (to_high - to_low) * (value - from_low) / (from_high - from_low) + to_low
}

fn motor(
    adc: u8,
    pin1: &mut OutputPin,
    pin2: &mut OutputPin,
    enable_pin: &mut OutputPin,
) -> Result<(), Box<dyn Error>> {
    let value = adc as i16 - 128;
    match value {
        v if v > 0 => {
            pin1.set_high();
            pin2.set_low();
            println!("turn forward....");
        }
        v if v < 0 => {
            pin1.set_low();
            pin2.set_high();
            println!("turn backward....");
        }
        _ => {
            pin1.set_low();
            pin2.set_low();
            println!("Motor Stop...");
        }
    }

    let duty_cycle = map(value.abs(), 0, 128, 0, 100) as u64;
    println!("The PWM duty cycle is {}%", duty_cycle);

    enable_pin.set_pwm(
        Duration::from_millis(20),
        Duration::from_micros(2000 * duty_cycle / 100),
    )?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(i2cAddress)?;

    let mut motor_pin1 = Gpio::new()?.get(27)?.into_output(); // 2
    let mut motor_pin2 = Gpio::new()?.get(17)?.into_output(); // 0
    let mut enable_pin = Gpio::new()?.get(22)?.into_output(); // 3

    loop {
        let commands = [0x84, 0xc4, 0x94, 0xd4, 0xa4, 0xe4, 0xb4, 0xf4];

        let mut adc = [0u8; 1];
        i2c.block_read(commands[0], &mut adc)?;

        println!("ADC value : {}", adc[0]);

        motor(adc[0], &mut motor_pin1, &mut motor_pin2, &mut enable_pin)?;
        thread::sleep(Duration::from_millis(1000));
    }

}
