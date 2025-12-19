use crate::{c_types::c_void, utils, win32};
use core::{marker::PhantomData, mem, ptr, slice};

const OFFSET_MASK: u64 = 0x0000_ffff_ffff_ffff;
const GENERATION_MASK: u64 = 0x00ff_0000_0000_0000;
const ARENA_ID_MASK: u64 = 0xff00_0000_0000_0000;

pub struct AllocationToken<T> {
    token: u64,
    _marker: PhantomData<T>,
}

impl<T> AllocationToken<T> {
    fn generation(&self) -> u8 {
        ((self.token & GENERATION_MASK) >> 48) as u8
    }

    fn arena_id(&self) -> u8 {
        ((self.token & ARENA_ID_MASK) >> 56) as u8
    }
}

pub struct AllocationArrayToken<T: ?Sized> {
    token: u64,
    size: usize,
    _marker: PhantomData<T>,
}

impl<T> AllocationArrayToken<T> {
    fn generation(&self) -> u8 {
        ((self.token & GENERATION_MASK) >> 48) as u8
    }

    fn arena_id(&self) -> u8 {
        ((self.token & ARENA_ID_MASK) >> 56) as u8
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct Pool {
    base: *mut u8,
    size: u64,
    used: u64,
    arena_id: u8,
    generation_id: u8,
}

impl Pool {
    pub fn new(size: u64, arena_id: u8) -> Pool {
        let base = win32::virtual_alloc(size as _) as *mut u8;
        utils::assert(!base.is_null());
        assert!(size < OFFSET_MASK);
        Pool {
            base,
            size,
            used: 0,
            arena_id,
            generation_id: 0,
        }
    }

    pub fn allocate<T: Copy>(&mut self, default: T) -> AllocationToken<T> {
        let size = mem::size_of::<T>() as u64;
        let align = mem::align_of::<T>() as u64;
        let aligned_offset = (self.used + align - 1) & !(align - 1);
        assert!(aligned_offset + size <= self.size);
        self.used = aligned_offset + size;

        let result = unsafe { self.base.add(aligned_offset as usize) };
        unsafe { ptr::write(result as *mut T, default) };

        AllocationToken {
            token: (aligned_offset & OFFSET_MASK)
                | ((self.arena_id as u64) << 56)
                | ((self.generation_id as u64) << 48),
            _marker: PhantomData,
        }
    }

    pub fn allocate_array<T: Copy>(&mut self, size: usize, default: T) -> AllocationArrayToken<T> {
        let byte_size = mem::size_of::<T>() as u64 * size as u64;
        let align = mem::align_of::<T>() as u64;
        let aligned_offset = (self.used + align - 1) & !(align - 1);
        assert!(aligned_offset + byte_size <= self.size);
        self.used = aligned_offset + byte_size;

        {
            let ptr = unsafe { self.base.add(aligned_offset as usize) } as *mut T;
            for i in 0..size {
                unsafe { ptr::write(ptr.add(i), default) };
            }
        }

        AllocationArrayToken {
            token: (aligned_offset & OFFSET_MASK)
                | ((self.arena_id as u64) << 56)
                | ((self.generation_id as u64) << 48),
            size,
            _marker: PhantomData,
        }
    }

    pub fn clean(&mut self) {
        self.used = 0;
        self.generation_id = self.generation_id.wrapping_add(1);
    }

    pub fn borrow<'a, T>(&'a self, token: &'a AllocationToken<T>) -> &'a T {
        assert_eq!(token.generation(), self.generation_id);
        assert_eq!(token.arena_id(), self.arena_id);

        let ptr = unsafe { self.base.add((token.token & OFFSET_MASK) as usize) } as *const T;
        unsafe { &*ptr }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn borrow_mut<'a, T>(&'a self, token: &'a mut AllocationToken<T>) -> &'a mut T {
        assert_eq!(token.generation(), self.generation_id);
        assert_eq!(token.arena_id(), self.arena_id);

        let ptr = unsafe { self.base.add((token.token & OFFSET_MASK) as usize) } as *mut T;
        unsafe { &mut *ptr }
    }

    pub fn borrow_slice<'a, T>(&'a self, token: &'a AllocationArrayToken<T>) -> &'a [T] {
        assert_eq!(token.generation(), self.generation_id);
        assert_eq!(token.arena_id(), self.arena_id);

        let ptr = unsafe { self.base.add((token.token & OFFSET_MASK) as usize) } as *const T;
        unsafe { slice::from_raw_parts(ptr, token.size) }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn borrow_slice_mut<'a, T>(
        &'a self,
        token: &'a mut AllocationArrayToken<T>,
    ) -> &'a mut [T] {
        assert_eq!(token.generation(), self.generation_id);
        assert_eq!(token.arena_id(), self.arena_id);

        let ptr = unsafe { self.base.add((token.token & OFFSET_MASK) as usize) } as *mut T;
        unsafe { slice::from_raw_parts_mut(ptr, token.size) }
    }

    pub fn try_borrow<'a, T>(&'a self, token: &'a AllocationToken<T>) -> Option<&'a T> {
        if token.generation() == self.generation_id && token.arena_id() == self.arena_id {
            let ptr = unsafe { self.base.add((token.token & OFFSET_MASK) as usize) } as *const T;
            Some(unsafe { &*ptr })
        } else {
            None
        }
    }

    pub fn try_borrow_mut<'a, T>(&'a self, token: &'a mut AllocationToken<T>) -> Option<&'a mut T> {
        if token.generation() == self.generation_id && token.arena_id() == self.arena_id {
            let ptr = unsafe { self.base.add((token.token & OFFSET_MASK) as usize) } as *mut T;
            Some(unsafe { &mut *ptr })
        } else {
            None
        }
    }

    pub fn try_borrow_slice<'a, T>(
        &'a self,
        token: &'a AllocationArrayToken<T>,
    ) -> Option<&'a [T]> {
        if token.generation() == self.generation_id && token.arena_id() == self.arena_id {
            let ptr = unsafe { self.base.add((token.token & OFFSET_MASK) as usize) } as *const T;
            Some(unsafe { slice::from_raw_parts(ptr, token.size) })
        } else {
            None
        }
    }

