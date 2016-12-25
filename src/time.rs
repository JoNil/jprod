use win32;

static mut FREQUENCY: i64 = 0;

pub fn now_s() -> f64 {

    if unsafe { FREQUENCY } == 0 {
        unsafe { FREQUENCY = win32::query_performance_frequency(); }
    }

    let time = win32::query_performance_counter();

    time as f64 / unsafe { FREQUENCY } as f64
}