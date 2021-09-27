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

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut alarm_pin = Gpio::new()?.get(17)?.into_output();
    let mut switch_pin = Gpio::new()?.get(18)?.into_input_pullup();

    alarm_pin.set_low();
    println!("set almarm pin to low");
    
    let mut switch_level = Arc::new(Mutex::<Option<Level>>::new(None));
    let switch_level_data = switch_level.clone();

    switch_pin.set_async_interrupt(Trigger::Both, move |l| {
        let mut thread_switchLevel = switch_level_data.lock().unwrap();
        *thread_switchLevel = Some(l);
    })?;
    
    while true {
        match *switch_level.lock().unwrap() {
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
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
