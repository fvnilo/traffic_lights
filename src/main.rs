use gpio_cdev::{Chip, EventRequestFlags, LineRequestFlags};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let red_led = chip.get_line(25).unwrap();
    let red_led_handle = red_led
        .request(LineRequestFlags::OUTPUT, 0, "red_led")
        .unwrap();

    let yellow_led = chip.get_line(8).unwrap();
    let yellow_led_handle = yellow_led
        .request(LineRequestFlags::OUTPUT, 0, "yellow_led")
        .unwrap();

    let green_led = chip.get_line(7).unwrap();
    let green_led_handle = green_led
        .request(LineRequestFlags::OUTPUT, 0, "red_led")
        .unwrap();

    let buzzer = chip.get_line(15).unwrap();
    let buzzer_handle = buzzer
        .request(LineRequestFlags::OUTPUT, 0, "buzzer")
        .unwrap();

    let button = chip.get_line(1).unwrap();

    // Red light on by default.
    red_led_handle.set_value(1).unwrap();

    for _event in button
        .events(
            LineRequestFlags::INPUT,
            EventRequestFlags::RISING_EDGE,
            "button-input",
        )
        .unwrap()
    {
        sleep(Duration::from_millis(1000));
        // Red lef off
        red_led_handle.set_value(0).unwrap();

        // Green led on and beep it the buzzer 4 times
        green_led_handle.set_value(1).unwrap();
        for _x in 0..3 {
            buzzer_handle.set_value(1).unwrap();
            sleep(Duration::from_millis(500));
            buzzer_handle.set_value(0).unwrap();
            sleep(Duration::from_millis(500));
        }

        // Yellow led on for 2 seconds and turning off green led
        yellow_led_handle.set_value(1).unwrap();
        green_led_handle.set_value(0).unwrap();
        sleep(Duration::from_millis(2000));

        // Red light on again.
        red_led_handle.set_value(1).unwrap();
        yellow_led_handle.set_value(0).unwrap();
    }
}
