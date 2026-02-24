// use std::fs;
//
// use object::{File, Object, ObjectSection, ObjectSymbol, Section, Symbol};
//
// use crate::vm_maps::{
//     pages_snapshot::{PageMap, PageMapSnapshot},
//     proc_pid_maps::mapping::Mapping,
// };
//
// pub trait SectionMap {
//     fn map_sections(&self, mapping: Mapping) -> Option<Vec<Section<'_, '_>>>;
// }
//
// pub trait GetSymbols {
//     fn get_symbols(&self, pagemap: PageMap) -> Option<Vec<Symbol<'_, '_>>>;
// }
//
// fn page_size() -> u64 {
//     4096 // TODO: dynamically get page size
// }
//
// // TODO: maybe rename
// struct ObjInfo<'a> {
//     sections: Vec<Section<'a, 'a>>,
//     symbols: Vec<Symbol<'a, 'a>>,
// }
//
// impl<'a> ObjInfo<'a> {
//     fn new(sections: Vec<Section<'a, 'a>>, symbols: Vec<Symbol<'a, 'a>>) -> Self {
//         ObjInfo { sections, symbols }
//     }
// }

// TODO: unwraps
// fn pagemap_symbols<'a>(pm: PageMap) -> ObjInfo<'a> {
//     let file = fs::read(pm.map.pathname.path()).unwrap();
//     let file = object::File::parse(&*file).unwrap();
//     let sections = file.map_sections(pm.map.clone()).unwrap();
//     let symbols = file.get_symbols(pm).unwrap();
//     ObjInfo::new(sections, symbols)
// }

// fn snapshot_symbols(snap: PageMapSnapshot) {}
//
// impl<'a> GetSymbols for File<'_> {
//     fn get_symbols(&self, pagemap: PageMap) -> Option<Vec<Symbol<'_, '_>>> {
//         if !pagemap.map.pathname.is_path() {
//             return None;
//         }
//         let pages = pagemap.pages;
//         let offset = pagemap.map.offset();
//         let symbols: Vec<_> = self
//             .symbols()
//             .filter(|symbol| {
//                 let symbol_addr = symbol.address();
//                 if pagemap.map.in_offset(symbol_addr) {
//                     let page = (symbol_addr - offset) / page_size();
//                     pages[page as usize].present
//                 } else {
//                     false
//                 }
//             })
//             .collect();
//         Some(symbols)
//     }
// }
//
// impl<'a> SectionMap for File<'a> {
//     fn map_sections(&self, mapping: Mapping) -> Option<Vec<Section<'_, '_>>> {
//         if !mapping.pathname.is_path() {
//             None
//         } else {
//             Some(
//                 self.sections()
//                     .filter(|section| mapping.in_offset(section.address()))
//                     .collect(),
//             )
//         }
//     }
// }
