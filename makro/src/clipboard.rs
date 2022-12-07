use std::time::Duration;

use evdev::Key;
use tracing::error;

use crate::input::VDevice;
use crate::{Pos,
            X};

pub struct Clipboard {
    arboard: arboard::Clipboard,
    curr: String,
}

const SLEEP: Duration = Duration::from_millis(20);

macro_rules! sleep {
    () => {
        std::thread::sleep(SLEEP);
    };
}

impl Clipboard {
    pub fn new() -> Self {
        let mut arboard = arboard::Clipboard::new().unwrap();
        let curr = arboard.get_text().unwrap_or("".into());
        Self { arboard, curr }
    }

    #[inline]
    fn get_clipboard(&mut self, pos: Pos, vd: &mut VDevice, x: &X) -> String {
        x.move_mouse(pos).unwrap();
        sleep!();
        vd.tap_with_lctrl(Key::KEY_C);
        sleep!();
        self.get()
    }

    pub fn move_and_get(&mut self, pos: Pos, vd: &mut VDevice, x: &X) -> &str {
        let curr = self.get_clipboard(pos, vd, x);
        if curr == self.curr {
            sleep!();
            error!("had the same clipboard");
            self.curr = self.get_clipboard(pos, vd, x);
            &self.curr
        } else {
            self.curr = curr;
            &self.curr
        }
    }

    #[inline]
    pub fn set(&mut self, text: &str) {
        self.arboard.set_text(text).unwrap();
    }

    #[inline]
    pub fn get(&mut self) -> String {
        self.arboard.get_text().unwrap()
    }
}

impl Default for Clipboard {
    fn default() -> Self {
        Self::new()
    }
}
