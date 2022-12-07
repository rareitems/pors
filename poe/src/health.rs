use makro::{Pos,
            Rgb,
            R,
            X};

type HpPerc = u8;

pub trait DetectHealth {
    fn perc(&self, x: &X) -> R<HpPerc>;
}

static COLORS: [Rgb; 179] = [
    Rgb { r: 41, g: 28, b: 25 },
    Rgb { r: 42, g: 25, b: 23 },
    Rgb { r: 43, g: 24, b: 23 },
    Rgb { r: 46, g: 24, b: 24 },
    Rgb { r: 53, g: 24, b: 24 },
    Rgb { r: 59, g: 26, b: 25 },
    Rgb { r: 63, g: 29, b: 27 },
    Rgb { r: 66, g: 27, b: 25 },
    Rgb { r: 73, g: 29, b: 28 },
    Rgb { r: 85, g: 32, b: 39 },
    Rgb { r: 93, g: 32, b: 39 },
    Rgb { r: 100, g: 32, b: 40 },
    Rgb { r: 104, g: 32, b: 40 },
    Rgb { r: 102, g: 33, b: 40 },
    Rgb { r: 99, g: 32, b: 40 },
    Rgb { r: 103, g: 33, b: 40 },
    Rgb { r: 106, g: 32, b: 40 },
    Rgb { r: 101, g: 34, b: 41 },
    Rgb { r: 104, g: 33, b: 41 },
    Rgb { r: 115, g: 33, b: 41 },
    Rgb { r: 122, g: 33, b: 42 },
    Rgb { r: 125, g: 32, b: 41 },
    Rgb { r: 120, g: 34, b: 41 },
    Rgb { r: 121, g: 33, b: 43 },
    Rgb { r: 116, g: 34, b: 42 },
    Rgb { r: 117, g: 34, b: 41 },
    Rgb { r: 122, g: 33, b: 40 },
    Rgb { r: 117, g: 33, b: 41 },
    Rgb { r: 126, g: 33, b: 41 },
    Rgb { r: 134, g: 31, b: 40 },
    Rgb { r: 135, g: 32, b: 41 },
    Rgb { r: 138, g: 30, b: 40 },
    Rgb { r: 146, g: 37, b: 47 },
    Rgb { r: 151, g: 29, b: 39 },
    Rgb { r: 133, g: 30, b: 38 },
    Rgb { r: 124, g: 31, b: 37 },
    Rgb { r: 132, g: 35, b: 42 },
    Rgb { r: 140, g: 30, b: 40 },
    Rgb { r: 153, g: 30, b: 39 },
    Rgb { r: 146, g: 29, b: 38 },
    Rgb { r: 141, g: 29, b: 39 },
    Rgb { r: 148, g: 29, b: 39 },
    Rgb { r: 167, g: 35, b: 41 },
    Rgb { r: 171, g: 18, b: 35 },
    Rgb { r: 172, g: 20, b: 35 },
    Rgb { r: 173, g: 22, b: 38 },
    Rgb { r: 172, g: 24, b: 40 },
    Rgb { r: 178, g: 19, b: 39 },
    Rgb { r: 181, g: 25, b: 40 },
    Rgb { r: 179, g: 24, b: 40 },
    Rgb { r: 178, g: 24, b: 40 },
    Rgb { r: 182, g: 27, b: 44 },
    Rgb { r: 182, g: 27, b: 44 },
    Rgb { r: 182, g: 25, b: 43 },
    Rgb { r: 174, g: 31, b: 47 },
    Rgb { r: 178, g: 30, b: 47 },
    Rgb { r: 180, g: 27, b: 45 },
    Rgb { r: 181, g: 28, b: 45 },
    Rgb { r: 186, g: 25, b: 46 },
    Rgb { r: 184, g: 29, b: 45 },
    Rgb { r: 184, g: 29, b: 45 },
    Rgb { r: 186, g: 28, b: 45 },
    Rgb { r: 185, g: 26, b: 43 },
    Rgb { r: 183, g: 23, b: 40 },
    Rgb { r: 185, g: 22, b: 41 },
    Rgb { r: 187, g: 21, b: 42 },
    Rgb { r: 184, g: 23, b: 39 },
    Rgb { r: 183, g: 21, b: 38 },
    Rgb { r: 186, g: 19, b: 38 },
    Rgb { r: 185, g: 21, b: 38 },
    Rgb { r: 185, g: 22, b: 37 },
    Rgb { r: 185, g: 19, b: 37 },
    Rgb { r: 187, g: 22, b: 39 },
    Rgb { r: 187, g: 23, b: 40 },
    Rgb { r: 184, g: 22, b: 39 },
    Rgb { r: 183, g: 28, b: 41 },
    Rgb { r: 183, g: 19, b: 40 },
    Rgb { r: 182, g: 23, b: 38 },
    Rgb { r: 176, g: 24, b: 38 },
    Rgb { r: 173, g: 26, b: 39 },
    Rgb { r: 173, g: 20, b: 37 },
    Rgb { r: 179, g: 24, b: 40 },
    Rgb { r: 179, g: 23, b: 40 },
    Rgb { r: 179, g: 21, b: 39 },
    Rgb { r: 178, g: 23, b: 38 },
    Rgb { r: 179, g: 22, b: 40 },
    Rgb { r: 180, g: 25, b: 42 },
    Rgb { r: 179, g: 22, b: 41 },
    Rgb { r: 176, g: 23, b: 40 },
    Rgb { r: 176, g: 30, b: 41 },
    Rgb { r: 174, g: 25, b: 39 },
    Rgb { r: 172, g: 23, b: 37 },
    Rgb { r: 168, g: 26, b: 36 },
    Rgb { r: 166, g: 22, b: 34 },
    Rgb { r: 164, g: 22, b: 34 },
    Rgb { r: 166, g: 24, b: 35 },
    Rgb { r: 164, g: 24, b: 35 },
    Rgb { r: 163, g: 26, b: 34 },
    Rgb { r: 163, g: 21, b: 34 },
    Rgb { r: 161, g: 20, b: 33 },
    Rgb { r: 160, g: 23, b: 34 },
    Rgb { r: 161, g: 28, b: 35 },
    Rgb { r: 159, g: 22, b: 33 },
    Rgb { r: 153, g: 24, b: 34 },
    Rgb { r: 144, g: 25, b: 33 },
    Rgb { r: 143, g: 25, b: 32 },
    Rgb { r: 147, g: 33, b: 32 },
    Rgb { r: 147, g: 21, b: 28 },
    Rgb { r: 146, g: 23, b: 29 },
    Rgb { r: 144, g: 23, b: 28 },
    Rgb { r: 140, g: 21, b: 26 },
    Rgb { r: 137, g: 22, b: 27 },
    Rgb { r: 130, g: 19, b: 26 },
    Rgb { r: 131, g: 21, b: 26 },
    Rgb { r: 129, g: 21, b: 26 },
    Rgb { r: 132, g: 21, b: 26 },
    Rgb { r: 129, g: 21, b: 26 },
    Rgb { r: 127, g: 20, b: 24 },
    Rgb { r: 126, g: 19, b: 23 },
    Rgb { r: 126, g: 17, b: 23 },
    Rgb { r: 126, g: 20, b: 22 },
    Rgb { r: 126, g: 22, b: 23 },
    Rgb { r: 116, g: 19, b: 22 },
    Rgb { r: 106, g: 14, b: 18 },
    Rgb { r: 111, g: 14, b: 19 },
    Rgb { r: 111, g: 14, b: 19 },
    Rgb { r: 109, g: 13, b: 17 },
    Rgb { r: 105, g: 15, b: 20 },
    Rgb { r: 103, g: 12, b: 17 },
    Rgb { r: 103, g: 12, b: 17 },
    Rgb { r: 102, g: 12, b: 16 },
    Rgb { r: 102, g: 12, b: 18 },
    Rgb { r: 104, g: 13, b: 19 },
    Rgb { r: 102, g: 13, b: 19 },
    Rgb { r: 103, g: 13, b: 19 },
    Rgb { r: 102, g: 11, b: 19 },
    Rgb { r: 106, g: 13, b: 19 },
    Rgb { r: 105, g: 14, b: 19 },
    Rgb { r: 104, g: 14, b: 19 },
    Rgb { r: 103, g: 13, b: 19 },
    Rgb { r: 100, g: 13, b: 18 },
    Rgb { r: 100, g: 13, b: 19 },
    Rgb { r: 105, g: 14, b: 20 },
    Rgb { r: 99, g: 12, b: 19 },
    Rgb { r: 94, g: 11, b: 19 },
    Rgb { r: 109, g: 16, b: 20 },
    Rgb { r: 102, g: 14, b: 18 },
    Rgb { r: 108, g: 16, b: 20 },
    Rgb { r: 104, g: 17, b: 21 },
    Rgb { r: 91, g: 13, b: 18 },
    Rgb { r: 89, g: 12, b: 17 },
    Rgb { r: 105, g: 16, b: 20 },
    Rgb { r: 101, g: 15, b: 18 },
    Rgb { r: 97, g: 14, b: 18 },
    Rgb { r: 102, g: 14, b: 18 },
    Rgb { r: 93, g: 12, b: 16 },
    Rgb { r: 91, g: 12, b: 16 },
    Rgb { r: 87, g: 10, b: 17 },
    Rgb { r: 84, g: 12, b: 17 },
    Rgb { r: 82, g: 12, b: 17 },
    Rgb { r: 84, g: 12, b: 17 },
    Rgb { r: 85, g: 13, b: 18 },
    Rgb { r: 83, g: 13, b: 18 },
    Rgb { r: 81, g: 13, b: 19 },
    Rgb { r: 75, g: 13, b: 19 },
    Rgb { r: 73, g: 14, b: 18 },
    Rgb { r: 71, g: 15, b: 17 },
    Rgb { r: 66, g: 14, b: 18 },
    Rgb { r: 66, g: 14, b: 17 },
    Rgb { r: 70, g: 15, b: 19 },
    Rgb { r: 73, g: 15, b: 19 },
    Rgb { r: 74, g: 15, b: 18 },
    Rgb { r: 78, g: 16, b: 19 },
    Rgb { r: 74, g: 16, b: 19 },
    Rgb { r: 67, g: 15, b: 19 },
    Rgb { r: 69, g: 16, b: 20 },
    Rgb { r: 89, g: 21, b: 22 },
    Rgb { r: 91, g: 22, b: 21 },
    Rgb { r: 76, g: 19, b: 21 },
];

