use config::{CfgCharacter,
             CfgGlobal};
use makro::{Key,
            Pos,
            Rgb,
            R,
            X};

// pub trait Skill {
//     fn is_on_cd(&mut self, x: &X) -> R<bool>;
//     fn has_charges(&mut self, _: &X) -> R<bool> {
//         Ok(true)
//     }
//     fn check_and_use(&mut self, x: &X, vd: &mut makro::input::VDevice) -> R<bool>;
//     fn can_use(&mut self, x: &X) -> R<bool> {
//         Ok(!self.is_on_cd(x)? && self.has_charges(x)?)
//     }
// }

pub struct Skill {
    pos: Pos,
    colors: [Rgb; 3],
    key: Key,
}

impl Skill {
    #[allow(clippy::eq_op)]
    pub fn build_from_config(
        cfg_global: &CfgGlobal,
        cfg_char: &CfgCharacter,
        x: &X,
    ) -> [Option<Skill>; 3] {
        let mut ret = [None, None, None];
        let keys_skills = cfg_global.keys_skills.as_ref().expect("No config");

        ret[1 - 1] = None;

        if let Some(skill2) = cfg_char.Skill2.as_ref() {
            if skill2.active {
                ret[2 - 1] = Some(Self::new_2(
                    keys_skills.skill2.expect("No Key for Skill2 in Global Config"),
                    x,
                ));
            }
        }

        if let Some(skill3) = cfg_char.Skill3.as_ref() {
            if skill3.active {
                ret[3 - 1] = Some(Self::new_3(
                    keys_skills.skill3.expect("No Key for Skill3 in Global Config"),
                    x,
                ));
            }
        }

        ret
    }

    pub fn new_2(key: Key, x: &X) -> Self {
        let pos = Pos::new(1604, 952);
        let colors = x.get_pixels_with_height(pos, 3).unwrap().to_rgb();
        assert_eq!(colors.len(), 3);
        Skill { pos, colors: [colors[0], colors[1], colors[2]], key }
    }

    pub fn new_3(key: Key, x: &X) -> Self {
        let pos = Pos::new(1660, 952);
        let colors = x.get_pixels_with_height(pos, 3).unwrap().to_rgb();
        assert_eq!(colors.len(), 3);
        Skill { pos, colors: [colors[0], colors[1], colors[2]], key }
    }
}

impl Skill {
    pub fn is_on_cd(&self, x: &X) -> R<bool> {
        let colors = x.get_pixels_with_height(self.pos, 3)?;
        assert_eq!(colors.to_rgb().len(), 3);
        Ok(colors.into_iter().zip(self.colors.iter()).all(|(x, y)| x.cmp_by_dist(*y, 1.0).is_gt()))
    }

    pub fn check_and_use(&self, x: &X, vd: &mut makro::input::VDevice) -> R<bool> {
        if self.is_on_cd(x)? {
            Ok(false)
        } else {
            vd.tap(self.key);
            Ok(true)
        }
    }
}
