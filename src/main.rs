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

    let mut pin2 = Gpio::new()?.get(17)?.into_output();
    let mut pin3 = Gpio::new()?.get(27)?.into_output();

    // Enable PWM channel 0 (BCM GPIO 18, physical pin 12) at 2 Hz with a 25% duty cycle.
    let pwm1 = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true)?;
    // let pwm2 = Pwm::with_frequency(Channel::Pwm1, 2.0, 0.25, Polarity::Normal, true)?;

    let mut rng = rand::thread_rng();
    
    while true {
        let n1 = rng.gen_range(0.0..1.0);
        let n2: f32 = rng.gen_range(0.0..1.0);
        let n3: f32 = rng.gen_range(0.0..1.0);

        pin2.set_pwm(
            Duration::from_millis(20),
            Duration::from_micros(((20000 as f32) * n2) as u64), // wthin 20 ms we take a percentage as the power level.
        )?;

        pin3.set_pwm(
            Duration::from_millis(20),
            Duration::from_micros(((20000 as f32) * n3) as u64),
        )?;

        
        pwm1.set_frequency(8.0, n1)?;
        // pwm2.set_frequency(8.0, n2)?;
        thread::sleep(Duration::from_secs(1));
    }


    Ok(())
}
