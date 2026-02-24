use std::fs::File;

use crate::{
    process::{process::Process, process_stopper::ProcessStopper},
    read_maps,
    vm_maps::{
        proc_pid_maps::{mapping::Mapping, parser::parse_maps},
        proc_pid_pagemap::{page::Page, parser::parse_mapping},
    },
};

pub struct PageMap {
    pub map: Mapping,
    pub pages: Vec<Page>,
}

impl PageMap {
    pub fn new(map: Mapping, pages: Vec<Page>) -> Self {
        PageMap { map, pages }
    }
}

pub struct PageMapSnapshot {
    pub snapshot: Vec<PageMap>,
}

impl PageMapSnapshot {
    pub fn new<Ps: Process>(ps: &Ps) -> Self {
        let pid = ps.pid();
        let _pstopper = ProcessStopper::new(ps);
        let maps = read_maps(pid).unwrap();
        let (_, parsed_maps) = parse_maps(&maps).unwrap();

        let mut pagemap = File::open(format!("/proc/{}/pagemap", pid)).unwrap();

        let pm = parsed_maps
            .into_iter()
            .filter(|m| m.pathname.is_path())
            .map(|mapping| parse_mapping(&mut pagemap, mapping))
            .collect();
        PageMapSnapshot { snapshot: pm }
    }
}
