// impl std::error::Error for Error {}

#[derive(Debug)]
pub enum Error {
    X11ConnectionError(x11rb::rust_connection::ConnectionError),
    X11ReplyError(x11rb::rust_connection::ReplyError),
}

impl From<x11rb::rust_connection::ConnectionError> for Error {
    fn from(err: x11rb::rust_connection::ConnectionError) -> Self {
        Self::X11ConnectionError(err)
    }
}

impl From<x11rb::rust_connection::ReplyError> for Error {
    fn from(err: x11rb::rust_connection::ReplyError) -> Self {
        Self::X11ReplyError(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X11ConnectionError(e) => e.fmt(f),
            Self::X11ReplyError(e) => e.fmt(f),
        }
    }
}
