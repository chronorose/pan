use crate::vm_maps::{proc_pid_maps::mapping::Mapping, proc_pid_pagemap::page::Page};

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
