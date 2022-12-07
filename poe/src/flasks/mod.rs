use std::time::Duration;

pub mod types;

use config::{CfgCharacter,
             CfgGlobal};
use makro::input::VDevice;
use makro::{R,
            X};

use self::types::{DurationFlaskSlot1,
                  DurationFlaskSlot2,
                  DurationFlaskSlot3,
                  DurationFlaskSlot4,
                  DurationFlaskSlot5,
                  FlaskIndex,
                  NonDurationFlaskSlot1,
                  NonDurationFlaskSlot2,
                  NonDurationFlaskSlot3,
                  NonDurationFlaskSlot4,
                  NonDurationFlaskSlot5};

pub struct Flasks {
    pub flask1: Option<Box<dyn FlaskSlot1>>,
    pub flask2: Option<Box<dyn FlaskSlot2>>,
    pub flask3: Option<Box<dyn FlaskSlot3>>,
    pub flask4: Option<Box<dyn FlaskSlot4>>,
    pub flask5: Option<Box<dyn FlaskSlot5>>,
}

impl Flasks {
    pub fn new(cfg_global: &CfgGlobal, cfg_character: &CfgCharacter, x: &X) -> Self {
        Flasks {
            flask1: {
                if let Some(f) = cfg_character.Flask1.as_ref() {
                    match &f.kind {
                        config::FlaskKind::Duration => Some(Box::new(DurationFlaskSlot1::new(
                            cfg_global.keys_flasks.slot1,
                            Duration::from_millis(f.duration.unwrap()),
                            x,
                        ))),
                        config::FlaskKind::NonDuration => Some(Box::new(
                            NonDurationFlaskSlot1::new(cfg_global.keys_flasks.slot1, x),
                        )),
                    }
                } else {
                    None
                }
            },
            flask2: {
                if let Some(f) = cfg_character.Flask2.as_ref() {
                    match &f.kind {
                        config::FlaskKind::Duration => Some(Box::new(DurationFlaskSlot2::new(
                            cfg_global.keys_flasks.slot2,
                            Duration::from_millis(f.duration.unwrap()),
                            x,
                        ))),
                        config::FlaskKind::NonDuration => Some(Box::new(
                            NonDurationFlaskSlot2::new(cfg_global.keys_flasks.slot2, x),
                        )),
                    }
                } else {
                    None
                }
            },
            flask3: {
                if let Some(f) = cfg_character.Flask3.as_ref() {
                    match &f.kind {
                        config::FlaskKind::Duration => Some(Box::new(DurationFlaskSlot3::new(
                            cfg_global.keys_flasks.slot3,
                            Duration::from_millis(f.duration.unwrap()),
                            x,
                        ))),
                        config::FlaskKind::NonDuration => Some(Box::new(
                            NonDurationFlaskSlot3::new(cfg_global.keys_flasks.slot3, x),
                        )),
                    }
                } else {
                    None
                }
            },
            flask4: {
                if let Some(f) = cfg_character.Flask4.as_ref() {
                    match &f.kind {
                        config::FlaskKind::Duration => Some(Box::new(DurationFlaskSlot4::new(
                            cfg_global.keys_flasks.slot4,
                            Duration::from_millis(f.duration.unwrap()),
                            x,
                        ))),
                        config::FlaskKind::NonDuration => Some(Box::new(
                            NonDurationFlaskSlot4::new(cfg_global.keys_flasks.slot4, x),
                        )),
                    }
                } else {
                    None
                }
            },
            flask5: {
                if let Some(f) = cfg_character.Flask5.as_ref() {
                    match &f.kind {
                        config::FlaskKind::Duration => Some(Box::new(DurationFlaskSlot5::new(
                            cfg_global.keys_flasks.slot5,
                            Duration::from_millis(f.duration.unwrap()),
                            x,
                        ))),
                        config::FlaskKind::NonDuration => Some(Box::new(
                            NonDurationFlaskSlot5::new(cfg_global.keys_flasks.slot5, x),
                        )),
                    }
                } else {
                    None
                }
            },
        }
    }

    pub fn is_on(&mut self, index: FlaskIndex, x: &X) -> R<bool> {
        match index {
            FlaskIndex::F1 => self
                .flask1
                .as_mut()
                .expect("Flask in spot 2 is not specified, but expected to use")
                .is_on(x),
            FlaskIndex::F2 => self
                .flask2
                .as_mut()
                .expect("Flask in spot 2 is not specified, but expected to use")
                .is_on(x),
            FlaskIndex::F3 => self
                .flask3
                .as_mut()
                .expect("Flask in spot 3 is not specified, but expected to use")
                .is_on(x),
            FlaskIndex::F4 => self
                .flask4
                .as_mut()
                .expect("Flask in spot 4 is not specified, but expected to use")
                .is_on(x),
            FlaskIndex::F5 => self
                .flask5
                .as_mut()
                .expect("Flask in spot 5 is not specified, but expected to use")
                .is_on(x),
        }
    }

    pub fn pop(&mut self, index: FlaskIndex, vd: &mut VDevice) {
        match index {
            FlaskIndex::F1 => self
                .flask1
                .as_mut()
                .expect("Flask in spot 2 is not specified, but expected to use")
                .pop(vd),
            FlaskIndex::F2 => self
                .flask2
                .as_mut()
                .expect("Flask in spot 2 is not specified, but expected to use")
                .pop(vd),
            FlaskIndex::F3 => self
                .flask3
                .as_mut()
                .expect("Flask in spot 3 is not specified, but expected to use")
                .pop(vd),
            FlaskIndex::F4 => self
                .flask4
                .as_mut()
                .expect("Flask in spot 4 is not specified, but expected to use")
                .pop(vd),
            FlaskIndex::F5 => self
                .flask5
                .as_mut()
                .expect("Flask in spot 5 is not specified, but expected to use")
                .pop(vd),
        }
    }

