use std::time::{Duration,
                Instant};

use makro::{Key,
            Pos,
            Rgb,
            R,
            X};

use super::{FlaskSlot1,
            FlaskSlot2,
            FlaskSlot3,
            FlaskSlot4,
            FlaskSlot5};

const FLASKSLOT1_CHARGES: Pos = Pos::new(328, 1056);
const FLASKSLOT2_CHARGES: Pos = Pos::new(372, 1056);
const FLASKSLOT3_CHARGES: Pos = Pos::new(418, 1056);
const FLASKSLOT4_CHARGES: Pos = Pos::new(463, 1056);
const FLASKSLOT5_CHARGES: Pos = Pos::new(509, 1056);

const FLASK_RGB_RUNNING: Rgb = Rgb::new(249, 215, 153);

const FLASKSLOT1_RUNNING: Pos = Pos::new(312, 1071);
const FLASKSLOT2_RUNNING: Pos = Pos::new(358, 1071);
const FLASKSLOT3_RUNNING: Pos = Pos::new(404, 1071);
const FLASKSLOT4_RUNNING: Pos = Pos::new(450, 1071);
const FLASKSLOT5_RUNNING: Pos = Pos::new(496, 1071);

pub struct DurationFlaskSlot1 {
    last_used: Instant,
    duration: Duration,
    has_charges_color: [Rgb; 3],
    key: makro::Key,
}
impl DurationFlaskSlot1 {
    pub fn new(key: Key, duration: Duration, x: &X) -> Self {
        let colors = x.get_pixels_with_height(FLASKSLOT1_CHARGES, 3).unwrap().to_rgb();
        Self {
            key,
            last_used: Instant::now(),
            duration,
            has_charges_color: [colors[0], colors[1], colors[2]],
        }
    }
}
impl FlaskSlot1 for DurationFlaskSlot1 {
    fn pop(&mut self, vd: &mut makro::input::VDevice) {
        self.last_used = Instant::now();
        vd.tap(self.key);
    }
    fn is_on(&mut self, _: &X) -> R<bool> {
        Ok(self.last_used.elapsed().cmp(&self.duration).is_lt())
    }
    fn has_charges(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(FLASKSLOT1_CHARGES, 3)?
            .iter()
            .zip(self.has_charges_color)
            .all(|(a, b)| a.cmp_by_dist(b, 4.0).is_lt()))
    }
}

pub struct DurationFlaskSlot2 {
    last_used: Instant,
    duration: Duration,
    has_charges_color: [Rgb; 3],
    key: makro::Key,
}
impl DurationFlaskSlot2 {
    pub fn new(key: Key, duration: Duration, x: &X) -> Self {
        let colors = x.get_pixels_with_height(FLASKSLOT2_CHARGES, 3).unwrap().to_rgb();
        Self {
            key,
            last_used: Instant::now(),
            duration,
            has_charges_color: [colors[0], colors[1], colors[2]],
        }
    }
}
impl FlaskSlot2 for DurationFlaskSlot2 {
    fn pop(&mut self, vd: &mut makro::input::VDevice) {
        self.last_used = Instant::now();
        vd.tap(self.key);
    }
    fn is_on(&mut self, _: &X) -> R<bool> {
        Ok(self.last_used.elapsed().cmp(&self.duration).is_lt())
    }
    fn has_charges(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(FLASKSLOT2_CHARGES, 3)?
            .iter()
            .zip(self.has_charges_color)
            .all(|(a, b)| a.cmp_by_dist(b, 4.0).is_lt()))
    }
}

pub struct DurationFlaskSlot3 {
    last_used: Instant,
    duration: Duration,
    has_charges_color: [Rgb; 3],
    key: makro::Key,
}
impl DurationFlaskSlot3 {
    pub fn new(key: Key, duration: Duration, x: &X) -> Self {
        let colors = x.get_pixels_with_height(FLASKSLOT3_CHARGES, 3).unwrap().to_rgb();
        Self {
            key,
            last_used: Instant::now(),
            duration,
            has_charges_color: [colors[0], colors[1], colors[2]],
        }
    }
}
impl FlaskSlot3 for DurationFlaskSlot3 {
    fn pop(&mut self, vd: &mut makro::input::VDevice) {
        self.last_used = Instant::now();
        vd.tap(self.key);
    }
    fn is_on(&mut self, _: &X) -> R<bool> {
        Ok(self.last_used.elapsed().cmp(&self.duration).is_lt())
    }
    fn has_charges(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(FLASKSLOT3_CHARGES, 3)?
            .iter()
            .zip(self.has_charges_color)
            .all(|(a, b)| a.cmp_by_dist(b, 4.0).is_lt()))
    }
}

