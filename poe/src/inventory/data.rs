use makro::{Pos,
            Rgb};

/// Color of red inventory square
pub const UNID: Rgb = Rgb::new(44, 5, 3);

/// Color of empty inventory square
pub const NONE: Rgb = Rgb::new(13, 10, 5);

/// Color of inventory square with something in it
pub const SOME: Rgb = Rgb::new(7, 5, 29);

pub static INV_POS: [Pos; 60] = [
    Pos { x: 1322, y: 589 }, // 0
    Pos { x: 1375, y: 589 }, // 1
    Pos { x: 1428, y: 589 }, // 2
    Pos { x: 1480, y: 589 }, // 3
    Pos { x: 1533, y: 589 }, // 4
    Pos { x: 1586, y: 589 }, // 5
    Pos { x: 1638, y: 589 }, // 6
    Pos { x: 1691, y: 589 }, // 7
    Pos { x: 1744, y: 589 }, // 8
    Pos { x: 1796, y: 589 }, // 9
    Pos { x: 1849, y: 589 }, // 10
    Pos { x: 1902, y: 589 }, // 11
    Pos { x: 1322, y: 642 }, // 12
    Pos { x: 1375, y: 642 }, // 13
    Pos { x: 1428, y: 642 }, // 14
    Pos { x: 1480, y: 642 }, // 15
    Pos { x: 1533, y: 642 }, // 16
    Pos { x: 1586, y: 642 }, // 17
    Pos { x: 1638, y: 642 }, // 18
    Pos { x: 1691, y: 642 }, // 19
    Pos { x: 1744, y: 642 }, // 20
    Pos { x: 1796, y: 642 }, // 21
    Pos { x: 1849, y: 642 }, // 22
    Pos { x: 1902, y: 642 }, // 23
    Pos { x: 1322, y: 694 }, // 24
    Pos { x: 1375, y: 694 }, // 25
    Pos { x: 1428, y: 694 }, // 26
    Pos { x: 1480, y: 694 }, // 27
    Pos { x: 1533, y: 694 }, // 28
    Pos { x: 1586, y: 694 }, // 29
    Pos { x: 1638, y: 694 }, // 30
    Pos { x: 1691, y: 694 }, // 31
    Pos { x: 1744, y: 694 }, // 32
    Pos { x: 1796, y: 694 }, // 33
    Pos { x: 1849, y: 694 }, // 34
    Pos { x: 1902, y: 694 }, // 35
    Pos { x: 1322, y: 747 }, // 36
    Pos { x: 1375, y: 747 }, // 37
    Pos { x: 1428, y: 747 }, // 38
    Pos { x: 1480, y: 747 }, // 39
    Pos { x: 1533, y: 747 }, // 40
    Pos { x: 1586, y: 747 }, // 41
    Pos { x: 1638, y: 747 }, // 42
    Pos { x: 1691, y: 747 }, // 43
    Pos { x: 1744, y: 747 }, // 44
    Pos { x: 1796, y: 747 }, // 45
    Pos { x: 1849, y: 747 }, // 46
    Pos { x: 1902, y: 747 }, // 47
    Pos { x: 1322, y: 800 }, // 48
    Pos { x: 1375, y: 800 }, // 49
    Pos { x: 1428, y: 800 }, // 50
    Pos { x: 1480, y: 800 }, // 51
    Pos { x: 1533, y: 800 }, // 52
    Pos { x: 1586, y: 800 }, // 53
    Pos { x: 1638, y: 800 }, // 54
    Pos { x: 1691, y: 800 }, // 55
    Pos { x: 1744, y: 800 }, // 56
    Pos { x: 1796, y: 800 }, // 57
    Pos { x: 1849, y: 800 }, // 58
    Pos { x: 1902, y: 800 }, // 59
];

// pub const INV_ROWS: [i16; 5] = [589, 642, 694, 747, 800];
pub static INV_POS_COL: [i16; 12] =
    [1322, 1375, 1428, 1480, 1533, 1586, 1638, 1691, 1744, 1796, 1849, 1902];

pub fn inv_pos_get_by_rc(r: usize, c: usize) -> Pos {
    INV_POS[(r - 1) * 12 + (c - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t0() {
        let actual = inv_pos_get_by_rc(1, 1);
        let expected = INV_POS[0];
        assert_eq!(actual, expected);
    }

    #[test]
    fn t1() {
        let actual = inv_pos_get_by_rc(1, 12);
        let expected = INV_POS[11];
        assert_eq!(actual, expected);
    }

    #[test]
    fn t3() {
        let actual = inv_pos_get_by_rc(2, 1);
        let expected = INV_POS[12];
        assert_eq!(actual, expected);
    }

    #[test]
    fn t2() {
        let actual = inv_pos_get_by_rc(5, 12);
        let expected = INV_POS[59];
        assert_eq!(actual, expected);
    }
}
