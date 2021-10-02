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


fn main() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(i2cAddress)?;

    while true {
        let commands = [0x84, 0xc4, 0x94, 0xd4, 0xa4, 0xe4, 0xb4, 0xf4];

        let mut reg = [0u8; 1];
        i2c.block_read(commands[0], &mut reg)?;
    
        let voltage = reg[0] as f32 / 255.0 * 3.3;
        let rt = 10.0 * voltage / (3.3 -voltage);
        let temp_k = 1.0/(1.0/(273.15+25.0) + (rt/10.0).ln()/3950.0);
        let temp_c = temp_k - 273.15;

        println!("ADC value : {} ,\tVoltage : {:.2}V, \tTemperature : {:.2}C\n", reg[0] ,voltage, temp_c);

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
