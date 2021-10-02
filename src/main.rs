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
    let mut switch_pin = Gpio::new()?.get(18)?.into_input_pullup();

    // NOTE: poll_interrupt is not working reliably, using async instead
    let switch_level = Arc::new(Mutex::<Option<Level>>::new(None));
    let switch_level_data = switch_level.clone();

    switch_pin.set_async_interrupt(Trigger::Both, move |l| {
        let mut thread_switch_level = switch_level_data.lock().unwrap();
        *thread_switch_level = Some(l);
    })?;

    while true {
        let commands = [0x84, 0xc4, 0x94, 0xd4, 0xa4, 0xe4, 0xb4, 0xf4];

        let mut reg_y = [0u8; 1];
        i2c.block_read(commands[0], &mut reg_y)?;

        let mut reg_x = [0u8; 1];
        i2c.block_read(commands[1], &mut reg_x)?;

        let val_z = *switch_level.lock().unwrap();

        println!(
            "val_X: {} ,\tval_Y: {} ,\tval_Z: {:?}",
            reg_x[0], reg_y[0], val_z
        );

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
