#![allow(dead_code)]

use c_types::c_void;
use core::cell::Cell;
use core::mem;
use core::slice;
use utils;
use win32;

pub struct Pool {
    memory: *mut u8,
    size: usize,
}

impl Pool {

    pub fn new(size: usize) -> Pool {

        let memory = win32::virtual_alloc(size) as *mut u8;

        utils::assert(memory.is_null());

        Pool {
            memory: memory,
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
    pub fn allocate_byte_slice(&'a self, size: usize) -> &'a mut [u8] {

        utils::assert(self.borrowed.get());

        let offset = self.offset + self.used.get();

        utils::assert(offset + size > self.pool.size);

        self.used.set(self.used.get() + size);

        let buffer = unsafe { slice::from_raw_parts_mut(self.pool.memory.offset(offset as isize), size) };

        for byte in buffer.iter_mut() {
            *byte = 0;
        }

        buffer
    }

    pub fn allocate<T: Copy>(&'a self) -> &'a mut T {

        let size = mem::size_of::<T>();

        let buffer = self.allocate_byte_slice(size);

        unsafe { mem::transmute(&mut *buffer.get_unchecked_mut(0)) }
    }

    pub fn allocate_slice<T: Copy>(&'a self, count: usize) -> &'a mut [T] {

        let size = mem::size_of::<T>();

        let buffer = self.allocate_byte_slice(count * size);

        unsafe { slice::from_raw_parts_mut(&mut *buffer.get_unchecked_mut(0) as *mut u8 as *mut _, buffer.len() / size) }
    }

    pub fn get_sub_allocator(&'a self) -> PoolAllocator<'a> {
        
        utils::assert(self.borrowed.get());

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
            let alloc_1 = allocator1.allocate_slice::<u8>(5);
            let alloc_2 = allocator1.allocate_slice::<u8>(5);

            assert_eq!(allocator1.used.get(), 10);
            assert_eq!(alloc_1.len(), 5);
            assert_eq!(alloc_2.len(), 5);

            //let mut alloc_5; // Should fail to compile

            {
                let sub_alloc_1 = allocator1.get_sub_allocator();

                assert_eq!(allocator1.borrowed.get(), true);
                assert_eq!(sub_alloc_1.used.get(), 0);
                assert_eq!(sub_alloc_1.offset, 10);

                let alloc_3 = sub_alloc_1.allocate_slice::<u8>(5);
                let alloc_4 = sub_alloc_1.allocate_slice::<u8>(5);

                assert_eq!(sub_alloc_1.used.get(), 10);
                assert_eq!(alloc_3.len(), 5);
                assert_eq!(alloc_4.len(), 5);

                //alloc_5 = sub_alloc_1.allocate(5); // Should fail to compile

                //let mut alloc_5 = allocator1.allocate(5); // Should panic
                //let mut sub_alloc_2 = allocator1.get_sub_allocator(); // Should panic
            }

            let alloc_5 = allocator1.allocate_slice::<u8>(5);

            assert_eq!(alloc_5.len(), 5);
        }
    }

    {
        let allocator2 = pool.get_allocator();

        {
            let alloc_1 = allocator2.allocate_slice::<u8>(5);
            let alloc_2 = allocator2.allocate_slice::<u8>(5);

            assert_eq!(allocator2.used.get(), 10);
            assert_eq!(alloc_1.len(), 5);
            assert_eq!(alloc_2.len(), 5);
        }
    }

}