    pub fn has_charges(&mut self, index: FlaskIndex, x: &X) -> R<bool> {
        match index {
            FlaskIndex::F1 => self
                .flask1
                .as_mut()
                .expect("Flask in spot 2 is not specified, but expected to use")
                .has_charges(x),
            FlaskIndex::F2 => self
                .flask2
                .as_mut()
                .expect("Flask in spot 2 is not specified, but expected to use")
                .has_charges(x),
            FlaskIndex::F3 => self
                .flask3
                .as_mut()
                .expect("Flask in spot 3 is not specified, but expected to use")
                .has_charges(x),
            FlaskIndex::F4 => self
                .flask4
                .as_mut()
                .expect("Flask in spot 4 is not specified, but expected to use")
                .has_charges(x),
            FlaskIndex::F5 => self
                .flask5
                .as_mut()
                .expect("Flask in spot 5 is not specified, but expected to use")
                .has_charges(x),
        }
    }

    pub fn check_and_pop(&mut self, index: FlaskIndex, x: &X, vd: &mut VDevice) -> R<bool> {
        match index {
            FlaskIndex::F1 => self
                .flask1
                .as_mut()
                .expect("Flask in spot 2 is not specified, but expected to use")
                .check_and_pop(x, vd),

            FlaskIndex::F2 => self
                .flask2
                .as_mut()
                .expect("Flask in spot 2 is not specified, but expected to use")
                .check_and_pop(x, vd),

            FlaskIndex::F3 => self
                .flask3
                .as_mut()
                .expect("Flask in spot 3 is not specified, but expected to use")
                .check_and_pop(x, vd),

            FlaskIndex::F4 => self
                .flask4
                .as_mut()
                .expect("Flask in spot 4 is not specified, but expected to use")
                .check_and_pop(x, vd),

            FlaskIndex::F5 => self
                .flask5
                .as_mut()
                .expect("Flask in spot 5 is not specified, but expected to use")
                .check_and_pop(x, vd),
        }
    }
}

pub trait FlaskSlot1 {
    fn is_on(&mut self, x: &X) -> R<bool>;
    fn has_charges(&mut self, x: &X) -> R<bool>;
    fn pop(&mut self, vd: &mut makro::input::VDevice);

    fn can_use(&mut self, x: &X) -> R<bool> {
        Ok(!self.is_on(x)? && self.has_charges(x)?)
    }
    fn check_and_pop(&mut self, x: &X, vd: &mut makro::input::VDevice) -> R<bool> {
        if !self.is_on(x)? && self.has_charges(x)? {
            self.pop(vd);
            return Ok(true);
        }
        Ok(false)
    }
}

pub trait FlaskSlot2 {
    fn is_on(&mut self, x: &X) -> R<bool>;
    fn has_charges(&mut self, x: &X) -> R<bool>;
    fn pop(&mut self, vd: &mut makro::input::VDevice);

    fn can_use(&mut self, x: &X) -> R<bool> {
        Ok(!self.is_on(x)? && self.has_charges(x)?)
    }
    fn check_and_pop(&mut self, x: &X, vd: &mut makro::input::VDevice) -> R<bool> {
        if !self.is_on(x)? && self.has_charges(x)? {
            self.pop(vd);
            return Ok(true);
        }
        Ok(false)
    }
}

pub trait FlaskSlot3 {
    fn is_on(&mut self, x: &X) -> R<bool>;
    fn has_charges(&mut self, x: &X) -> R<bool>;
    fn pop(&mut self, vd: &mut makro::input::VDevice);

    fn can_use(&mut self, x: &X) -> R<bool> {
        Ok(!self.is_on(x)? && self.has_charges(x)?)
    }
    fn check_and_pop(&mut self, x: &X, vd: &mut makro::input::VDevice) -> R<bool> {
        if !self.is_on(x)? && self.has_charges(x)? {
            self.pop(vd);
            return Ok(true);
        }
        Ok(false)
    }
}

pub trait FlaskSlot4 {
    fn is_on(&mut self, x: &X) -> R<bool>;
    fn has_charges(&mut self, x: &X) -> R<bool>;
    fn pop(&mut self, vd: &mut makro::input::VDevice);

    fn can_use(&mut self, x: &X) -> R<bool> {
        Ok(!self.is_on(x)? && self.has_charges(x)?)
    }
    fn check_and_pop(&mut self, x: &X, vd: &mut makro::input::VDevice) -> R<bool> {
        if !self.is_on(x)? && self.has_charges(x)? {
            self.pop(vd);
            return Ok(true);
        }
        Ok(false)
    }
}

pub trait FlaskSlot5 {
    fn is_on(&mut self, x: &X) -> R<bool>;
    fn has_charges(&mut self, x: &X) -> R<bool>;
    fn pop(&mut self, vd: &mut makro::input::VDevice);

    fn can_use(&mut self, x: &X) -> R<bool> {
        Ok(!self.is_on(x)? && self.has_charges(x)?)
    }
    fn check_and_pop(&mut self, x: &X, vd: &mut makro::input::VDevice) -> R<bool> {
        if !self.is_on(x)? && self.has_charges(x)? {
            self.pop(vd);
            return Ok(true);
        }
        Ok(false)
    }
}
