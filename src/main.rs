use gpio_cdev::{Chip, EventRequestFlags, LineRequestFlags};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();

    // SOUTH
    let red_led_south = chip.get_line(25).unwrap();
    let red_led_south_handle = red_led_south
        .request(LineRequestFlags::OUTPUT, 0, "red_led_south")
        .unwrap();

    let yellow_led_south = chip.get_line(8).unwrap();
    let yellow_led_south_handle = yellow_led_south
        .request(LineRequestFlags::OUTPUT, 0, "yellow_led_south")
        .unwrap();

    let green_led_south = chip.get_line(7).unwrap();
    let green_led_south_handle = green_led_south
        .request(LineRequestFlags::OUTPUT, 0, "green_led_south")
        .unwrap();

    // EAST
    let red_led_east = chip.get_line(14).unwrap();
    let red_led_east_handle = red_led_east
        .request(LineRequestFlags::OUTPUT, 0, "red_led_east")
        .unwrap();

    let yellow_led_east = chip.get_line(15).unwrap();
    let yellow_led_east_handle = yellow_led_east
        .request(LineRequestFlags::OUTPUT, 0, "yellow_led_east")
        .unwrap();

    let green_led_east = chip.get_line(18).unwrap();
    let green_led_east_handle = green_led_east
        .request(LineRequestFlags::OUTPUT, 0, "green_led_east")
        .unwrap();

    // let button = chip.get_line(1).unwrap();

    red_led_south_handle.set_value(1).unwrap();
    green_led_east_handle.set_value(1).unwrap();

    // for _event in button
    //     .events(
    //         LineRequestFlags::INPUT,
    //         EventRequestFlags::RISING_EDGE,
    //         "button-input",
    //     )
    //     .unwrap()
    // {
    loop {
        red_led_south_handle.set_value(1).unwrap();
        green_led_east_handle.set_value(1).unwrap();

        sleep(Duration::from_millis(3000));

        green_led_east_handle.set_value(0).unwrap();
        yellow_led_east_handle.set_value(1).unwrap();

        sleep(Duration::from_millis(3000));

        yellow_led_east_handle.set_value(0).unwrap();
        red_led_east_handle.set_value(1).unwrap();

        sleep(Duration::from_millis(1000));

        red_led_south_handle.set_value(0).unwrap();
        green_led_south_handle.set_value(1).unwrap();

        sleep(Duration::from_millis(3000));

        yellow_led_south_handle.set_value(1).unwrap();
        green_led_south_handle.set_value(0).unwrap();

        sleep(Duration::from_millis(3000));

        red_led_south_handle.set_value(1).unwrap();
        yellow_led_south_handle.set_value(0).unwrap();

        sleep(Duration::from_millis(1000));

        red_led_east_handle.set_value(0).unwrap();
    }
    // }
}
