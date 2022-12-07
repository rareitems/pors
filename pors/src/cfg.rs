use std::collections::BTreeSet;

use config::{CfgCharacter,
             CfgGlobal};
use makro::Pos;
use poe::inventory::IgnoreMaps;

#[derive(Debug)]
pub struct Config {
    pub pos_wisdom: Option<Pos>,
    pub ignore_items: Option<BTreeSet<Pos>>,
    pub ignore_maps: Option<IgnoreMaps>,
}

impl Config {
    pub fn new(global: &CfgGlobal, _char: &CfgCharacter) -> Self {
        Config {
            pos_wisdom: global.wisdom_scroll.map(|(r, c)| poe::inventory::inv_pos_get_by_rc(r, c)),
            ignore_items: global.ignore_items.as_ref().map(|it| {
                BTreeSet::from_iter(
                    it.iter().map(|(r, c)| poe::inventory::inv_pos_get_by_rc(*r, *c)),
                )
            }),
            ignore_maps: global.ignore_maps.as_ref().map(|it| IgnoreMaps::from(it.clone())),
        }
    }
}
