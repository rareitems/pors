use evdev::uinput::VirtualDeviceBuilder;
use evdev::{AttributeSet,
            EventType,
            InputEvent,
            Key};

static MOUSE_BTNS: [&str; 20] = [
    "BTN_MISC",
    "BTN_0",
    "BTN_1",
    "BTN_2",
    "BTN_3",
    "BTN_4",
    "BTN_5",
    "BTN_6",
    "BTN_7",
    "BTN_8",
    "BTN_9",
    "BTN_MOUSE",
    "BTN_LEFT",
    "BTN_RIGHT",
    "BTN_MIDDLE",
    "BTN_SIDE",
    "BTN_EXTRA",
    "BTN_FORWARD",
    "BTN_BACK",
    "BTN_TASK",
];

pub struct VDevice {
    vdevice: evdev::uinput::VirtualDevice,
}

impl VDevice {
    pub fn new() -> Self {
        let mut keys: AttributeSet<Key> = AttributeSet::new();
        for code in Key::KEY_RESERVED.code()..Key::BTN_TRIGGER_HAPPY40.code() {
            let key = Key::new(code);
            let name = format!("{:?}", key);

            if MOUSE_BTNS.contains(&&*name) {
                keys.insert(key);
            }
            if name.starts_with("KEY_") {
                keys.insert(key);
            }
        }

        // let mut relative_axes: AttributeSet<RelativeAxisType> = AttributeSet::new();
        // relative_axes.insert(RelativeAxisType::REL_X);
        // relative_axes.insert(RelativeAxisType::REL_Y);
        // relative_axes.insert(RelativeAxisType::REL_HWHEEL);
        // relative_axes.insert(RelativeAxisType::REL_WHEEL);
        // relative_axes.insert(RelativeAxisType::REL_MISC);

        let vdevice = VirtualDeviceBuilder::new()
            .unwrap()
            .name("vdevice")
            .with_keys(&keys)
            .unwrap()
            // .with_relative_axes(&relative_axes)
            // .unwrap()
            .build()
            .unwrap();

        Self { vdevice }
    }

    pub fn emit_event(&mut self, event: InputEvent) {
        self.vdevice.emit(&[event]).unwrap()
    }

    pub fn emit_events(&mut self, events: &[InputEvent]) {
        self.vdevice.emit(events).unwrap()
    }

    #[inline]
    pub fn press(&mut self, key: evdev::Key) {
        self.vdevice.emit(&[InputEvent::new(EventType::KEY, key.code(), 1)]).unwrap();
    }

    #[inline]
    pub fn release(&mut self, key: evdev::Key) {
        self.vdevice.emit(&[InputEvent::new(EventType::KEY, key.code(), 0)]).unwrap();
    }

    #[inline]
    pub fn tap(&mut self, key: evdev::Key) {
        self.press(key);
        self.release(key);
    }

    pub fn type_string(mut prefix: Vec<InputEvent>, to_type: &str) -> Vec<InputEvent> {
        prefix.reserve(to_type.len() * 2);

        for char in to_type.chars() {
            let c = char_to_key(char);
            prefix.push(InputEvent::new(EventType::KEY, c.code(), 1));
            prefix.push(InputEvent::new(EventType::KEY, c.code(), 0));
        }

        prefix
    }

    pub fn tap_with_lctrl(&mut self, key: evdev::Key) {
        self.vdevice
            .emit(&[
                InputEvent::new(EventType::KEY, Key::KEY_LEFTCTRL.code(), 1),
                InputEvent::new(EventType::KEY, key.code(), 1),
                InputEvent::new(EventType::KEY, key.code(), 0),
                InputEvent::new(EventType::KEY, Key::KEY_LEFTCTRL.code(), 0),
            ])
            .unwrap()
    }

    pub fn tap_with_lshift(&mut self, key: evdev::Key) {
        self.vdevice
            .emit(&[
                InputEvent::new(EventType::KEY, Key::KEY_LEFTSHIFT.code(), 1),
                InputEvent::new(EventType::KEY, key.code(), 1),
                InputEvent::new(EventType::KEY, key.code(), 0),
                InputEvent::new(EventType::KEY, Key::KEY_LEFTSHIFT.code(), 0),
            ])
            .unwrap()
    }

    pub fn tap_with_lalt(&mut self, key: evdev::Key) {
        self.vdevice
            .emit(&[
                InputEvent::new(EventType::KEY, Key::KEY_LEFTALT.code(), 1),
                InputEvent::new(EventType::KEY, key.code(), 1),
                InputEvent::new(EventType::KEY, key.code(), 0),
                InputEvent::new(EventType::KEY, Key::KEY_LEFTALT.code(), 0),
            ])
            .unwrap()
    }

    pub fn tap_with_lctrl_lshift(&mut self, key: evdev::Key) {
        self.vdevice
            .emit(&[
                InputEvent::new(EventType::KEY, Key::KEY_LEFTCTRL.code(), 1),
                InputEvent::new(EventType::KEY, Key::KEY_LEFTSHIFT.code(), 1),
                InputEvent::new(EventType::KEY, key.code(), 1),
                InputEvent::new(EventType::KEY, key.code(), 0),
                InputEvent::new(EventType::KEY, Key::KEY_LEFTCTRL.code(), 0),
                InputEvent::new(EventType::KEY, Key::KEY_LEFTSHIFT.code(), 0),
            ])
            .unwrap()
    }
}

impl Default for VDevice {
    fn default() -> Self {
        Self::new()
    }
}

fn char_to_key(c: char) -> evdev::Key {
    match c {
        'a' => evdev::Key::KEY_A,
        'b' => evdev::Key::KEY_B,
        'c' => evdev::Key::KEY_C,
        'd' => evdev::Key::KEY_D,
        'e' => evdev::Key::KEY_E,
        'f' => evdev::Key::KEY_F,
        'g' => evdev::Key::KEY_G,
        'h' => evdev::Key::KEY_H,
        'i' => evdev::Key::KEY_I,
        'j' => evdev::Key::KEY_J,
        'k' => evdev::Key::KEY_K,
        'l' => evdev::Key::KEY_L,
        'm' => evdev::Key::KEY_M,
        'n' => evdev::Key::KEY_N,
        'o' => evdev::Key::KEY_O,
        'p' => evdev::Key::KEY_P,
        'q' => evdev::Key::KEY_Q,
        'r' => evdev::Key::KEY_R,
        's' => evdev::Key::KEY_S,
        't' => evdev::Key::KEY_T,
        'u' => evdev::Key::KEY_U,
        'v' => evdev::Key::KEY_V,
        'w' => evdev::Key::KEY_W,
        'x' => evdev::Key::KEY_X,
        'y' => evdev::Key::KEY_Y,
        'z' => evdev::Key::KEY_Z,
        '/' => evdev::Key::KEY_SLASH,
        _ => panic!("Unsupported character {c}"),
    }
}
