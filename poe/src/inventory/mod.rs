use std::collections::BTreeSet;
use std::time::Duration;

use config::CfgIgnoreMaps;
use makro::input::VDevice;
use makro::{Clipboard,
            Key,
            Pos,
            X};

pub mod data;
pub mod item;
pub mod types;

pub use data::{inv_pos_get_by_rc,
               INV_POS,
               INV_POS_COL,
               NONE,
               SOME,
               UNID};
pub use types::{Class,
                Rarity};

use self::item::Item;

const UNID_SLEEP: Duration = Duration::from_millis(20);
const SLEEP: Duration = Duration::from_millis(10);
const SMALL_SLEEP: Duration = Duration::from_millis(1);

macro_rules! sleep {
    () => {
        std::thread::sleep(SLEEP);
    };
}

macro_rules! unid_sleep {
    () => {
        std::thread::sleep(UNID_SLEEP);
    };
}

macro_rules! small_sleep {
    () => {
        std::thread::sleep(SMALL_SLEEP);
    };
}

// /// Returns a vec of positions of inventory squares that have [UNID] background
// fn get_unid(x: &X) -> Vec<Pos> {
//     let mut ret = Vec::with_capacity(INV_POS.len());
//     x.move_mouse(Pos::new(0, 0)).unwrap();
//
//     for pos in INV_POS {
//         let color = x.get_pixel(pos).unwrap();
//         if color.cmp_by_dist(UNID, 5.0).is_le() {
//             ret.push(pos);
//         }
//     }
//
//     ret
// }

/// Returns a vec of positions of inventory squares that have [SOME] background
fn get_some(x: &X) -> Vec<Pos> {
    let mut ret = Vec::with_capacity(INV_POS.len());
    x.move_mouse(Pos::new(0, 0)).unwrap();

    for pos in INV_POS {
        let color = x.get_pixel(pos).unwrap();
        if color.cmp_by_dist(SOME, 5.0).is_lt() {
            ret.push(pos);
        }
    }

    ret
}

/// Clicks all the inventory rows with [SOME] and [UNID] backgrounds
pub fn click_all_items(x: &X, vd: &mut VDevice) {
    let mut pos_some = Vec::with_capacity(INV_POS.len());

    x.move_mouse(Pos::new(0, 0)).unwrap();

    for pos in INV_POS {
        let color = x.get_pixel(pos).unwrap();
        if color.cmp_by_dist(SOME, 5.0).is_lt() {
            pos_some.push(pos);
        }
        if color.cmp_by_dist(UNID, 5.0).is_lt() {
            pos_some.push(pos);
        }
    }

    for pos in pos_some {
        x.move_mouse(pos).unwrap();
        std::thread::sleep(SLEEP);
        vd.tap_with_lctrl(Key::BTN_LEFT);
        std::thread::sleep(SLEEP);
    }
}

/// Drops off items that are affinities
pub fn drop_by_affinity(x: &X, vd: &mut VDevice, c: &mut Clipboard) {
    let some = get_some(x);

    for pos in some {
        let clip = c.move_and_get(pos, vd, x);
        let item = Item::new(clip);

        if item.class.is_affinity() {
            vd.tap_with_lctrl(Key::BTN_LEFT);
        }
    }
}

pub fn unid_items(
    pos_of_wisdom: Pos,
    x: &X,
    vd: &mut VDevice,
    c: &mut Clipboard,
) -> Option<&'static str> {
    if Item::new(c.move_and_get(pos_of_wisdom, vd, x)).name.unwrap_or("") != "Scroll of Wisdom" {
        return Some("The item in 'wisdom_scroll' was not 'Scroll of Wisdom'");
    }

    sleep!();
    vd.tap(Key::BTN_RIGHT);
    vd.press(Key::KEY_LEFTSHIFT);
    sleep!();

    x.move_mouse(Pos::new(0, 0)).unwrap(); // move the mouse so it doesn't cover the background

    for chunk in INV_POS.chunks(12) {
        for pos in chunk {
            let pos = *pos;
            if x.get_pixel(pos).unwrap().cmp_by_dist(UNID, 5.0).is_lt() {
                x.move_mouse(pos).unwrap();
                unid_sleep!();
                vd.tap(Key::BTN_LEFT);
                unid_sleep!();
            }
        }

        x.move_mouse(Pos::new(0, 0)).unwrap(); // move the mouse so it doesn't cover the background
        small_sleep!();
    }

    vd.release(Key::KEY_LEFTSHIFT);

    None
}