pub struct DurationFlaskSlot4 {
    last_used: Instant,
    duration: Duration,
    has_charges_color: [Rgb; 3],
    key: makro::Key,
}
impl DurationFlaskSlot4 {
    pub fn new(key: Key, duration: Duration, x: &X) -> Self {
        let colors = x.get_pixels_with_height(FLASKSLOT4_CHARGES, 3).unwrap().to_rgb();
        Self {
            key,
            last_used: Instant::now(),
            duration,
            has_charges_color: [colors[0], colors[1], colors[2]],
        }
    }
}
impl FlaskSlot4 for DurationFlaskSlot4 {
    fn pop(&mut self, vd: &mut makro::input::VDevice) {
        self.last_used = Instant::now();
        vd.tap(self.key);
    }
    fn is_on(&mut self, _: &X) -> R<bool> {
        Ok(self.last_used.elapsed().cmp(&self.duration).is_lt())
    }
    fn has_charges(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(FLASKSLOT4_CHARGES, 3)?
            .iter()
            .zip(self.has_charges_color)
            .all(|(a, b)| a.cmp_by_dist(b, 4.0).is_lt()))
    }
}

pub struct DurationFlaskSlot5 {
    last_used: Instant,
    duration: Duration,
    has_charges_color: [Rgb; 3],
    key: makro::Key,
}
impl DurationFlaskSlot5 {
    pub fn new(key: Key, duration: Duration, x: &X) -> Self {
        let colors = x.get_pixels_with_height(FLASKSLOT5_CHARGES, 3).unwrap().to_rgb();
        Self {
            key,
            last_used: Instant::now(),
            duration,
            has_charges_color: [colors[0], colors[1], colors[2]],
        }
    }
}
impl FlaskSlot5 for DurationFlaskSlot5 {
    fn is_on(&mut self, _: &X) -> R<bool> {
        Ok(self.last_used.elapsed().cmp(&self.duration).is_lt())
    }
    fn pop(&mut self, vd: &mut makro::input::VDevice) {
        self.last_used = Instant::now();
        vd.tap(self.key);
    }
    fn has_charges(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(FLASKSLOT5_CHARGES, 3)?
            .iter()
            .zip(self.has_charges_color)
            .all(|(a, b)| a.cmp_by_dist(b, 4.0).is_lt()))
    }
}

pub struct NonDurationFlaskSlot1 {
    has_charges_color: [Rgb; 3],
    key: makro::Key,
}
impl NonDurationFlaskSlot1 {
    pub fn new(key: Key, x: &X) -> Self {
        let colors = x.get_pixels_with_height(FLASKSLOT1_CHARGES, 3).unwrap().to_rgb();
        Self { key, has_charges_color: [colors[0], colors[1], colors[2]] }
    }
}
impl FlaskSlot1 for NonDurationFlaskSlot1 {
    fn pop(&mut self, vd: &mut makro::input::VDevice) {
        vd.tap(self.key);
    }
    fn is_on(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixel(FLASKSLOT1_RUNNING)?.cmp_by_dist(FLASK_RGB_RUNNING, 1.0).is_lt())
    }
    fn has_charges(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(FLASKSLOT1_CHARGES, 3)?
            .iter()
            .zip(self.has_charges_color)
            .all(|(a, b)| a.cmp_by_dist(b, 4.0).is_lt()))
    }
}

pub struct NonDurationFlaskSlot2 {
    has_charges_color: [Rgb; 3],
    key: makro::Key,
}
impl NonDurationFlaskSlot2 {
    pub fn new(key: Key, x: &X) -> Self {
        let colors = x.get_pixels_with_height(FLASKSLOT2_CHARGES, 3).unwrap().to_rgb();
        Self { key, has_charges_color: [colors[0], colors[1], colors[2]] }
    }
}
impl FlaskSlot2 for NonDurationFlaskSlot2 {
    fn pop(&mut self, vd: &mut makro::input::VDevice) {
        vd.tap(self.key);
    }
    fn is_on(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixel(FLASKSLOT2_RUNNING)?.cmp_by_dist(FLASK_RGB_RUNNING, 1.0).is_lt())
    }
    fn has_charges(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(FLASKSLOT2_CHARGES, 3)?
            .iter()
            .zip(self.has_charges_color)
            .all(|(a, b)| a.cmp_by_dist(b, 4.0).is_lt()))
    }
}

