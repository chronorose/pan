use std::fmt::Display;

use nom::Parser;
use nom::branch::alt;
use nom::bytes::take_until;
use nom::character::{char, complete::*};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::{IResult, character::complete::hex_digit1};

#[derive(Debug, Clone)]
pub struct Perms {
    read: bool,
    write: bool,
    exec: bool,
    shared: bool,
    private: bool,
}

impl Perms {
    fn new(read: bool, write: bool, exec: bool, shared: bool, private: bool) -> Self {
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
    pub pathname: Pathname,
}

impl Mapping {
    pub fn address_range(&self) -> u64 {
        self.address_end - self.address_start
    }
    pub fn address_start(&self) -> u64 {
        self.address_start
    }
    pub fn address_end(&self) -> u64 {
        self.address_end
    }
    fn new(addresses: (u64, u64), perms: Perms, offset: u64, pathname: Pathname) -> Self {
        Mapping {
            address_start: addresses.0,
            address_end: addresses.1,
            perms,
            offset,
            pathname,
        }
    }
}

pub fn parse_maps(str: &str) -> IResult<&str, Vec<Mapping>> {
    many0(pattern).parse_complete(str)
}

fn pathname(str: &str) -> IResult<&str, Pathname> {
    map(take_until("\n"), |str: &str| {
        if str.is_empty() {
            Pathname::Mapping
        } else if str.starts_with("[") {
            Pathname::PseudoPath(str.to_string())
        } else {
            Pathname::Path(str.to_string())
        }
    })
    .parse(str)
}

fn pattern(str: &str) -> IResult<&str, Mapping> {
    map(
        (
            parse_addresses,
            space1,
            perms,
            space1,
            hex_digit1,
            (
                space1,
                hex_digit1,
                char(':'),
                hex_digit1,
                space1,
                hex_digit1,
                space1,
            ),
            pathname,
            line_ending,
        ),
        |(ad, _, perms, _, offset, _, pathname, _)| {
            Mapping::new(
                ad,
                perms,
                u64::from_str_radix(offset, 16).unwrap(),
                pathname,
            )
        },
    )
    .parse(str)
}

fn perms(str: &str) -> IResult<&str, Perms> {
    map(
        (
            either_letter_dash('r'),
            either_letter_dash('w'),
            either_letter_dash('x'),
            alt((char('p'), char('s'))),
        ),
        |(r, w, x, sp)| Perms::new(r, w, x, sp == 's', sp == 'p'),
    )
    .parse(str)
}

fn either_letter_dash(ch: char) -> impl Fn(&str) -> IResult<&str, bool> {
    move |str: &str| map(alt((char(ch), char('-'))), |g: char| g == ch).parse(str)
}

fn parse_address(str: &str) -> IResult<&str, u64> {
    map(hex_digit1, |num: &str| {
        u64::from_str_radix(num, 16).unwrap()
    })
    .parse(str)
}

fn parse_addresses(str: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(parse_address, char('-'), parse_address).parse(str)
}
