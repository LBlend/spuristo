#[macro_export]
macro_rules! or_continue {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(_) => continue,
        }
    };
}
