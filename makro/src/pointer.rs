use x11rb::protocol::xproto::ConnectionExt;

use crate::{Pos,
            R,
            X};

impl X {
    /// Moves the mouse to the 'pos'
    #[inline]
    pub fn move_mouse<T: Into<Pos>>(&self, pos: T) -> R<()> {
        let pos = pos.into();

        self.conn
            .warp_pointer(self.root_window, self.root_window, 0, 0, 1920, 1080, pos.x, pos.y)?
            .check()?;

        Ok(())
    }

    /// Query the position of the mouse
    pub fn query_mouse(&self) -> R<Pos> {
        let a = self.conn.query_pointer(self.root_window)?.reply()?;

        Ok(Pos { x: a.root_x, y: a.root_y })
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::super::*;

    #[test]
    #[serial]
    fn mouse_0() {
        let m = X::new();
        m.move_mouse(Pos::new(100, 100)).unwrap();
        assert_eq!(m.query_mouse().unwrap(), Pos::new(100, 100));
    }

    #[test]
    #[serial]
    fn mouse_1() {
        let m = X::new();
        m.move_mouse(Pos::new(1919, 1079)).unwrap();
        assert_eq!(m.query_mouse().unwrap(), Pos::new(1919, 1079));
    }
}
