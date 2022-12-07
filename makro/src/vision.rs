use x11rb::protocol::xproto::ConnectionExt;

use super::{VecBgr,
            R,
            X};
use crate::{Pos,
            Rgb};

impl X {
    /// Gets the color of pixels from (x, y) to (x - 1 + width, y - 1 height), 0-indexed
    pub fn get_pixels<T: Into<Pos>>(&self, pos: T, width: u16, height: u16) -> R<VecBgr> {
        let pos = pos.into();
        Ok(VecBgr::new(
            self.conn
                .get_image(
                    x11rb::protocol::xproto::ImageFormat::Z_PIXMAP,
                    self.root_window,
                    pos.x,
                    pos.y,
                    width,
                    height,
                    !0,
                )?
                .reply()?
                .data,
        ))
    }

    /// Gets the color of pixel at (x, y), 0-indexed
    pub fn get_pixel<T: Into<Pos>>(&self, pos: T) -> R<Rgb> {
        let rgb = self.get_pixels(pos, 1, 1)?.0;

        unsafe { Ok(Rgb::new(*rgb.get_unchecked(2), *rgb.get_unchecked(1), *rgb.get_unchecked(0))) }
    }

    /// Gets the color of pixel at (x, y) to (x, y + height - 1), 0-indexed
    pub fn get_pixels_with_height<T: Into<Pos>>(&self, pos: T, height: u16) -> R<VecBgr> {
        self.get_pixels(pos, 1, height)
    }

    /// Gets the color of pixel at (x, y) to (x + width - 1, y), 0-indexed
    pub fn get_pixels_with_width<T: Into<Pos>>(&self, pos: T, width: u16) -> R<VecBgr> {
        self.get_pixels(pos, width, 1)
    }

    /// Retrieves window name of the currently focused window with maximum of 100 letters
    pub fn get_window_name(&self) -> R<WindowName> {
        let focus_window = x11rb::protocol::xproto::get_input_focus(&self.conn)?.reply()?.focus;

        let value = x11rb::protocol::xproto::get_property(
            &self.conn,
            false,
            focus_window,
            x11rb::protocol::xproto::AtomEnum::WM_NAME,
            x11rb::protocol::xproto::AtomEnum::STRING,
            0,
            25,
        )?
        .reply()?
        .value;

        Ok(WindowName { value })
    }

    /// Retrieves window name with maximum of `size` of letters
    pub fn get_window_name_specific_size(&self, size: usize) -> R<WindowName> {
        let focus_window = x11rb::protocol::xproto::get_input_focus(&self.conn)?.reply()?.focus;

        let value = x11rb::protocol::xproto::get_property(
            &self.conn,
            false,
            focus_window,
            x11rb::protocol::xproto::AtomEnum::WM_NAME,
            x11rb::protocol::xproto::AtomEnum::STRING,
            0,
            ((size * 8) as f32 / 32.0).ceil() as u32, /* this function retrieves this ammount of
                                                       * mutliples of 32bit data */
        )?
        .reply()?
        .value;

        Ok(WindowName { value })
    }
}

#[derive(Debug)]
pub struct WindowName {
    pub value: Vec<u8>,
}

