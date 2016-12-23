use core::cell::Cell;
use core::marker::PhantomData;
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

    pub fn get_allocator<'a>(&'a mut self) -> (PoolAllocator<'a>, PoolReborrowToken<'a>) {
        (
            PoolAllocator {
                pool: self,
                offset: 0,
                used: Cell::new(0),
            },
            PoolReborrowToken {
                marker: PhantomData
            }
        )
    }

}

impl Drop for Pool {
    fn drop(&mut self) {

    }
}

pub struct PoolAllocator<'a> {
    pool: &'a Pool,
    offset: usize,
    used: Cell<usize>,
} 

pub struct PoolReborrowToken<'a> {
    marker: PhantomData<&'a mut Pool>
}


impl<'a, 'b> PoolAllocator<'a> {
    pub fn allocate(&'b self, token: &mut PoolReborrowToken, size: usize) -> &'b mut [u8]
        where 'a: 'b
    {

        unsafe { slice::from_raw_parts_mut(ptr::null_mut(), 1) }
    }

    pub fn get_sub_allocator(&'b self, token: &'b mut PoolReborrowToken) -> (PoolAllocator<'b>, PoolReborrowToken<'b>)
        where 'a: 'b
    {
        (
            PoolAllocator {
                pool: self.pool,
                offset: self.offset + self.used.get(),
                used: Cell::new(0),
            },
            PoolReborrowToken {
                marker: PhantomData
            }
        )
    }
}


impl<'a> Drop for PoolAllocator<'a> {
    fn drop(&mut self) {

    }   
}

impl<'a> Drop for PoolReborrowToken<'a> {
    fn drop(&mut self) {

    }   
}

#[test]
fn pool_test() {

    let mut pool = Pool::new(4096);


    {
        let (mut allocator1, mut token) = pool.get_allocator();

        {
            let mut alloc_1 = allocator1.allocate(&mut token, 5);
            let mut alloc_2 = allocator1.allocate(&mut token, 5);

            //let mut alloc_5;

            {
                let (mut sub_alloc_1, mut sub_token) = allocator1.get_sub_allocator(&mut token);

                let mut alloc_3 = sub_alloc_1.allocate(&mut sub_token, 5);
                let mut alloc_4 = sub_alloc_1.allocate(&mut sub_token, 5);
                //alloc_5 = sub_alloc_1.allocate(5);

                let mut alloc_5 = allocator1.allocate(&mut token, 5);
            }
        }
    }

    {
        let (mut allocator2, mut token) = pool.get_allocator();

        {
            let mut alloc_1 = allocator2.allocate(&mut token, 5);
            let mut alloc_2 = allocator2.allocate(&mut token, 5);
        }
    }

}