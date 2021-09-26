use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::Trigger;
use rppal::system::DeviceInfo;
use rppal::pwm::{Channel, Polarity, Pwm};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    // Enable PWM channel 0 (BCM GPIO 18, physical pin 12) at 2 Hz with a 25% duty cycle.
    let pwm = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true)?;

    // Sleep for 2 seconds while the LED blinks.
    thread::sleep(Duration::from_secs(2));

    // Reconfigure the PWM channel for an 8 Hz frequency, 50% duty cycle.
    pwm.set_frequency(8.0, 0.5)?;

    thread::sleep(Duration::from_secs(3));

    Ok(())
}
