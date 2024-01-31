use std::time::{SystemTime, UNIX_EPOCH};

pub fn log_method_duration<R>(function: impl FnOnce() -> R) -> R {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let result: R = function();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", end - start);
    result
}
