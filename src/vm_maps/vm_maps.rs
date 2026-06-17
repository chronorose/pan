use std::path::Path;

use crate::vm_maps::{proc_pid_maps::mapping::Mapping, proc_pid_pagemap::page::Page, proc_pid_maps::mapping::Pathname};

#[derive(Clone)]
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

    pub fn snapshot_of(&self, path: &Path) -> Vec<VMMap> {
        self.snapshot.clone().into_iter().filter(|vm| {
            match vm.maps.pathname() {
                Pathname::Path(snap_path) => snap_path == path.to_str().expect("Path was not utf8"),
                _ => false
            }
        }).collect()
    }
}
