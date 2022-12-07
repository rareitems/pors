use std::cmp::Ordering;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[inline]
    pub fn dist(self, other: Self) -> f64 {
        let b = f64::from(self.b) - f64::from(other.b);
        let r = f64::from(self.r) - f64::from(other.r);
        let g_pow2 = (f64::from(self.g) - f64::from(other.g)).powi(2);
        r.mul_add(r, b.mul_add(b, g_pow2)).sqrt()
    }

    #[inline]
    pub fn cmp_by_dist(self, other: Self, dist: f64) -> Ordering {
        self.dist(other).partial_cmp(&dist).unwrap()
    }
}

pub struct RgbIntoIter {
    iter: std::vec::IntoIter<u8>,
}

impl Iterator for RgbIntoIter {
    type Item = Rgb;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(b) = self.iter.next() {
            let ret = Some(Rgb {
                b,
                g: unsafe { self.iter.next().unwrap_unchecked() },
                r: unsafe { self.iter.next().unwrap_unchecked() },
            });
            unsafe { self.iter.next().unwrap_unchecked() };
            ret
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.iter.len() as f64 / 4.0).ceil() as usize;
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for RgbIter<'a> {
    fn len(&self) -> usize {
        (self.iter.len() as f64 / 4.0).ceil() as usize
    }
}

pub struct RgbIter<'a> {
    pub iter: std::slice::Iter<'a, u8>,
}

impl<'a> Iterator for RgbIter<'a> {
    type Item = Rgb;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(b) = self.iter.next() {
            let ret = Some(Rgb {
                b: *b,
                g: unsafe { *self.iter.next().unwrap_unchecked() },
                r: unsafe { *self.iter.next().unwrap_unchecked() },
            });
            unsafe { *self.iter.next().unwrap_unchecked() };
            ret
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.iter.len() as f64 / 4.0).ceil() as usize;
        (len, Some(len))
    }
}

/// 'Vector' of 'u8' where each chunk of 4 item represent blue, green, red colors and some
/// padding(?)
#[derive(Debug)]
pub struct VecBgr(pub Vec<u8>);

impl VecBgr {
    #[inline]
    pub fn iter(&self) -> RgbIter {
        RgbIter { iter: self.0.iter() }
    }

    #[inline]
    pub fn to_rgb(&self) -> Vec<Rgb> {
        self.iter().collect()
    }
}

impl VecBgr {
    #[inline]
    pub fn new(data: Vec<u8>) -> Self {
        VecBgr(data)
    }
}

impl IntoIterator for VecBgr {
    type Item = Rgb;
    type IntoIter = RgbIntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RgbIntoIter { iter: self.0.into_iter() }
    }
}

impl<'a> IntoIterator for &'a VecBgr {
    type Item = Rgb;
    type IntoIter = RgbIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        RgbIter { iter: self.0.iter() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmp_by_dist_raw_0() {
        let color = Rgb::new(1, 0, 0);
        let actual = color.cmp_by_dist(Rgb::new(5, 0, 0), 5.0);
        let expected = Ordering::Less;
        assert_eq!(actual, expected);
    }
}
