#![no_std]
#![no_main]

#![allow(dead_code)] // 使用しないメソッドでコンパイラが警告を出さないようにする

use panic_halt as _; // (1)
use wio_terminal as wio; // (1)

use wio::entry; // (1)
use wio::pac::Peripherals; // (1)

use wio_lib::gpio::Led; // (2)
use wio_lib::gpio::Button1; // (2)

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap(); // (3)
    let mut pins = wio::Pins::new(peripherals.PORT); // (4)

    // LEDドライバオブジェクトを初期化する
    let mut led = Led::new(pins.user_led, &mut pins.port); // (5)
    // ボタンドライバオブジェクトを初期化する
    let button1 = Button1::new(pins.button1, &mut pins.port); // (6)

/*
let mut sets = wio::Pins::new(peripherals.PORT).split(); // (4')
let mut led = Led::new(sets.user_led, &mut sets.port); // (5')
let button1 = Button1::new(sets.buttons.button1, &mut sets.port); (6')
*/

    loop {
        if button1.is_pressed() { // (7)
            led.turn_on(); // (8)
        } else {
            led.turn_off(); // (9)
        }
    }

}
