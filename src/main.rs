

extern crate rppal;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rppal::gpio::{Gpio, Mode, PullUpDown};
extern crate hal_sensor_dht;
use hal_sensor_dht::{DHTSensor, SensorType};

struct MyPin(rppal::gpio::IoPin);

impl MyPin {
    pub fn new(pin: rppal::gpio::Pin) -> MyPin {
        MyPin(pin.into_io(Mode::Input))
    }
}

impl InputPin for MyPin {
    type Error = <rppal::gpio::IoPin as InputPin>::Error;
    fn is_high(&self) -> Result<bool, <rppal::gpio::IoPin as InputPin>::Error> {
        Ok(self.0.is_high())
    }
    fn is_low(&self) -> Result<bool, <rppal::gpio::IoPin as InputPin>::Error> {
        Ok(self.0.is_low())
    }
}

impl OutputPin for MyPin {
    type Error = <rppal::gpio::IoPin as OutputPin>::Error;
    fn set_high(&mut self) -> Result<(), <rppal::gpio::IoPin as OutputPin>::Error> {
        Ok(self.0.set_high())
    }
    fn set_low(&mut self) -> Result<(), <rppal::gpio::IoPin as OutputPin>::Error> {
        Ok(self.0.set_low())
    }
}

impl hal_sensor_dht::IoPin for MyPin {
    fn set_input_pullup_mode(&mut self) {
        self.0.set_mode(Mode::Input);
        self.0.set_pullupdown(PullUpDown::PullUp);
    }
    fn set_output_mode(&mut self) {
        self.0.set_mode(Mode::Output);
    }
}

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use std::ptr::read_volatile;
use std::ptr::write_volatile;
use std::thread;
use std::time::Duration;

struct MyTimer {}

impl DelayUs<u16> for MyTimer {
    fn delay_us(&mut self, t: u16) {
        let mut i = 0;
        unsafe {
            while read_volatile(&mut i) < t {
                write_volatile(&mut i, read_volatile(&mut i) + 1);
            }
        }
    }
}

impl DelayMs<u16> for MyTimer {
    fn delay_ms(&mut self, ms: u16) {
        thread::sleep(Duration::from_millis(ms.into()));
    }
}

extern crate libc;
use libc::sched_param;
use libc::sched_setscheduler;
use libc::SCHED_FIFO;
use libc::SCHED_OTHER;

struct MyInterruptCtrl {}

impl hal_sensor_dht::InterruptCtrl for MyInterruptCtrl {
    fn enable(&mut self) {
        unsafe {
            let param = sched_param { sched_priority: 32 };
            let result = sched_setscheduler(0, SCHED_FIFO, &param);

            if result != 0 {
                panic!("Error setting priority, you may not have cap_sys_nice capability");
            }
        }
    }
    fn disable(&mut self) {
        unsafe {
            let param = sched_param { sched_priority: 0 };
            let result = sched_setscheduler(0, SCHED_OTHER, &param);

            if result != 0 {
                panic!("Error setting priority, you may not have cap_sys_nice capability");
            }
        }
    }
}
use std::error::Error;

// to run it, under super user;
// cargo build && sudo target/debug/rust_test
fn main() -> Result<(), Box<dyn Error>> {
    let pin_number = 17;

    let pin = Gpio::new()?.get(pin_number)?;
    let my_pin = MyPin::new(pin);
    let my_timer = MyTimer {};
    let my_interrupt = MyInterruptCtrl {};
    let mut sensor = DHTSensor::new(SensorType::DHT11, my_pin, my_timer, my_interrupt);

    loop {
        if let Ok(r) = sensor.read() {
            println!(
                "Temperature = {} / {} and humidity = {}",
                r.temperature_celsius(),
                r.temperature_fahrenheit(),
                r.humidity_percent()
            );
        }
        thread::sleep(Duration::from_secs(10));
    }
}
