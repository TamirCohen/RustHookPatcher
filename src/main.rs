use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::ops::Add;

const HOOKS_FILE: &'static str = "/home/duzicman/projects/rust/qemu_patcher/hooks/target/thumbv7m-none-eabi/release/hooks";

//BL address: 0x000004A8
//symbol address 0x00007000

fn main() {
    let mut process = Command::new("gdb-multiarch");

    process.stdin(Stdio::piped());

    let mut child = process.spawn().unwrap();


    let mut stdin = child.stdin.take().unwrap(); 

    stdin.write_all(b"target remote :3333\n").unwrap();

    let load_command:String = "restore ".to_string();
    let load_command = load_command.add(HOOKS_FILE);
    let load_command = load_command.add("\n");

    stdin.write_all(load_command.as_bytes()).unwrap();

    //jump to 0x7000, currently using the website https://armconverter.com/?code=BL%20%200x00007000&offset=4a8
    // 06F0AAFD

    stdin.write_all(b"set {int}0x4a8 = 4255838214\n").unwrap();
    stdin.write_all(b"detach\nq\n").unwrap();
    child.wait().unwrap();
}
