use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::Trigger;
use rppal::system::DeviceInfo;

const GPIO_LED: u8 = 17; // GPIO17
const GPIO_SWITCH: u8 = 18; // GPIO18

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut led_pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    let mut switch_pin = Gpio::new()?.get(GPIO_SWITCH)?.into_input_pullup();
    switch_pin.set_interrupt(Trigger::Both)?;

    while true {        
        let switch = switch_pin.poll_interrupt(false, None)?;
        match switch {
            Some(level) => {
                if level == Level::High {
                    led_pin.set_high();
                } else {
                    led_pin.set_low();
                }
            }
            None => {}
        }

        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
