pub mod gdb;
//I have no Idea why there are 2 gdbs here
use gdb::gdb::{GdbCommand, RemoteGdbConnection};
const HOOKS_FILE: &'static str = "/home/duzicman/projects/rust/qemu_patcher/hooks/target/thumbv7m-none-eabi/release/hooks";
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};


//TODO: thumb arm 7 - https://web.eecs.umich.edu/~prabal/teaching/eecs373-f10/readings/ARMv7-M_ARM.pdf
//TODO: A6-49 
fn patch(target:i32, source:i32) -> Vec<u8> {
    let offset = target - source;

    let mut offset_1_to_11_bytes:u16 = ((offset >> 1) & ((1 << 11) - 1)) as u16;
    
    //TODO: I have no idea why the minus 2 - but hey it works
    offset_1_to_11_bytes -= 2;

    let offset_12_to_21_bytes:u16 = ((offset >> 12) & ((1 << 10) - 1)) as u16;

    let sign:u16 = u16::from(offset < 0);

    let j2 = ((!(offset >> 22 ) & 1) as u16) ^ sign;
    let j1 = ((!(offset >> 23 ) & 1) as u16) ^ sign;
    
    let high:u16 = (0b11110 << 11) + (sign << 10) + offset_12_to_21_bytes;
    let low:u16 = (0b11 << 14) + (j1 << 13) + (1 << 12) + (j2 << 11) + offset_1_to_11_bytes;

    let mut out_vec = vec![];

    out_vec.write_u16::<LittleEndian>(high).unwrap();
    out_vec.write_u16::<LittleEndian>(low).unwrap();
    out_vec
}

fn main() {
    let patch_content = patch(0x7000, 0x49c);
    let mut rdr = Cursor::new(patch_content);

    let mut connection = RemoteGdbConnection::connect();
    connection.send_command(GdbCommand::LoadFile(HOOKS_FILE.to_string()));
    connection.send_command(GdbCommand::WriteU32(0x49c, rdr.read_u32::<LittleEndian>().unwrap()));
}
