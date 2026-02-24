use std::{
    fs::File,
    io::{Read, Seek},
};

use crate::vm_maps::{
    pages_snapshot::PageMap, proc_pid_maps::mapping::Mapping, proc_pid_pagemap::page::Page,
};

pub fn parse_mapping(pagemap: &mut File, m: Mapping) -> PageMap {
    let range = ((m.address_range() / Page::page_size()) + 1) * 8; // (address / 4096) * 8
    let start = m.address_start();
    assert!(start % Page::page_size() == 0);
    let starting_page = (start / Page::page_size()) * 8;

    // TODO: fixed buffer without reallocations for each mapping.
    let mut buffer: Vec<u8> = vec![0; range as usize];

    pagemap
        .seek(std::io::SeekFrom::Start(starting_page))
        .unwrap();

    pagemap.read_exact(&mut buffer).unwrap();

    let pages: Vec<Page> = buffer
        .chunks_exact(8)
        .map(|chunk| Page::new(chunk))
        .collect();

    PageMap::new(m, pages)
}
