#[macro_export]
macro_rules! or_continue {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(_) => {
                println!("[or_continue!] Skipped due to error");
                continue;
            }
        }
    };
}
