use std::fs;

use object::{File, Object, ObjectSection, ObjectSymbol, Section, Symbol};
use ouroboros::self_referencing;

use crate::vm_maps::{proc_pid_pagemap::page::Page, vm_maps::VMMap};

// Cursed module because of how rust works with self-referencing structs.
//
// Will be better for your mental health to just ignore it.

#[self_referencing]
pub struct Elf {
    map: VMMap,
    data: Vec<u8>,
    #[borrows(data)]
    #[not_covariant]
    file: File<'this>,
}

impl Elf {
    pub fn create(vmm: VMMap) -> Option<Self> {
        if !vmm.maps().pathname().is_path() {
            return None;
        }

        let file = fs::read(vmm.maps().pathname().path()).ok()?;

        ElfTryBuilder {
            map: vmm,
            data: file,
            file_builder: |bytes| File::parse(&bytes[..]).map_err(|_| "Object file parsing failed"),
        }
        .try_build()
        .ok()
    }

    pub fn get_sections<'a>(&'a self) -> Vec<Section<'a, 'a>> {
        let mapping = self.borrow_map().maps();

        self.with_file(|file| {
            file.sections()
                .filter(|s| mapping.in_offset(s.address()))
                .collect()
        })
    }

    pub fn get_symbols<'a>(&'a self) -> Vec<Symbol<'a, 'a>> {
        self.with_file(|file| file.symbols().collect())
    }

    pub fn get_loaded_symbols<'a>(&'a self) -> Vec<Symbol<'a, 'a>> {
        let pm = self.borrow_map();
        let pages = pm.pagemap();
        let offset = pm.maps().offset();

        self.get_symbols()
            .into_iter()
            .filter(|symbol| {
                let symbol_addr = symbol.address();
                let page = (symbol_addr - offset) / Page::page_size();
                pm.maps().in_offset(symbol_addr) && pages[page as usize].present
            })
            .collect()
    }
}
