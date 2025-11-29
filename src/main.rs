#[allow(warnings)]
use libc;
use std::env::args;
use std::fs::read_to_string;
use std::{io, process};

use crate::parse_maps::parse_maps;

mod parse_maps;

fn get_proc_from_cli() -> Option<String> {
    let args: Vec<_> = args().collect();
    if args.len() != 2 {
        None
    } else {
        Some(args[1].clone())
    }
}

fn read_maps(pid: u32) -> Result<String, io::Error> {
    read_to_string(format!("/proc/{}/maps", pid))
}

fn main() {
    let proc_name = get_proc_from_cli();
    println!("{}", process::id());
    let maps = read_maps(process::id());
    let parsed_maps = parse_maps(&maps.unwrap()).unwrap().1;
}
