mod lib;

use std::mem;
use lib::*;

fn main() {
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