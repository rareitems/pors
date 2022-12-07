use makro::{Pos,
            Rgb,
            R,
            X};

pub trait Detect {
    fn is_on(&self, x: &X) -> R<bool>;
    fn is_poe_window_on(&self, x: &X) -> R<bool>;
    fn is_vendor_window_open(&self, x: &X) -> R<bool>;
    fn is_stash_window_open(&self, x: &X) -> R<bool>;
}

static POE_WM_NAME: [u8; 13] = *b"Path of Exile";

static IS_ON_COLORS: [Rgb; 5] = [
    Rgb { r: 88, g: 66, b: 42 },
    Rgb { r: 79, g: 59, b: 39 },
    Rgb { r: 84, g: 63, b: 40 },
    Rgb { r: 92, g: 68, b: 42 },
    Rgb { r: 94, g: 65, b: 40 },
];

const STASH_OPEN_POS: Pos = Pos::new(609, 26);
static STASH_OPEN_COLORS: [Rgb; 9] = [
    Rgb { r: 238, g: 209, b: 147 },
    Rgb { r: 244, g: 210, b: 147 },
    Rgb { r: 243, g: 209, b: 146 },
    Rgb { r: 245, g: 201, b: 138 },
    Rgb { r: 240, g: 188, b: 124 },
    Rgb { r: 240, g: 185, b: 114 },
    Rgb { r: 240, g: 182, b: 109 },
    Rgb { r: 239, g: 183, b: 109 },
    Rgb { r: 179, g: 140, b: 81 },
];

/// Vendor in towns
const VENDOR_TOWN_OPEN_POS: Pos = Pos::new(630, 116);
static VENDOR_TOWN_OPEN_COLORS: [Rgb; 14] = [
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 0, g: 0, b: 0 },
    Rgb { r: 198, g: 139, b: 78 },
];

/// Gwennen, Kirac, Lilly
const VENDOR_HIDEOUT_OPEN_POS: Pos = Pos::new(621, 192);
static VENDOR_HIDEOUT_OPEN_COLORS: [Rgb; 13] = [
    Rgb { r: 194, g: 183, b: 140 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
];

/// Heist Vendor
const VENDOR_HEIST_OPEN_POS: Pos = Pos::new(621, 196);
static VENDOR_HEIST_OPEN_COLORS: [Rgb; 11] = [
    Rgb { r: 196, g: 185, b: 142 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
    Rgb { r: 227, g: 215, b: 166 },
];

pub struct Belly;
impl Detect for Belly {
    #[inline]
    fn is_on(&self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(Pos::new(40, 1010), 5)?.iter().eq(IS_ON_COLORS))
    }

    #[inline]
    fn is_poe_window_on(&self, x: &X) -> R<bool> {
        Ok(x.get_window_name_specific_size(POE_WM_NAME.len())?.value.eq(&POE_WM_NAME))
    }

    #[inline]
    fn is_vendor_window_open(&self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(VENDOR_HIDEOUT_OPEN_POS, 13)?
            .iter()
            .zip(VENDOR_HIDEOUT_OPEN_COLORS.iter())
            .all(|(x, y)| x.cmp_by_dist(*y, 5.0).is_lt())
            || x.get_pixels_with_height(VENDOR_TOWN_OPEN_POS, 14)?
                .iter()
                .eq(VENDOR_TOWN_OPEN_COLORS)
            || x.get_pixels_with_height(VENDOR_HEIST_OPEN_POS, 11)?
                .iter()
                .eq(VENDOR_HEIST_OPEN_COLORS))
    }

    #[inline]
    fn is_stash_window_open(&self, x: &X) -> R<bool> {
        Ok(x.get_pixels_with_height(STASH_OPEN_POS, 9)?.iter().eq(STASH_OPEN_COLORS))
    }
}
