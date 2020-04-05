extern crate sysfs_gpio;

use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

fn main() {
    let red_led = Pin::new(25);
    let yellow_led = Pin::new(8);
    let green_led = Pin::new(7);
    let buzzer = Pin::new(15);
    let button = Pin::new(1);

    red_led.export().unwrap();
    red_led.set_direction(Direction::Out).unwrap();

    yellow_led.export().unwrap();
    yellow_led.set_direction(Direction::Out).unwrap();

    green_led.export().unwrap();
    green_led.set_direction(Direction::Out).unwrap();

    buzzer.export().unwrap();
    buzzer.set_direction(Direction::Out).unwrap();

    button
        .with_exported(|| {
            button.set_direction(Direction::In)?;

            // The red led is on by default
            red_led.set_value(1).unwrap();

            // The button's initial state is High, or 1.
            let mut prev_val: u8 = 1;
            loop {
                let val = button.get_value()?;
                if val != prev_val {
                    if val == 1 {
                        sleep(Duration::from_millis(1000));
                        // Red lef off
                        red_led.set_value(0).unwrap();

                        // Green led on and beep it the buzzer 4 times
                        green_led.set_value(1).unwrap();
                        for _x in 0..3 {
                            buzzer.set_value(1).unwrap();
                            sleep(Duration::from_millis(500));
                            buzzer.set_value(0).unwrap();
                            sleep(Duration::from_millis(500));
                        }

                        // Yellow led on for 2 seconds and turning off green led
                        yellow_led.set_value(1).unwrap();
                        green_led.set_value(0).unwrap();
                        sleep(Duration::from_millis(2000));

                        // Red light on again.
                        red_led.set_value(1).unwrap();
                        yellow_led.set_value(0).unwrap();
                    }
                    prev_val = val;
                }
                sleep(Duration::from_millis(10));
            }
        })
        .unwrap();
}
