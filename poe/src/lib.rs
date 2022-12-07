pub mod detection;
pub mod flasks;
pub mod health;
pub mod inventory;
pub mod mana;
pub mod skills;

use detection::Detect;
pub use flasks::Flasks;
use health::DetectHealth;
pub use mana::DetectMana;

pub struct PoE<D, HP, MANA>
where
    D: Detect,
    HP: DetectHealth,
    MANA: DetectMana,
{
    pub detection: D,
    pub health: HP,
    pub mana: MANA,
}

// impl<D, HP, MANA> PoE<D, HP, MANA>
// where
//     D: Detect,
//     HP: DetectHealth,
//     MANA: DetectMana,
// {
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
