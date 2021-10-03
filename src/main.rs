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

// Servo configuration. Change these values based on your servo's verified safe
// minimum and maximum values.
//
// Period: 20 ms (50 Hz). Pulse width: min. 500 µs, neutral 1500 µs, max. 2500 µs.
// for micro servo 9g sg90
const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 500;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 2500;

fn main() -> Result<(), Box<dyn Error>> {

    // you might need to reboot, if the following code is not working.
    let pwm = Pwm::with_period(
        Channel::Pwm0,
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_MAX_US),
        Polarity::Normal,
        true,
    )?;

    println!("moving to max position");
    pwm.set_pulse_width(Duration::from_micros(PULSE_MAX_US))?;
    // Sleep for 500 ms while the servo moves into position.
    thread::sleep(Duration::from_millis(500));
    
    println!("moving to min position");
    // Rotate the servo to the opposite side.
    pwm.set_pulse_width(Duration::from_micros(PULSE_MIN_US))?;

    thread::sleep(Duration::from_millis(500));

    println!("moving from min to middle");
    // Rotate the servo to its neutral (center) position in small steps.
    for pulse in (PULSE_MIN_US..=PULSE_NEUTRAL_US).step_by(10) {
        pwm.set_pulse_width(Duration::from_micros(pulse))?;
        thread::sleep(Duration::from_millis(20));
    }

    Ok(())


}
