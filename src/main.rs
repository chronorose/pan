use std::env::args;
use std::fs::read_to_string;
use std::io;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use crate::pages_snapshot::take_snapshot;
use crate::parse_maps::Mapping;
use crate::parse_pagemap::Page;
use crate::process_manipulation::ChildProcess;

mod pages_snapshot;
mod parse_maps;
mod parse_pagemap;
mod process_manipulation;

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

type PageMap = (Mapping, Vec<Page>);

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
    let mut cmd = Command::new(proc[0]);
    cmd.args(&proc[1..proc.len()]);
    let ps = ChildProcess::new(cmd).unwrap();
    let snapshot = take_snapshot(&ps);
    for pm in snapshot {
        print_stats(pm);
        println!();
    }
}
