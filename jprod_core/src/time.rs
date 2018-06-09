use win32;

static mut FREQUENCY: i64 = 0;

#[inline]
pub fn now_s() -> f64 {

    if unsafe { FREQUENCY } == 0 {
        unsafe { FREQUENCY = win32::query_performance_frequency(); }
    }

    let time = win32::query_performance_counter();

    time as f64 / unsafe { FREQUENCY } as f64
}

#[allow(unused_mut)]
#[inline]
pub fn rdtsc() -> u64 {
    let mut low: u32;
    let mut high: u32;

    unsafe {
        asm!("rdtsc" : "={eax}" (low), "={edx}" (high));
    }
    
    ((high as u64) << 32) | (low as u64)
}