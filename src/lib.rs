use std::ptr::NonNull;
use mem_marco::impl_with_ptr;

pub struct MemBuffer {
    pub mem: *mut u8,
    ptr: NonNull<u8>,
    len: usize,
}

pub struct MemView<T> {
    ptr: NonNull<T>,
}

pub enum MemOffset {
    TypeOffset(usize),
    ByteOffset(usize),
}

#[derive(Debug)]
pub enum MemErr {
    OutOfRange
}

use MemErr::*;

impl MemBuffer {
    pub fn from_size(size: usize) -> MemBuffer {
        let mut vec = vec![0u8; size];
        let bytes = vec.as_mut_slice();
        MemBuffer::from_bytes(bytes)
    }

    pub fn from_bytes(bytes: &mut [u8]) -> MemBuffer {
        let mut_ptr = bytes.as_mut_ptr();
        let size = size_of_val(bytes);
        MemBuffer {
            mem: mut_ptr,
            ptr: NonNull::new(mut_ptr).unwrap(),
            len: size,
        }
    }

    pub fn get_ptr<T>(&self) -> Result<NonNull<T>, MemErr> {
        let end = size_of::<T>();
        if end > self.len { return Err(OutOfRange); }
        Ok(self.ptr.cast())
    }

    pub fn get_ptr_offset<T>(&self, mem_offset: MemOffset) -> Result<NonNull<T>, MemErr> {
        self.get_ptr_byte_offset(match mem_offset {
            MemOffset::TypeOffset(offset) => offset * size_of::<T>(),
            MemOffset::ByteOffset(offset) => offset,
        })
    }

    pub fn get_ptr_type_offset<T>(&self, offset: usize) -> Result<NonNull<T>, MemErr> {
        self.get_ptr_byte_offset(offset * size_of::<T>())
    }

    pub fn get_ptr_byte_offset<T>(&self, byte_offset: usize) -> Result<NonNull<T>, MemErr> {
        let unit = size_of::<T>();
        let end = byte_offset + unit;
        if end > self.len { return Err(OutOfRange); }
        let i_byte_offset = isize::from_ne_bytes(byte_offset.to_ne_bytes());
        Ok(unsafe { self.ptr.byte_offset(i_byte_offset) }.cast())
    }
}

impl<T> MemView<T> {
    fn new(ptr: NonNull<T>) -> MemView<T> {
        Self { ptr }
    }

    fn read(&self) -> T {
        unsafe { self.ptr.read() }
    }

    fn write(&self, val: T) {
        unsafe { self.ptr.write(val) }
    }
}

impl_with_ptr!(write: |val: T| -> (), {
    unsafe { ptr.write(val) }
});
impl_with_ptr!(read: || -> T, {
    unsafe { ptr.read() }
});
impl_with_ptr!(get_view: || -> MemView<T>, {
    MemView { ptr }
});

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0001() {
        let bytes = &mut [0u8; 16];

        let buf = MemBuffer::from_bytes(bytes);

        let ptr_bytes = buf.get_ptr::<[u8; 16]>().unwrap();

        unsafe {
            println!("{:?}", ptr_bytes.read().map(|b| format!("0x{:02X}", b)));

            buf.write_type_offset::<u32>(0, 0x11223344).unwrap();
            buf.write_type_offset::<u32>(1, 0x55667788).unwrap();
            buf.write_type_offset::<u32>(2, 0x99AABBCC).unwrap();
            buf.write_type_offset::<u32>(3, 0xDDEEFF00).unwrap();

            println!("{:02} -> 0x{:08X}", 0, buf.read_byte_offset::<u32>(0).unwrap());
            println!("{:02} -> 0x{:08X}", 1, buf.read_byte_offset::<u32>(1).unwrap());
            println!("{:02} -> 0x{:08X}", 2, buf.read_byte_offset::<u32>(2).unwrap());
            println!("{:02} -> 0x{:08X}", 3, buf.read_byte_offset::<u32>(3).unwrap());

            println!("{:02} -> 0x{:08X}", 6, buf.read_byte_offset::<u32>(6).unwrap());
            println!("{:02} -> 0x{:08X}", 10, buf.read_byte_offset::<u32>(10).unwrap());
        }

        println!("{:?}", bytes.map(|b| format!("0x{:02X}", b)));
    }
}