impl std::fmt::Display for WindowName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.iter().map(|x| *x as char).collect::<String>().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serial_test::serial;

    use super::super::*;

    #[test]
    #[serial]
    fn test_specific_pixel() {
        let m = X::new();
        let shell = xshell::Shell::new().unwrap();
        xshell::cmd!(shell, "test_data/vision/x20_y10_w105_h95_101_102_103.sh").run().unwrap();
        std::thread::sleep(Duration::from_millis(500));
        let pixel = m.get_pixel((20, 10)).unwrap();

        xshell::cmd!(shell, "pkill feh").run().unwrap();

        assert_eq!(pixel, Rgb::new(101, 102, 103));
    }

    #[test]
    #[serial]
    fn test_specific_pixel_1() {
        let m = X::new();
        let shell = xshell::Shell::new().unwrap();
        xshell::cmd!(shell, "test_data/vision/test_all_100_100_100.sh").run().unwrap();
        std::thread::sleep(Duration::from_millis(500));
        let pixel = m.get_pixel((1919, 1079)).unwrap();
        xshell::cmd!(shell, "pkill feh").run().unwrap();
        assert_eq!(pixel, Rgb::new(100, 100, 100));
    }

    #[test]
    #[serial]
    fn test_all_100_100_100() {
        let m = X::new();
        let shell = xshell::Shell::new().unwrap();
        xshell::cmd!(shell, "test_data/vision/test_all_100_100_100.sh").run().unwrap();
        std::thread::sleep(Duration::from_millis(500));
        let colors = m.get_pixels((0, 0), 1920, 1080).unwrap();
        let mut ass = true;
        for c in &colors {
            if !c.eq(&Rgb::new(100, 100, 100)) {
                ass = false;
                break;
            }
        }
        xshell::cmd!(shell, "pkill feh").run().unwrap();
        assert!(ass);
    }

    #[test]
    #[serial]
    fn test_with_width() {
        let m = X::new();
        let shell = xshell::Shell::new().unwrap();
        xshell::cmd!(shell, "test_data/vision/test_all_100_100_100.sh").run().unwrap();
        std::thread::sleep(Duration::from_millis(500));
        let colors = m.get_pixels_with_width((0, 0), 1920).unwrap();
        let mut ass = true;
        for c in &colors {
            if !c.eq(&Rgb::new(100, 100, 100)) {
                ass = false;
                break;
            }
        }
        let size = colors.iter().fold(0, |acc, _| acc + 1);
        assert_eq!(size, 1920);
        xshell::cmd!(shell, "pkill feh").run().unwrap();
        assert!(ass);
    }

    #[test]
    #[serial]
    fn test_with_height() {
        let m = X::new();
        let shell = xshell::Shell::new().unwrap();
        xshell::cmd!(shell, "test_data/vision/test_all_100_100_100.sh").run().unwrap();
        std::thread::sleep(Duration::from_millis(500));
        let colors = m.get_pixels_with_height((0, 0), 1080).unwrap();
        let mut ass = true;
        for c in &colors {
            if !c.eq(&Rgb::new(100, 100, 100)) {
                ass = false;
                break;
            }
        }
        xshell::cmd!(shell, "pkill feh").run().unwrap();
        let size = colors.iter().fold(0, |acc, _| acc + 1);
        assert_eq!(size, 1080);
        assert!(ass);
    }

    #[test]
    #[serial]
    fn test_specific_position() {
        let m = X::new();
        let shell = xshell::Shell::new().unwrap();

        xshell::cmd!(shell, "test_data/vision/x20_y10_w105_h95_101_102_103.sh").run().unwrap();
        std::thread::sleep(Duration::from_millis(500));
        let colors = m.get_pixels((20, 10), 105, 95).unwrap();

        let mut ass = true;
        for c in &colors {
            if !c.eq(&Rgb::new(101, 102, 103)) {
                ass = false;
                break;
            }
        }

        xshell::cmd!(shell, "pkill feh").run().unwrap();
        assert!(ass);
    }

    #[test]
    #[serial]
    fn test_wm_name() {
        let s = "feh [1 of 1] - test_data/vision/all_100_100_100.png".to_string();
        let s_as_bytes = s.as_bytes();

        let m = X::new();
        let shell = xshell::Shell::new().unwrap();
        xshell::cmd!(shell, "test_data/vision/test_all_100_100_100.sh").run().unwrap();
        std::thread::sleep(Duration::from_millis(500));
        let wm_name = m.get_window_name().unwrap();
        xshell::cmd!(shell, "pkill feh").run().unwrap();
        assert!(dbg!(wm_name.to_string()).eq(&s));
        assert!(wm_name.value.eq(s_as_bytes));
    }

    #[test]
    #[serial]
    fn test_wm_name_specific_size() {
        let s = "feh [1 of 1] - test_data/vision/all_100_100_100.png".to_string();
        let s_as_bytes = s.as_bytes();

        let m = X::new();
        let shell = xshell::Shell::new().unwrap();

        xshell::cmd!(shell, "test_data/vision/test_all_100_100_100.sh").run().unwrap();
        std::thread::sleep(Duration::from_millis(500));
        let wm_name = m.get_window_name_specific_size(s.len()).unwrap();
        xshell::cmd!(shell, "pkill feh").run().unwrap();
        assert!(dbg!(wm_name.to_string()).eq(&s));
        assert!(wm_name.value.eq(s_as_bytes));
    }
}
