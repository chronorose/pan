#[derive(Clone)]
pub struct Page {
    pub present: bool,
}

impl Page {
    pub fn new(input: &[u8]) -> Self {
        assert!(input.len() == 8);
        let last_byte = input[7];
        let present_bit = (last_byte & 128) != 0;
        Page {
            present: present_bit,
        }
    }
    pub fn page_size() -> u64 {
        PAGE_SIZE // TODO: unhardcode page_size
    }
}

const PAGE_SIZE: u64 = 4096;