/// Ctrl-clicks all the equipment item in the inventory
/// Skips veiled items
pub fn sell_simple(
    // pos_of_wisdom: Pos,
    // ignore: &[(Pos, String)],
    ignore_items: &BTreeSet<Pos>,
    x: &X,
    vd: &mut VDevice,
    c: &mut Clipboard,
) {
    // let set_ignore =
    //     std::collections::BTreeSet::from_iter(ignore.iter().filter_map(|(pos, str)| {
    //         if x.get_pixel(*pos).unwrap().cmp_by_dist(SOME, 5.0).is_lt() {
    //             let clip = c.get(*pos, vd, x);
    //             if str == clip { Some(*pos) } else { None }
    //         } else {
    //             None
    //         }
    //     }));
    // sleep!();

    sleep!();

    x.move_mouse(Pos::new(0, 0)).unwrap(); // move the mouse so it doesn't cover the background

    for chunk in INV_POS.chunks(12) {
        for pos in chunk {
            if ignore_items.contains(pos) {
                continue;
            }

            let pos = *pos;
            if x.get_pixel(pos).unwrap().cmp_by_dist(SOME, 5.0).is_lt() {
                let clip = c.move_and_get(pos, vd, x);
                let item = Item::new(clip);

                if item.class.is_equipment()
                    && matches!(item.rarity, Rarity::Normal | Rarity::Magic | Rarity::Rare)
                {
                    if Item::is_veiled(clip) {
                        continue;
                    }

                    x.move_mouse(pos).unwrap();
                    sleep!();
                    vd.tap_with_lctrl(Key::BTN_LEFT);
                    sleep!();
                }
            }
        }

        x.move_mouse(Pos::new(0, 0)).unwrap(); // move the mouse so it doesn't cover the background
        small_sleep!();
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IgnoreMaps {
    pub min_tier: u8,
    pub min_rarity: Rarity,
    pub max_columns_pos: i16,
}

impl From<CfgIgnoreMaps> for IgnoreMaps {
    fn from(value: CfgIgnoreMaps) -> Self {
        let min_rarity = match value.min_rarity.as_str() {
            "Rare" => Rarity::Rare,
            "Magic" => Rarity::Magic,
            "Normal" => Rarity::Normal,
            "Unique" => Rarity::Unique,
            s => {
                panic!("Wrong 'min_rarity', expected 'Rare', 'Magic', 'Normal' or 'Unique' got {s}")
            }
        };
        Self {
            min_tier: value.min_tier,
            min_rarity,
            max_columns_pos: INV_POS_COL[value.max_column - 1],
        }
    }
}

pub fn stash_simple(
    ignore_items: &BTreeSet<Pos>,
    ignore_maps: IgnoreMaps,
    x: &X,
    vd: &mut VDevice,
    c: &mut Clipboard,
) {
    // let set_ignore =
    //     std::collections::BTreeSet::from_iter(ignore_items.iter().filter_map(|(pos, str)| {
    //         if x.get_pixel(*pos).unwrap().cmp_by_dist(SOME, 5.0).is_lt() {
    //             let clip = c.get(*pos, vd, x);
    //             if str == clip { Some(*pos) } else { None }
    //         } else {
    //             None
    //         }
    //     }));

    x.move_mouse(Pos::new(0, 0)).unwrap(); // move the mouse so it doesn't cover the background

    for chunk in INV_POS.chunks(12) {
        for pos in chunk {
            if ignore_items.contains(pos) {
                continue;
            }

            let pos = *pos;
            if x.get_pixel(pos).unwrap().cmp_by_dist(SOME, 5.0).is_lt() {
                let clip = c.move_and_get(pos, vd, x);
                let item = Item::new(clip);

                if item.class.is_map() {
                    if pos.x <= ignore_maps.max_columns_pos
                        && is_rarity_ge(item.rarity, ignore_maps.min_rarity)
                        && Item::extract_map_tier(clip) >= ignore_maps.min_tier
                    {
                        continue;
                    }

                    x.move_mouse(pos).unwrap();
                    sleep!();
                    vd.tap_with_lctrl(Key::BTN_LEFT);
                    sleep!();
                } else if item.class.is_affinity() {
                    x.move_mouse(pos).unwrap();
                    sleep!();
                    vd.tap_with_lctrl(Key::BTN_LEFT);
                    sleep!();
                }
            }
        }

        x.move_mouse(Pos::new(0, 0)).unwrap(); // move the mouse so it doesn't cover the background
        small_sleep!();
    }
}

fn is_rarity_ge(a: Rarity, b: Rarity) -> bool {
    match (a, b) {
        (Rarity::Normal, Rarity::Normal) => true,
        (Rarity::Normal, Rarity::Magic) => false,
        (Rarity::Normal, Rarity::Rare) => false,
        (Rarity::Normal, Rarity::Unique) => false,

        (Rarity::Magic, Rarity::Normal) => true,
        (Rarity::Magic, Rarity::Magic) => true,
        (Rarity::Magic, Rarity::Rare) => false,
        (Rarity::Magic, Rarity::Unique) => false,

        (Rarity::Rare, Rarity::Normal) => true,
        (Rarity::Rare, Rarity::Magic) => true,
        (Rarity::Rare, Rarity::Rare) => true,
        (Rarity::Rare, Rarity::Unique) => false,

        (Rarity::Unique, Rarity::Normal) => true,
        (Rarity::Unique, Rarity::Magic) => true,
        (Rarity::Unique, Rarity::Rare) => true,
        (Rarity::Unique, Rarity::Unique) => true,

        _ => unreachable!(),
    }
}
