use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Perms {
    read: bool,
    write: bool,
    exec: bool,
    shared: bool,
    private: bool,
}

impl Perms {
    pub fn new(read: bool, write: bool, exec: bool, shared: bool, private: bool) -> Self {
        Perms {
            read,
            write,
            exec,
            shared,
            private,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Pathname {
    Path(String),
    PseudoPath(String),
    Mapping,
}

impl Pathname {
    pub fn path(&self) -> &str {
        match &self {
            Self::Path(s) => s,
            Self::PseudoPath(s) => s,
            Self::Mapping => "anon_mapping",
        }
    }
    pub fn is_path(&self) -> bool {
        match &self {
            Pathname::Path(_) => true,
            _ => false,
        }
    }
}

impl Display for Pathname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Pathname::Path(name) => write!(f, "{}", name),
            Pathname::PseudoPath(name) => write!(f, "{}", name),
            Pathname::Mapping => write!(f, "anon mapping"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mapping {
    address_start: u64,
    address_end: u64,
    perms: Perms,
    offset: u64,
    pathname: Pathname,
}

impl Mapping {
    pub fn pathname(&self) -> &Pathname {
        &self.pathname
    }

    pub fn address_range(&self) -> u64 {
        self.address_end - self.address_start
    }
    pub fn address_start(&self) -> u64 {
        self.address_start
    }
    pub fn address_end(&self) -> u64 {
        self.address_end
    }
    pub fn offset(&self) -> u64 {
        self.offset
    }
    pub fn in_offset(&self, address: u64) -> bool {
        address >= self.offset() && address <= self.offset() + self.address_range()
    }
    pub fn new(addresses: (u64, u64), perms: Perms, offset: u64, pathname: Pathname) -> Self {
        Mapping {
            address_start: addresses.0,
            address_end: addresses.1,
            perms,
            offset,
            pathname,
        }
    }
}
