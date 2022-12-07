use makro::{Pos,
            Rgb,
            R,
            X};

pub trait DetectMana {
    fn perc(&self, x: &X) -> R<u8>;
}

pub struct SimpleMana;

impl DetectMana for SimpleMana {
    fn perc(&self, x: &X) -> makro::R<u8> {
        if x.get_pixel(Pos::new(1770, 980))?.cmp_by_dist(Rgb::new(19, 79, 149), 1.0).is_lt() {
            Ok(100)
        } else {
            Ok(40)
        }
    }
}
