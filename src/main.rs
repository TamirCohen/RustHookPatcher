pub mod gdb;
use std::error::Error;
//I have no Idea why there are 2 gdbs here
use gdb::gdb::{GdbCommand, RemoteGdbConnection};
const HOOKS_FILE: &'static str = "/home/duzicman/projects/rust/qemu_patcher/hooks/target/thumbv7m-none-eabi/release/hooks";

//BL address: 0x000004A8
//symbol address 0x00007000

trait MemoryAccess {
    fn read_memory(&self, length:u32, address:u64) -> Result<Vec<u8>, Box<dyn Error>>;
    fn write_memory(&self, content: &Vec<u8>, address:u64) -> Box<dyn Error>;
}

trait Patcher{
    fn patch(&self, length:u32) -> Result<Vec<u8>, Box<dyn Error>>;
    fn unpatch(&self, content: &Vec<u8>) -> Box<dyn Error>;
}

fn main() {
    let mut connection = RemoteGdbConnection::connect();
    connection.send_command(GdbCommand::LoadFile(HOOKS_FILE.to_string()));
    connection.send_command(GdbCommand::WriteU32(0x4a8, 0xfdaaf006));

    //jump to 0x7000, currently using the website https://armconverter.com/?code=BL%20%200x00007000&offset=4a8
    // 06F0AAFD
}
