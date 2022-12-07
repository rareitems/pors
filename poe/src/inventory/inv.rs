use makro::{Pos,
            Rgb};
use regex::Regex;

// pub const UNID: Rgb = Rgb::new(44, 5, 3);
// pub const NONE: Rgb = Rgb::new(13, 10, 5);
// pub const SOME: Rgb = Rgb::new(7, 5, 29);

struct UnidItem {
    pos: Pos,
}

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
    pub fn is_unid(clip: &str) -> bool {
        let re = regex!(r"^Unidentified$");
        re.is_match(clip)
    }

    pub fn simple_parse(clip: &str) -> Item {
        let re_rarity = regex!(r"^Rarity: (.+)$");
        let re_class = regex!(r"^Item Class: (.+)$");

        Item {
            rarity: unsafe { re_rarity.captures(clip).unwrap()[1].parse().unwrap_unchecked() },
            class: unsafe { re_class.captures(clip).unwrap()[1].parse().unwrap_unchecked() },
            name: clip.lines().nth(2),
        }
    }
}
