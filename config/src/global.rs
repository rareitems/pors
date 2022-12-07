use serde::{Deserialize,
            Serialize};

use crate::character::cfg_keymaps::Keymaps;

/// User's supplied keys to which flasks are binded in-game.
/// Slots are counted from left to right.
#[derive(Debug, Deserialize, Serialize)]
pub struct KeysFlasks {
    pub slot1: makro::Key,
    pub slot2: makro::Key,
    pub slot3: makro::Key,
    pub slot4: makro::Key,
    pub slot5: makro::Key,
}

/// User's supplied keys to which flasks are binded in-game.
/// Slots are counted from left to right.
#[derive(Debug, Deserialize)]
pub struct KeysSkills {
    #[serde(default)]
    pub skill1: Option<makro::Key>,
    #[serde(default)]
    pub skill2: Option<makro::Key>,
    #[serde(default)]
    pub skill3: Option<makro::Key>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CfgIgnoreMaps {
    pub max_column: usize,
    pub min_tier: u8,
    pub min_rarity: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct CfgGlobal {
    pub device_path: String,
    pub keys_flasks: KeysFlasks,

    #[serde(default)]
    pub keys_skills: Option<KeysSkills>,

    #[serde(default)]
    pub ignore_items: Option<Vec<(usize, usize)>>,

    #[serde(default)]
    pub ignore_maps: Option<CfgIgnoreMaps>,

    #[serde(default)]
    pub wisdom_scroll: Option<(usize, usize)>,

    #[serde(default)]
    pub Keymaps: Option<Keymaps>,
}

impl CfgGlobal {
    pub fn parse(path: &std::path::Path) -> Self {
        toml::from_str::<CfgGlobal>(&std::fs::read_to_string(path).unwrap()).unwrap()
    }
}
