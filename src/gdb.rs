pub mod gdb{
    use std::process::{Command, Stdio};
    use std::io::prelude::*;
    use std::ops::Add;

struct GdbProcess{
    stdin:std::process::ChildStdin,
    process:std::process::Child
}

pub enum GdbCommand{
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

pub struct RemoteGdbConnection{
    gdb:GdbProcess,
}

impl RemoteGdbConnection{
    pub fn connect() -> RemoteGdbConnection{
        let mut gdb = GdbProcess::execute("gdb-multiarch");
        gdb.send_command(GdbCommand::ConnectRemote("127.0.0.1".to_string(), 3333));
        RemoteGdbConnection{gdb}
    }
    
    pub fn send_command(&mut self, command:GdbCommand){
        let command = self.gdb.serialize_command(command);
        self.gdb.send_raw_command(&command);
    }
}

impl Drop for RemoteGdbConnection{
    fn drop(&mut self){
        self.send_command(GdbCommand::Quit());
    }
}
}