use crate::vm_maps::{proc_pid_maps::mapping::Mapping, proc_pid_pagemap::page::Page};

pub struct VMMap {
    maps: Mapping,
    pagemap: Vec<Page>,
}

impl VMMap {
    pub fn maps(&self) -> &Mapping {
        &self.maps
    }

    pub fn pagemap(&self) -> &Vec<Page> {
        &self.pagemap
    }

    pub fn new(maps: Mapping, pagemap: Vec<Page>) -> Self {
        VMMap { maps, pagemap }
    }
}

pub struct VMMaps {
    snapshot: Vec<VMMap>,
}

impl VMMaps {
    pub fn new(snapshot: Vec<VMMap>) -> Self {
        VMMaps { snapshot }
    }

    pub fn snapshot(&self) -> &Vec<VMMap> {
        &self.snapshot
    }
}
