#[macro_export]
macro_rules! err {
    ($e:expr) => {
        match $e {
            Ok(ok) => ok,
            Err(err) => {
                tracing::error!("{}", err.to_string());
                continue;
            }
        }
    };
}
