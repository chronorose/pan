use std::fs;

use object::{File, Object, ObjectSection, ObjectSymbol, Section, Symbol};
use ouroboros::self_referencing;

use crate::vm_maps::{
    proc_pid_maps::mapping::Mapping, proc_pid_pagemap::page::Page, vm_maps::VMMap,
};

// Cursed module because of how rust works with self-referencing structs.
//
// Will be better for your mental health to just ignore it.

#[self_referencing]
pub struct Elf {
    data: Vec<u8>,
    #[borrows(data)]
    #[not_covariant]
    file: File<'this>,
}

impl Elf {
    fn create(vmm: &VMMap) -> Option<Self> {
        if !vmm.maps().pathname().is_path() {
            return None;
        }

        let file = fs::read(vmm.maps().pathname().path()).ok()?;
        // let obj_file = object::File::parse(bytes).unwrap();

        ElfTryBuilder {
            data: file,
            file_builder: |bytes| File::parse(&bytes[..]).map_err(|_| "Object file parsing failed"),
        }
        .try_build()
        .ok()
    }

    fn get_sections<'a>(&'a self, mapping: &Mapping) -> Option<Vec<Section<'a, 'a>>> {
        if !mapping.pathname().is_path() {
            return None;
        }

        Some(self.with_file(|file| {
            file.sections()
                .filter(|s| mapping.in_offset(s.address()))
                .collect()
        }))
    }

    fn get_loaded_symbols<'a>(&'a self, pm: &VMMap) -> Option<Vec<Symbol<'a, 'a>>> {
        if !pm.maps().pathname().is_path() {
            return None;
        }

        let pages = pm.pagemap();
        let offset = pm.maps().offset();
        let symbols: Vec<Symbol> = self.with_file(|file| {
            file.symbols()
                .filter(|symbol| {
                    let symbol_addr = symbol.address();
                    let page = (symbol_addr - offset) / Page::page_size();
                    pm.maps().in_offset(symbol_addr) && pages[page as usize].present
                })
                .collect()
        });

        Some(symbols)
    }
}
