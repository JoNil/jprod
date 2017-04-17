use core::cell::Cell;
use core::f64;
use core::marker::PhantomData;
use math;
use super::Context;
use super::gl;
use time;
use utils;

const QUERY_COUNT: usize = 256;

pub struct QueryManager {
    marker: PhantomData<*const u32>,

    time_elapsed_1: [u32; QUERY_COUNT],
    time_elapsed_2: [u32; QUERY_COUNT],
    time_stamps_1: [u32; QUERY_COUNT],
    time_stamps_2: [u32; QUERY_COUNT],

    next_query: Cell<i64>,
    last_query_count: i64,

    is_tick: bool,

    now: u64,
    last: u64,
    second_last: u64,

    now_real: u64,
    last_real: u64,
    second_last_real: u64,
}

impl QueryManager {
    #[inline]
    pub fn new(_: &Context) -> QueryManager {

        let mut querys = QueryManager {

            marker: PhantomData,

            time_elapsed_1: [0; QUERY_COUNT],
            time_elapsed_2: [0; QUERY_COUNT],
            time_stamps_1: [0; QUERY_COUNT],
            time_stamps_2: [0; QUERY_COUNT],

            next_query: Cell::new(0),
            last_query_count: 0,

            is_tick: false,

            now: 0,
            last: 0,
            second_last: 0,

            now_real: 0,
            last_real: 0,
            second_last_real: 0,
        };

        unsafe {
            gl::GenQueries(QUERY_COUNT as i32, querys.time_elapsed_1.as_mut_ptr());
            gl::GenQueries(QUERY_COUNT as i32, querys.time_elapsed_2.as_mut_ptr());
            gl::GenQueries(QUERY_COUNT as i32, querys.time_stamps_1.as_mut_ptr());
            gl::GenQueries(QUERY_COUNT as i32, querys.time_stamps_2.as_mut_ptr());
        }

        querys
    }

    #[inline]
    pub fn query<'a>(&'a self) -> Query<'a> {

        let query = Query {
            manager: self,
            index: self.next_query.get()
        };

        self.next_query.set(query.index + 1);

        utils::assert(query.index < QUERY_COUNT as i64);

        unsafe {
            if self.is_tick {
                gl::QueryCounter(*self.time_stamps_1.get_unchecked(query.index as usize), gl::TIMESTAMP);
                gl::BeginQuery(gl::TIME_ELAPSED, *self.time_elapsed_1.get_unchecked(query.index as usize));
            } else {
                gl::QueryCounter(*self.time_stamps_2.get_unchecked(query.index as usize), gl::TIMESTAMP);
                gl::BeginQuery(gl::TIME_ELAPSED, *self.time_elapsed_2.get_unchecked(query.index as usize));
            }
        }

        query
    }

    #[inline]
    pub fn submit_zones(&mut self) {

        let mut time_stamps: [u64; QUERY_COUNT] = [0; QUERY_COUNT];
        let mut time_elapsed: [u64; QUERY_COUNT] = [0; QUERY_COUNT];

        for i in 0..self.last_query_count {
            unsafe {
                if self.is_tick {
                    gl::GetQueryObjectui64v(*self.time_stamps_2.get_unchecked(i as usize), gl::QUERY_RESULT, time_stamps.get_unchecked_mut(i as usize) as *mut _);
                    gl::GetQueryObjectui64v(*self.time_elapsed_2.get_unchecked(i as usize), gl::QUERY_RESULT, time_elapsed.get_unchecked_mut(i as usize) as *mut _);

                } else {
                    gl::GetQueryObjectui64v(*self.time_stamps_1.get_unchecked(i as usize), gl::QUERY_RESULT, time_stamps.get_unchecked_mut(i as usize) as *mut _);
                    gl::GetQueryObjectui64v(*self.time_elapsed_1.get_unchecked(i as usize), gl::QUERY_RESULT, time_elapsed.get_unchecked_mut(i as usize) as *mut _);
                }
            }
        }

        self.second_last = self.last;
        self.last = self.now;
        self.now = time::rdtsc();

        self.second_last_real = self.last_real;
        self.last_real = self.now_real;
        {
            let mut now: i64 = 0;
            unsafe { gl::GetInteger64v(gl::TIMESTAMP, &mut now as *mut _) };
            self.now_real = now as u64;
        }

        let period = self.now - self.last;
        let real_period = self.now_real - self.last_real;

        let scale = period as f64 / real_period as f64;

        for i in 0..self.last_query_count {

            let zone_duraton = unsafe { ((*time_elapsed.get_unchecked(i as usize) as f64) * scale) as u64 };
            let zone_start = unsafe { ((self.second_last as f64) + math::max_f64((*time_stamps.get_unchecked(i as usize) as f64) - (self.second_last_real as f64), 0.0) * scale) as u64 };
            let zone_end = zone_start + zone_duraton;

            tm_zone_at_time!(
                    "Gpu",
                    7,
                    zone_start,
                    zone_end,
                    1);
        }

        self.is_tick = !self.is_tick;
        self.last_query_count = self.next_query.get();
        self.next_query.set(0);

    }
}

impl Drop for QueryManager {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteQueries(QUERY_COUNT as i32, self.time_elapsed_1.as_mut_ptr());
            gl::DeleteQueries(QUERY_COUNT as i32, self.time_elapsed_2.as_mut_ptr());
            gl::DeleteQueries(QUERY_COUNT as i32, self.time_stamps_1.as_mut_ptr());
            gl::DeleteQueries(QUERY_COUNT as i32, self.time_stamps_2.as_mut_ptr());
        }
    }
}

pub struct Query<'a> {
    manager: &'a QueryManager,
    index: i64,
}

impl<'a> Drop for Query<'a> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if self.manager.is_tick {
                gl::EndQuery(gl::TIME_ELAPSED);
            } else {
                gl::EndQuery(gl::TIME_ELAPSED);
            }
        }
    }
}