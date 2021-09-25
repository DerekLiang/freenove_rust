use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::Trigger;
use rppal::system::DeviceInfo;

const GPIO_LEDS: [u8;10] = [17, 18, 27, 22, 23, 24, 25, 2, 3, 8];   

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    
    while true {      
        for &led in GPIO_LEDS.iter() {
            
            let mut led_pin = Gpio::new()?.get(led)?.into_output();
            led_pin.set_low();
            thread::sleep(Duration::from_millis(100));
            led_pin.set_high();
            thread::sleep(Duration::from_millis(100));
        }
    }

    Ok(())
}
