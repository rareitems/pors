// pub const INV_ROWS: [i16; 5] = [589, 642, 694, 747, 800];
// pub const INV_COL: [i16; 12] = [1322, 1375, 1428, 1480, 1533, 1586, 1638, 1691, 1744, 1796, 1849,
// 1902];

use super::{Class,
            Rarity};

// struct UnidItem {
//     pos: Pos,
// }

#[derive(Debug)]
pub struct Item<'a> {
    pub rarity: Rarity,
    pub class: Class,
    pub name: Option<&'a str>,
}

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::RegexBuilder::new($re).multi_line(true).build().unwrap())
    }};
}

impl<'a> Item<'a> {
    pub fn new(clip: &str) -> Item {
        let re_rarity = regex!(r"^Rarity: (.+)$");
        let re_class = regex!(r"^Item Class: (.+)$");

        Item {
            rarity: re_rarity.captures(clip).unwrap()[1].parse().unwrap(),
            class: re_class.captures(clip).unwrap()[1].parse().unwrap(),
            name: clip.lines().nth(2),
        }
    }

    pub fn is_unid(clip: &str) -> bool {
        let re = regex!(r"^Unidentified$");
        re.is_match(clip)
    }

    pub fn extract_map_tier(clip: &str) -> u8 {
        let tier = regex!(r"^Map Tier: (.+)$");
        tier.captures(clip).unwrap()[1].parse().unwrap()
    }

    pub fn is_veiled(clip: &str) -> bool {
        let re = regex!(r"^Veiled Suffix|Veiled Prefix$");
        re.is_match(clip)
    }
}
