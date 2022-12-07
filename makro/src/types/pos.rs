#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Pos {
    pub x: i16,
    pub y: i16,
}

impl Pos {
    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl From<(i16, i16)> for Pos {
    fn from(value: (i16, i16)) -> Self {
        Pos { x: value.0, y: value.1 }
    }
}
