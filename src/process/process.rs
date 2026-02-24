use std::{
    fs::File,
    io,
    process::{Child, Command},
};

use crate::{
    process::process_stopper::ProcessStopper,
    read_maps,
    vm_maps::{
        pages_snapshot::PageMapSnapshot, proc_pid_maps::parser::parse_maps,
        proc_pid_pagemap::parser::parse_mapping,
    },
};

pub trait Process {
    fn pid(&self) -> u32;
    fn snapshot(&self) -> PageMapSnapshot {
        let pid = self.pid();
        let _pstopper = ProcessStopper::new(self);
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

pub struct ChildProcess {
    ps: Child,
}

impl ChildProcess {
    pub fn new(mut cmd: Command) -> io::Result<Self> {
        cmd.spawn().map(|ps| ChildProcess { ps })
    }
}

impl Drop for ChildProcess {
    fn drop(&mut self) {
        let w = self.ps.try_wait();
        match w {
            Ok(_) => (),
            Err(err) if err.raw_os_error().unwrap() == 10 => (), // this is the case where all
            // child processes exited and now
            // it throws error for some
            // reason?
            Err(_) => self.ps.kill().unwrap(),
        }
    }
}

impl Process for ChildProcess {
    fn pid(&self) -> u32 {
        self.ps.id()
    }
}
