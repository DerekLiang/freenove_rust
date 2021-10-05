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

struct StepMoter {
    step: u8,
    pins: Vec<OutputPin>,
    min_delay_ms: u8, // min delay before each steps
}

#[derive(Clone, Copy, PartialEq)]
enum MoterDirection {
    Forward,
    Backward,
}

impl StepMoter {
    pub fn new(pins: Vec<u8>, step: u8, min_delay_ms: u8) -> Result<Self, Box<dyn Error>> {
        let pins = Self::get_pins(pins)?;
        Ok(Self {
            step,
            pins,
            min_delay_ms,
        })
    }

    fn get_pins(pins: Vec<u8>) -> Result<Vec<OutputPin>, Box<dyn Error>> {
        let mut result = vec![];
        for pin in pins {
            result.push(Gpio::new()?.get(pin)?.into_output());
        }
        Ok(result)
    }

    fn move_one_period(&mut self, direction: MoterDirection) {
        let min_delay_ms = self.min_delay_ms.max(3) as u64;
        let step = self.step;

        (0..self.step)
            .into_iter()
            .map(|cycle| match direction {
                MoterDirection::Forward => cycle as usize,
                MoterDirection::Backward => (step - cycle - 1) as usize,
            })
            .for_each(|cycle| {
                self.pins.iter_mut().enumerate().for_each(|(index, pin)| {
                    if index == cycle {
                        pin.set_high();
                    } else {
                        pin.set_low();
                    }
                });
                thread::sleep(Duration::from_millis(min_delay_ms));
            });
    }

    pub fn move_steps(&mut self, steps: u32, direction: MoterDirection) {
        (0..steps)
            .into_iter()
            .for_each(|_| self.move_one_period(direction))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut step_moter = StepMoter::new(vec![18, 23, 24, 25], 4, 3)?;
    loop {
        step_moter.move_steps(512, MoterDirection::Forward);
        thread::sleep(Duration::from_millis(500));

        step_moter.move_steps(512, MoterDirection::Backward);
        thread::sleep(Duration::from_millis(500));
    }
}
