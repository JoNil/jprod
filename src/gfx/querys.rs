use super::gl;

const QUERY_COUNT: usize = 5000;

pub struct TimerQuerys {
    time_elapsed_1: [u32; QUERY_COUNT],
    time_elapsed_2: [u32; QUERY_COUNT],
    time_stamps_1: [u32; QUERY_COUNT],
    time_stamps_2: [u32; QUERY_COUNT],
}

impl TimerQuerys {
    pub fn new() -> TimerQuerys {

        let mut querys = TimerQuerys {
            time_elapsed_1: [0; QUERY_COUNT],
            time_elapsed_2: [0; QUERY_COUNT],
            time_stamps_1: [0; QUERY_COUNT],
            time_stamps_2: [0; QUERY_COUNT],
        };

        unsafe {
            gl::GenQueries(QUERY_COUNT as i32, querys.time_elapsed_1.as_mut_ptr());
            gl::GenQueries(QUERY_COUNT as i32, querys.time_elapsed_2.as_mut_ptr());
            gl::GenQueries(QUERY_COUNT as i32, querys.time_stamps_1.as_mut_ptr());
            gl::GenQueries(QUERY_COUNT as i32, querys.time_stamps_2.as_mut_ptr());
        }

        querys
    }
}

impl Drop for TimerQuerys {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteQueries(QUERY_COUNT as i32, self.time_elapsed_1.as_mut_ptr());
            gl::DeleteQueries(QUERY_COUNT as i32, self.time_elapsed_2.as_mut_ptr());
            gl::DeleteQueries(QUERY_COUNT as i32, self.time_stamps_1.as_mut_ptr());
            gl::DeleteQueries(QUERY_COUNT as i32, self.time_stamps_2.as_mut_ptr());
        }
    }
}