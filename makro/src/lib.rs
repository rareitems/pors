mod clipboard;
pub mod error;
pub mod input;
pub mod pointer;
mod types;
pub mod vision;

pub use clipboard::Clipboard;
use error::Error;
pub use evdev::{Device,
                EventStream,
                EventType,
                InputEvent,
                Key};
pub use types::pos::Pos;
pub use types::rgb::{Rgb,
                     VecBgr};

pub type R<T> = Result<T, Error>;

pub struct X {
    conn: x11rb::rust_connection::RustConnection,
    root_window: u32,
}

impl X {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for X {
    fn default() -> Self {
        let (conn, screen_num): (x11rb::rust_connection::RustConnection, usize) =
            x11rb::connect(None).unwrap();
        let screen = &x11rb::connection::Connection::setup(&conn).roots[screen_num];

        let root_window = screen.root;
        let root_width = screen.width_in_pixels;
        let root_height = screen.height_in_pixels;

        assert_eq!(root_width, 1920);
        assert_eq!(root_height, 1080);

        Self { conn, root_window }
    }
}