    pub fn try_borrow_slice_mut<'a, T>(
        &'a self,
        token: &'a mut AllocationArrayToken<T>,
    ) -> Option<&'a mut [T]> {
        if token.generation() == self.generation_id && token.arena_id() == self.arena_id {
            let ptr = unsafe { self.base.add((token.token & OFFSET_MASK) as usize) } as *mut T;
            Some(unsafe { slice::from_raw_parts_mut(ptr, token.size) })
        } else {
            None
        }
    }
}

impl Drop for Pool {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            win32::virtual_free(self.base as *mut c_void);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_allocation() {
        let mut arena = Pool::new(1024, 0);

        let mut token = arena.allocate::<u32>(0);
        *arena.borrow_mut(&mut token) = 42;
        assert_eq!(*arena.borrow(&token), 42);

        let mut token2 = arena.allocate::<f64>(0.0);
        *arena.borrow_mut(&mut token2) = core::f64::consts::PI;
        assert_eq!(*arena.borrow(&token2), core::f64::consts::PI);
    }

    #[test]
    fn array_allocation() {
        let mut arena = Pool::new(1024, 0);

        let mut token = arena.allocate_array::<u32>(4, 0);
        let slice = arena.borrow_slice_mut(&mut token);
        assert_eq!(slice, &[0, 0, 0, 0]);

        slice[0] = 1;
        slice[1] = 2;
        slice[2] = 3;
        slice[3] = 4;

        assert_eq!(arena.borrow_slice(&token), &[1, 2, 3, 4]);
    }

    #[test]
    fn cleaning() {
        let mut arena = Pool::new(1024, 0);

        let mut token = arena.allocate::<u32>(0);
        *arena.borrow_mut(&mut token) = 42;
        assert_eq!(*arena.borrow(&token), 42);

        arena.clean();

        let token2 = arena.allocate::<u32>(0);
        assert_eq!(*arena.borrow(&token2), 0);
    }

    #[test]
    #[should_panic]
    fn borrow_after_clean() {
        let mut arena = Pool::new(1024, 0);

        let mut token = arena.allocate::<u32>(0);
        *arena.borrow_mut(&mut token) = 42;

        arena.clean();

        // This should panic because the token is from a previous generation
        arena.borrow(&token);
    }

    #[test]
    fn try_borrow_after_clean() {
        let mut arena = Pool::new(1024, 0);

        let mut token = arena.allocate::<u32>(0);
        *arena.try_borrow_mut(&mut token).unwrap() = 42;

        arena.clean();

        // This should return None because the token is from a previous generation
        assert!(arena.try_borrow(&token).is_none());
    }
}
