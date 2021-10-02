use rppal::i2c::I2c;
use core::f32::consts::PI;
use rppal::gpio::OutputPin;
use std::error::Error;
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::sync::{Arc, Mutex};

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::Trigger;
use rppal::system::DeviceInfo;
use rppal::pwm::{Channel, Polarity, Pwm};

// run the following command and find the address
// > i2cdetect -y 1
const i2cAddress: u16 = 0x4b;


fn main() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(i2cAddress)?;

    // representing different ports, from a0-a7
    let commands = [0x84, 0xc4, 0x94, 0xd4, 0xa4, 0xe4, 0xb4, 0xf4];
    let mut reg = [0u8; 1];

    while true {
        
        let adc_value = i2c.block_read(commands[0], &mut reg)?;
        println!("ADC value : {}, Voltage: {:2}", reg[0],  reg[0] as f32 / 255.0 * 3.3);
        
        thread::sleep(Duration::from_secs(1)); 
    }

    Ok(())
}
