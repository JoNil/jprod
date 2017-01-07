use pool::PoolAllocator;
use win32::types::*;
use win32;

pub struct File {
	handle: Handle,
}

impl File {
	pub fn open(file_name: &[u8]) -> Option<File> {

		let handle = win32::open_file(file_name);

		if handle == INVALID_HANDLE_VALUE {
			return None;
		}

		Some(File {
			handle: handle,
		})
	}

	pub fn read_entire_file<'a>(&self, allocator: &'a PoolAllocator<'a>) -> &'a mut [u8] {

		let size = win32::get_file_size(self.handle);

		let buffer = allocator.allocate_byte_slice(size as usize);

		win32::read_file(self.handle, buffer);

		buffer
	}
}


impl Drop for File {
	fn drop(&mut self) {
		win32::close_file(self.handle);
	}
}