pub struct NonDurationFlaskSlot3 {
    has_charges_color: [Rgb; 3],
    key: makro::Key,
}
impl NonDurationFlaskSlot3 {
    pub fn new(key: Key, x: &X) -> Self {
        let colors = x.get_pixels_with_height(FLASKSLOT3_CHARGES, 3).unwrap().to_rgb();
        Self { key, has_charges_color: [colors[0], colors[1], colors[2]] }
    }
}
impl FlaskSlot3 for NonDurationFlaskSlot3 {
    fn pop(&mut self, vd: &mut makro::input::VDevice) {
        vd.tap(self.key);
    }
    fn is_on(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixel(FLASKSLOT3_RUNNING)?.cmp_by_dist(FLASK_RGB_RUNNING, 1.0).is_lt())
    }
    fn has_charges(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(FLASKSLOT3_CHARGES, 3)?
            .iter()
            .zip(self.has_charges_color)
            .all(|(a, b)| a.cmp_by_dist(b, 4.0).is_lt()))
    }
}

pub struct NonDurationFlaskSlot4 {
    has_charges_color: [Rgb; 3],
    key: makro::Key,
}
impl NonDurationFlaskSlot4 {
    pub fn new(key: Key, x: &X) -> Self {
        let colors = x.get_pixels_with_height(FLASKSLOT4_CHARGES, 3).unwrap().to_rgb();
        Self { key, has_charges_color: [colors[0], colors[1], colors[2]] }
    }
}
impl FlaskSlot4 for NonDurationFlaskSlot4 {
    fn pop(&mut self, vd: &mut makro::input::VDevice) {
        vd.tap(self.key);
    }
    fn is_on(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixel(FLASKSLOT4_RUNNING)?.cmp_by_dist(FLASK_RGB_RUNNING, 1.0).is_lt())
    }
    fn has_charges(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(FLASKSLOT4_CHARGES, 3)?
            .iter()
            .zip(self.has_charges_color)
            .all(|(a, b)| a.cmp_by_dist(b, 4.0).is_lt()))
    }
}

pub struct NonDurationFlaskSlot5 {
    has_charges_color: [Rgb; 3],
    key: makro::Key,
}
impl NonDurationFlaskSlot5 {
    pub fn new(key: Key, x: &X) -> Self {
        let colors = x.get_pixels_with_height(FLASKSLOT5_CHARGES, 3).unwrap().to_rgb();
        Self { key, has_charges_color: [colors[0], colors[1], colors[2]] }
    }
}
impl FlaskSlot5 for NonDurationFlaskSlot5 {
    fn pop(&mut self, vd: &mut makro::input::VDevice) {
        vd.tap(self.key);
    }
    fn is_on(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixel(FLASKSLOT5_RUNNING)?.cmp_by_dist(FLASK_RGB_RUNNING, 1.0).is_lt())
    }
    fn has_charges(&mut self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(FLASKSLOT5_CHARGES, 3)?
            .iter()
            .zip(self.has_charges_color)
            .all(|(a, b)| a.cmp_by_dist(b, 4.0).is_lt()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlaskIndex {
    F1 = 1,
    F2 = 2,
    F3 = 3,
    F4 = 4,
    F5 = 5,
}

impl From<u8> for FlaskIndex {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::F1,
            2 => Self::F2,
            3 => Self::F3,
            4 => Self::F4,
            5 => Self::F5,
            _ => panic!("FlaskSlot outside of 1 to 5 range. Entered with {value}"),
        }
    }
}

impl From<usize> for FlaskIndex {
    fn from(value: usize) -> Self {
        match value {
            1 => Self::F1,
            2 => Self::F2,
            3 => Self::F3,
            4 => Self::F4,
            5 => Self::F5,
            _ => panic!("FlaskSlot outside of 1 to 5 range. Entered with {value}"),
        }
    }
}
