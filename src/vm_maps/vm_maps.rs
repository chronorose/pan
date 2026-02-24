use crate::vm_maps::{proc_pid_maps::mapping::Mapping, proc_pid_pagemap::page::Page};

pub struct VMMap {
    pub maps: Mapping,
    pub pagemap: Vec<Page>,
}

impl VMMap {
    pub fn new(maps: Mapping, pagemap: Vec<Page>) -> Self {
        VMMap { maps, pagemap }
    }
}

pub struct VMMaps {
    pub snapshot: Vec<VMMap>,
}
