use libc;
use std::env::args;
use std::fs::{File, read_to_string};
use std::io;
use std::process::Command;

use crate::parse_maps::{Mapping, parse_maps};
use crate::parse_pagemap::{Page, parse_mapping};

mod parse_maps;
mod parse_pagemap;

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

struct ProcessStopper {
    pid: u32,
}

impl ProcessStopper {
    fn new(pid: u32) -> ProcessStopper {
        unsafe {
            libc::kill(pid as i32, libc::SIGSTOP);
        }
        ProcessStopper { pid }
    }
}

impl Drop for ProcessStopper {
    fn drop(&mut self) {
        unsafe {
            libc::kill(self.pid as i32, libc::SIGCONT);
        }
    }
}

type PageMap = (Mapping, Vec<Page>);

fn take_snapshot(pid: u32) -> Vec<PageMap> {
    ProcessStopper::new(pid);
    let (_, parsed_maps) = parse_maps(&read_maps(pid).unwrap()).unwrap(); // FIXME: cringe unwraps

    let mut pagemap = File::open(format!("/proc/{}/pagemap", pid)).unwrap();

    let pm = parsed_maps
        .into_iter()
        .filter(|m| !(m.pathname.starts_with("[") || m.pathname.is_empty()))
        .map(|mapping| parse_mapping(&mut pagemap, mapping))
        .collect();
    pm
}

fn print_stats(pm: PageMap) {
    let total = pm.1.len();
    let present_pages = pm.1.iter().filter(|page| page.present).count();
    let not_present_pages = total - present_pages;

    println!(
        "Pathname {} has mapped {} page(s) in total.",
        pm.0.pathname, total,
    );

    println!(
        "Out of them, present in RAM currently are {}, not present in RAM are {}",
        present_pages, not_present_pages
    );

    println!(
        "Percentage of present in RAM pages: {}%",
        if present_pages > 0 {
            (present_pages as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    );
}

fn main() {
    let proc_name = get_proc_from_cli().unwrap();
    let proc: Vec<&str> = proc_name.split(" ").collect();
    let mut child = Command::new(proc[0])
        .args(&proc[1..proc.len()])
        .spawn()
        .unwrap();

    let snapshot = take_snapshot(child.id());
    for pm in snapshot {
        print_stats(pm);
        println!();
    }
    child.kill().unwrap();
}
