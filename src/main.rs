use core::f32::consts::PI;
use rand::Rng;
use rppal::gpio::OutputPin;
use rppal::i2c::I2c;
use std::error::Error;
use std::slice::Iter;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::Trigger;
use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::system::DeviceInfo;

use chrono::Local;
use lcd_pcf8574::Pcf8574;
use std::fs;

fn get_cpu_tempture() -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?;
    let cpu_tmp = u16::from_str_radix(&contents.trim_end(), 10)?;

    Ok(format!("CPU Temp:{:.2}F ", cpu_tmp as f32 / 1000.0))
}

fn get_date_time() -> String {
    let now = Local::now();

    format!("{}", now.format("%H:%M:%S"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bus = 1;
    let addr = 0x27;
    let mut display = lcd::Display::new(Pcf8574::new(bus, addr)?);
    display.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    display.display(
        lcd::DisplayMode::DisplayOn,
        lcd::DisplayCursor::CursorOff,
        lcd::DisplayBlink::BlinkOff,
    );

    display.clear();
    display.home();

    loop {
        display.position(0, 0);
        display.print(get_cpu_tempture()?.as_str());
        display.position(0, 1);
        display.print(get_date_time().as_str());
        thread::sleep(Duration::from_secs(1));
    }

    // Ok(())
}
