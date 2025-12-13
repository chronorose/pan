use std::{fs::File, thread::sleep, time::Duration};

use crate::{
    PageMap,
    parse_maps::parse_maps,
    parse_pagemap::parse_mapping,
    process_manipulation::{Process, ProcessStopper},
    read_maps,
};

pub fn take_snapshot<Ps: Process>(ps: &Ps) -> Vec<PageMap> {
    let pid = ps.pid();
    let pstopper = ProcessStopper::new(ps);
    let maps = read_maps(pid).unwrap();
    let (_, parsed_maps) = parse_maps(&maps).unwrap();

    let mut pagemap = File::open(format!("/proc/{}/pagemap", pid)).unwrap();

    let pm = parsed_maps
        .into_iter()
        .filter(|m| m.pathname.is_path())
        .map(|mapping| parse_mapping(&mut pagemap, mapping))
        .collect();
    pm
}
