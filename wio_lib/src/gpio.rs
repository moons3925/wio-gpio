use wio_terminal as wio; // (1)
use wio::hal::gpio::*; // (1)
use wio::prelude::*; // (1) 主要な構造体やトレイトをインポートする

pub struct Button1 { // (2)
    pin: Pc26<Input<Floating>>, // (3)
}

impl Button1 { // (4)
    // PC26ピンを入力モードに設定する
    pub fn new(pin: Pc26<Input<Floating>>, port: &mut Port) -> Button1 { // (5)
        Button1 {
            pin: pin.into_floating_input(port),
        }
    }
    // ボタンが押されていればtrueを返す
    pub fn is_pressed(&self) -> bool { // (6)
        self.pin.is_low().unwrap()
    }
    // ボタンが押されていなければtrueを返す
    pub fn is_released(&self) -> bool { // (7)
        self.pin.is_high().unwrap()
    }
}

// Wio TerminalのユーザーLEDドライバ
pub struct Led { // (8)
    pin: Pa15<Output<PushPull>>, // (9)
}

impl Led { // (10)
    // デフォルトモードのPA15ピンを、出力モードに移行する
    pub fn new(pin: Pa15<Input<Floating>>, port: &mut Port) -> Led { // (11)
        Led {
            pin: pin.into_push_pull_output(port),
        }
    }
    // LEDを点灯する
    pub fn turn_on(&mut self) { // (12)
        self.pin.set_high().unwrap();
    }
    // LEDを消灯する
    pub fn turn_off(&mut self) { // (13)
        self.pin.set_low().unwrap();
    }
    // LEDが点灯しているときは消灯し、消灯しているときは点灯する
    pub fn toggle(&mut self) { // (14)
        self.pin.toggle();
    }
}
