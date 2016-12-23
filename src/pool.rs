use core::cell::Cell;
use core::ptr;
use core::slice;

pub struct Pool {
    memory: *mut u8,
    size: usize,
}

impl Pool {

    pub fn new(size: usize) -> Pool {
        Pool {
            memory: ptr::null_mut(),
            size: 0,
        }
    }

    pub fn get_allocator<'a>(&'a mut self) -> PoolAllocator<'a> {
        PoolAllocator {
            pool: self,
            offset: 0,
            used: Cell::new(0),
        }
    }

}

impl Drop for Pool {
    fn drop(&mut self) {

    }
}

pub struct PoolAllocator<'a> {
    pool: &'a mut Pool,
    offset: usize,
    used: Cell<usize>,
} 


impl<'a, 'b> PoolAllocator<'a> {
    pub fn allocate(&'b self, size: usize) -> &'b mut [u8]
        where 'a: 'b
    {

        unsafe { slice::from_raw_parts_mut(ptr::null_mut(), 1) }
    }

    pub fn get_sub_allocator(&'b mut self) -> PoolAllocator<'b>
        where 'a: 'b
    {
        PoolAllocator {
            pool: self.pool,
            offset: self.offset + self.used.get(),
            used: Cell::new(0),
        }
    }
}


impl<'a> Drop for PoolAllocator<'a> {
    fn drop(&mut self) {

    }   
}

#[test]
fn pool_test() {

    let mut pool = Pool::new(4096);


    {
        let mut allocator1 = pool.get_allocator();

        {
            //let mut alloc_1 = allocator1.allocate(5);
            //let mut alloc_2 = allocator1.allocate(5);

            //let mut alloc_5;

            {
                let mut sub_alloc_1 = allocator1.get_sub_allocator();

                let mut alloc_3 = sub_alloc_1.allocate(5);
                let mut alloc_4 = sub_alloc_1.allocate(5);
                //alloc_5 = sub_alloc_1.allocate(5);
            }
        }
    }

    {
        let mut allocator2 = pool.get_allocator();

        {
            let mut alloc_1 = allocator2.allocate(5);
            let mut alloc_2 = allocator2.allocate(5);
        }
    }

}