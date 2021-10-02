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

fn alertor( pin: &mut OutputPin  ) -> Result<(), Box<dyn Error>> {
    for x in 0..360 {
        let sin_val = (x as f32 * (PI / 180.0)).sin();
        let ton_val = 2000.0 + sin_val * 1000.0;
        let value = ((1000 as f32) * ton_val / 3000.0) as u64;
        // println!("value {}", value);
        pin.set_pwm(
            Duration::from_millis(1),
            Duration::from_micros(value),
        )?;
        thread::sleep(Duration::from_micros(100));
    };    
    Ok(())
}

fn stop_alertor(pin: &mut OutputPin)-> Result<(), Box<dyn Error>>  {
    pin.set_pwm(
        Duration::from_millis(20),
        Duration::from_micros(((20000 as f32) * 0.0) as u64),
    )?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut alarm_pin = Gpio::new()?.get(17)?.into_output();
    let mut switch_pin = Gpio::new()?.get(18)?.into_input_pullup();

    alarm_pin.set_low();
    println!("set almarm pin to low");
    
    let switch_level = Arc::new(Mutex::<Option<Level>>::new(None));
    let switch_level_data = switch_level.clone();

    switch_pin.set_async_interrupt(Trigger::Both, move |l| {
        let mut thread_switch_level = switch_level_data.lock().unwrap();
        *thread_switch_level = Some(l);
    })?;
    
    while true {
        match *switch_level.lock().unwrap() {
            Some(level) => {
                if level == Level::High {
                    stop_alertor(&mut alarm_pin)?;                    
                } else {
                    alertor(&mut alarm_pin)?;
                    println!("set almarm pin to low");
                }
            }
            None => {
                stop_alertor(&mut alarm_pin)?;
            }
        }
        thread::sleep(Duration::from_micros(1));
    }

    Ok(())
}
