use c_types::c_void;
use core::cell::Cell;
use core::slice;
use win32;

pub struct Pool {
    memory: *mut u8,
    size: usize,
}

impl Pool {

    pub fn new(size: usize) -> Pool {
        Pool {
            memory: win32::virtual_alloc(size) as *mut u8,
            size: size,
        }
    }

    pub fn get_allocator<'a>(&'a mut self) -> PoolAllocator<'a> {
        PoolAllocator {
            pool: self,
            offset: 0,
            used: Cell::new(0),
            parent: None,
            borrowed: Cell::new(false),
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        win32::virtual_free(self.memory as *mut c_void);
    }
}

pub struct PoolAllocator<'a> {
    pool: &'a Pool,
    offset: usize,
    used: Cell<usize>,
    parent: Option<&'a PoolAllocator<'a>>,
    borrowed: Cell<bool>,
}

impl<'a> PoolAllocator<'a> {
    pub fn allocate(&'a self, size: usize) -> &'a mut [u8] {

        if self.borrowed.get() {
            win32::debug_break();
        }

        let offset = self.offset + self.used.get();

        if offset + size > self.pool.size {
            win32::debug_break();   
        }

        self.used.set(self.used.get() + size);

        unsafe { slice::from_raw_parts_mut(self.pool.memory.offset(offset as isize), size) }
    }

    pub fn get_sub_allocator(&'a self) -> PoolAllocator<'a> {
        
        if self.borrowed.get() {
            win32::debug_break();
        }

        self.borrowed.set(true);

        PoolAllocator {
            pool: self.pool,
            offset: self.offset + self.used.get(),
            used: Cell::new(0),
            borrowed: Cell::new(false),
            parent: Some(self),
        }
    }
}

impl<'a> Drop for PoolAllocator<'a> {
    fn drop(&mut self) {
        if let Some(parent) = self.parent {
            parent.borrowed.set(false);
        }
    }   
}

#[test]
fn pool_test() {

    let mut pool = Pool::new(4096);


    {
        let allocator1 = pool.get_allocator();

        assert_eq!(allocator1.used.get(), 0);

        {
            let alloc_1 = allocator1.allocate(5);
            let alloc_2 = allocator1.allocate(5);

            assert_eq!(allocator1.used.get(), 10);
            assert_eq!(alloc_1.len(), 5);
            assert_eq!(alloc_2.len(), 5);

            //let mut alloc_5; // Should fail to compile

            {
                let sub_alloc_1 = allocator1.get_sub_allocator();

                assert_eq!(allocator1.borrowed.get(), true);
                assert_eq!(sub_alloc_1.used.get(), 0);
                assert_eq!(sub_alloc_1.offset, 10);

                let alloc_3 = sub_alloc_1.allocate(5);
                let alloc_4 = sub_alloc_1.allocate(5);

                assert_eq!(sub_alloc_1.used.get(), 10);
                assert_eq!(alloc_3.len(), 5);
                assert_eq!(alloc_4.len(), 5);

                //alloc_5 = sub_alloc_1.allocate(5); // Should fail to compile

                //let mut alloc_5 = allocator1.allocate(5); // Should panic
                //let mut sub_alloc_2 = allocator1.get_sub_allocator(); // Should panic
            }

            let alloc_5 = allocator1.allocate(5);

            assert_eq!(alloc_5.len(), 5);
        }
    }

    {
        let allocator2 = pool.get_allocator();

        {
            let alloc_1 = allocator2.allocate(5);
            let alloc_2 = allocator2.allocate(5);

            assert_eq!(allocator2.used.get(), 10);
            assert_eq!(alloc_1.len(), 5);
            assert_eq!(alloc_2.len(), 5);
        }
    }

}