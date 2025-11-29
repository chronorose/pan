use std::{
    fs::File,
    io::{Read, Seek},
};

use crate::parse_maps::Mapping;

pub struct Page {
    pub present: bool,
}

impl Page {
    fn new(input: &[u8]) -> Self {
        assert!(input.len() == 8);
        let last_byte = input[7];
        let present_bit = (last_byte & 128) != 0;
        Page {
            present: present_bit,
        }
    }
}

const PAGE_SIZE: u64 = 4096; // TODO: unhardcode page_size

pub fn parse_mapping(pagemap: &mut File, m: Mapping) -> (Mapping, Vec<Page>) {
    let range = (m.address_range() / PAGE_SIZE) * 8; // (address / 4096) * 8
    let start = m.address_start();
    assert!(start % PAGE_SIZE == 0);
    let starting_page = (start / PAGE_SIZE) * 8;

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

    (m, pages)
}
