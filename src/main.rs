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

fn read_and_set(i2c: &I2c, index: usize, led: &mut OutputPin) -> Result<(), Box<dyn Error>> {
    // a0-a7 adc port
    let commands = [0x84, 0xc4, 0x94, 0xd4, 0xa4, 0xe4, 0xb4, 0xf4];

    let mut reg = [0u8; 1];
    i2c.block_read(commands[index], &mut reg)?;

    println!(
        "ADC{} value : {}, Voltage: {:2}",
        index,
        reg[0],
        reg[0] as f32 / 255.0 * 3.3
    );

    led.set_pwm(
        Duration::from_millis(20),
        Duration::from_micros(20000 * reg[0] as u64 / 255), // wthin 20 ms we take a percentage as the power level.
    )?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(i2cAddress)?;

    let mut leds = vec![
        Gpio::new()?.get(17)?.into_output(),
        Gpio::new()?.get(27)?.into_output(),
        Gpio::new()?.get(22)?.into_output(),
    ];

    while true {
        for index in 0..leds.len() {
            read_and_set(&i2c, index, &mut leds[index])?;
        }

        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
