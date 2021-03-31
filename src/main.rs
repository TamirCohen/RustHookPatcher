extern crate log;
use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::ops::Add;
use std::error::Error;
use log::{info};

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

//checkout how ip/port is represented nicely
struct GdbProcess{
    stdin:std::process::ChildStdin,
    process:std::process::Child
}

enum GdbCommand{
    ConnectRemote(String, u16),
    DisconnectRemote(),
    LoadFile(String),
    Quit(),
    WriteU32(u64, u32)
}

impl GdbProcess{
    fn execute(gdb_executable:&str) -> GdbProcess{
        let mut command = Command::new(gdb_executable);
        command.stdin(Stdio::piped());
        command.stdout(Stdio::piped());

        let mut process = command.spawn().unwrap();
        let stdin = process.stdin.take().unwrap();
        
        GdbProcess{stdin, process}
    }

    fn serialize_command(&mut self, command:GdbCommand) -> String{
        match command{
            GdbCommand::ConnectRemote(ip, port) =>  format!("target remote {}:{}", ip, port),
            GdbCommand::LoadFile(file_name) => format!("restore {}", file_name),
            GdbCommand::DisconnectRemote() => "detach".to_string(),
            GdbCommand::Quit() => "q".to_string(),
            GdbCommand::WriteU32(address, value) => format!("set {{int}}{:#X} = {}", address, value)
        }.add("\n")
    }
    fn send_command(&mut self, command:GdbCommand){
        let command = self.serialize_command(command);
        self.send_raw_command(&command);
    }
    fn send_raw_command(&mut self, command:&str){
        println!("SENT: {}", command);
        self.stdin.write_all(command.as_bytes()).unwrap();
    }
}

impl Drop for GdbProcess{
    fn drop(&mut self){
        self.process.wait().unwrap();
    }
}

struct RemoteGdbConnection{
    gdb:GdbProcess,
}

impl RemoteGdbConnection{
    fn connect() -> RemoteGdbConnection{
        let mut gdb = GdbProcess::execute("gdb-multiarch");
        gdb.send_command(GdbCommand::ConnectRemote("127.0.0.1".to_string(), 3333));
        RemoteGdbConnection{gdb}
    }
    
    fn send_command(&mut self, command:GdbCommand){
        let command = self.gdb.serialize_command(command);
        self.gdb.send_raw_command(&command);
    }
}

impl Drop for RemoteGdbConnection{
    fn drop(&mut self){
        self.send_command(GdbCommand::Quit());
    }
}


fn main() {
    let mut connection = RemoteGdbConnection::connect();
    connection.send_command(GdbCommand::LoadFile(HOOKS_FILE.to_string()));
    connection.send_command(GdbCommand::WriteU32(0x4a8, 0xfdaaf006));

    //jump to 0x7000, currently using the website https://armconverter.com/?code=BL%20%200x00007000&offset=4a8
    // 06F0AAFD
}
