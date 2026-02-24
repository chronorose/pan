use std::fs::read_to_string;
use std::io;
use std::process::Command;
use std::time::Duration;
use std::{env::args, thread::sleep};

use crate::process::process::{ChildProcess, Process};

mod process;
mod stats;
mod symbols;
mod vm_maps;

fn read_maps(pid: u32) -> Result<String, io::Error> {
    read_to_string(format!("/proc/{}/maps", pid))
}

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        panic!("not enough args");
    }
    let mut cmd = Command::new(args[1].clone());
    cmd.args(&args[2..args.len()]);
    let ps = ChildProcess::new(cmd).unwrap();
    sleep(Duration::new(0, 5));
    let desc = stats::pagemap::PageMapStats::stats_description(&ps.snapshot());
    println!("{}", desc);
}
