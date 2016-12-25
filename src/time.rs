use win32;

lazy_static! {
    static ref FREQUENCY: i64 = win32::query_performance_frequency();
}

pub fn now_s() -> f64 {
    let time = win32::query_performance_counter();

    time as f64 / *FREQUENCY as f64
}