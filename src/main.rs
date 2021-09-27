use std::error::Error;
use std::thread;
use std::time::Duration;
use rand::Rng;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::Trigger;
use rppal::system::DeviceInfo;
use rppal::pwm::{Channel, Polarity, Pwm};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut alarm_pin = Gpio::new()?.get(17)?.into_output();
    let mut switch_pin = Gpio::new()?.get(18)?.into_input_pullup();

    switch_pin.set_interrupt(Trigger::Both)?;
    alarm_pin.set_low();
    println!("set almarm pin to low");
    
    while true {
        let switch = switch_pin.poll_interrupt(false, None)?;
        match switch {
            Some(level) => {
                if level == Level::High {
                    alarm_pin.set_high();
                    println!("set almarm pin to high");
                } else {
                    alarm_pin.set_low();
                    println!("set almarm pin to low");
                }
            }
            None => {}
        }
    }

    Ok(())
}
