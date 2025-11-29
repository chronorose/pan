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

fn read_pagemap(pid: u32) -> Result<String, io::Error> {
    read_to_string(format!("/proc/{}/pagemap", pid))
}

fn take_snapshot(pid: u32) {
    unsafe {
        libc::kill(pid as i32, libc::SIGSTOP);
    }
    let parsed_maps = parse_maps(&read_maps(pid).unwrap()).unwrap(); // FIXME: cringe unwraps
    // parse pagemap
}

fn main() {
    let proc_name = get_proc_from_cli();
    take_snapshot(process::id());
}
