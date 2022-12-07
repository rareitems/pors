use makro::{input,
            Key};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyboardState {
    Normal,
    LCtrl,
    LShift,
    // LCtrlShift,
    LCtrlF,
    LAlt,
    Enter,
}

impl KeyboardState {
    pub fn release_state(self, vd: &mut input::VDevice) {
        match self {
            KeyboardState::Normal => (),
            KeyboardState::LCtrl => vd.release(Key::KEY_LEFTCTRL),
            KeyboardState::LShift => vd.release(Key::KEY_LEFTSHIFT),
            KeyboardState::LAlt => vd.release(Key::KEY_LEFTALT),

            // KeyboardState::LCtrlShift => unreachable!(),
            KeyboardState::LCtrlF => unreachable!(),
            KeyboardState::Enter => unreachable!(),
        }
    }

    pub fn press_state(self, vd: &mut input::VDevice) {
        match self {
            KeyboardState::Normal => (),
            KeyboardState::LCtrl => vd.press(Key::KEY_LEFTCTRL),
            KeyboardState::LShift => vd.press(Key::KEY_LEFTSHIFT),
            KeyboardState::LAlt => vd.press(Key::KEY_LEFTALT),

            // KeyboardState::LCtrlShift => unreachable!(),
            KeyboardState::LCtrlF => unreachable!(),
            KeyboardState::Enter => unreachable!(),
        }
    }
}

impl KeyboardState {
    pub fn on_release(&mut self, key: Key) {
        let state = *self;
        match key {
            // Key::KEY_LEFTSHIFT if state == KeyboardState::LCtrlShift => {
            // *self = KeyboardState::LCtrl
            // }
            // Key::KEY_LEFTCTRL if state == KeyboardState::LCtrlShift => {
            //     *self = KeyboardState::LShift
            // }
            Key::KEY_LEFTSHIFT if state == KeyboardState::LShift => *self = KeyboardState::Normal,
            Key::KEY_LEFTCTRL if state == KeyboardState::LCtrl => *self = KeyboardState::Normal,

            Key::KEY_LEFTALT if state == KeyboardState::LAlt => *self = KeyboardState::Normal,

            // Key::KEY_ESC if state == KeybaordState::LCtrlF => *self = KeybaordState::Normal,
            // Key::KEY_ENTER if state == KeybaordState::LCtrlF => *self = KeybaordState::Normal,
            // Key::KEY_ENTER if state == KeybaordState::Enter => *self = KeybaordState::Normal,
            _ => (),
        }
    }

    pub fn change_on_press(&mut self, key: Key) -> bool {
        let state = *self;
        match key {
            Key::KEY_F if state == KeyboardState::LCtrl => {
                *self = KeyboardState::LCtrlF;
                false
            }

            Key::KEY_ENTER if state == KeyboardState::Normal => {
                *self = KeyboardState::Enter;
                false
            }

            Key::KEY_ENTER if state == KeyboardState::Enter => {
                *self = KeyboardState::Normal;
                false
            }

            Key::KEY_ENTER if state == KeyboardState::LCtrlF => {
                *self = KeyboardState::Normal;
                false
            }

            Key::KEY_ESC if state == KeyboardState::LCtrlF => {
                *self = KeyboardState::Normal;
                false
            }

            // Key::KEY_LEFTSHIFT if state == KeyboardState::LCtrl => {
            //     *self = KeyboardState::LCtrlShift;
            //     false
            // }
            Key::KEY_LEFTSHIFT => {
                *self = KeyboardState::LShift;
                false
            }

            // Key::KEY_LEFTCTRL if state == KeyboardState::LShift => {
            //     *self = KeyboardState::LCtrlShift;
            //     false
            // }
            Key::KEY_LEFTCTRL => {
                *self = KeyboardState::LCtrl;
                false
            }

            Key::KEY_LEFTALT => {
                *self = KeyboardState::LAlt;
                false
            }

            _ => true,
        }
    }
}