pub struct HpByColorsArray;

const POS: Pos = Pos::new(105, 871);

/// let v = x.get_pixels_with_height((105, 871), 179).unwrap().to_rgb();
/// assert_eq!(Rgb::new(41, 28, 25), a[0]);
/// assert_eq!(Rgb::new(76, 19, 21), a[178]);
/// 102 871
/// y   q
/// 13  1048
impl DetectHealth for HpByColorsArray {
    fn perc(&self, x: &X) -> R<HpPerc> {
        // TODO: Benchmark
        let cmp = x.get_pixels_with_height(POS, 179)?.iter().enumerate().find(|(i, x)| {
            x.cmp_by_dist(
                unsafe {
                    debug_assert!(COLORS.get(*i).is_some());
                    *COLORS.get_unchecked(*i)
                },
                1.0,
            )
            .is_lt()
        });

        // let cmp = x
        //     .get_pixels_with_height((105, 871), 179)?
        //     .iter()
        //     .zip(COLORS.iter())
        //     .enumerate()
        //     .find(|(_, (x, y))| x.cmp_by_dist(**y, 0.5).is_le());

        match cmp {
            Some((i, _)) => {
                if i >= 100 {
                    Ok((104.0 - (i as f64 * 0.5)) as u8)
                } else {
                    Ok((102.0 - (i as f64 * 0.5)) as u8)
                }
            }
            None => Ok(13),
        }
    }
}
