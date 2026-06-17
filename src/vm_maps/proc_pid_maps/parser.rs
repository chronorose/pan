use nom::Parser;
use nom::branch::alt;
use nom::bytes::take_until;
use nom::character::{char, complete::*};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::separated_pair;
use nom::{IResult, character::complete::hex_digit1};

use crate::vm_maps::proc_pid_maps::mapping::{Mapping, Pathname, Perms};

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
