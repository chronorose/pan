use std::{
    fs::{File, read_to_string},
    io,
};

use crate::{
    process::{
        process::{PID, Process},
        process_stopper::ProcessStopper,
    },
    vm_maps::{
        proc_pid_maps::{mapping::Mapping, parser::parse_maps},
        proc_pid_pagemap::parser::parse_mapping,
        vm_maps::{VMMap, VMMaps},
    },
};

pub trait Snapshotter {
    type Context: ?Sized;
    type Output;

    fn snapshot(ctx: &Self::Context) -> Self::Output;
    fn snapshot_with<R>(ctx: &Self::Context, f: impl FnOnce(Self::Output) -> R) -> R;
    fn snapshot_stopped_with<R>(ctx: &Self::Context, f: impl FnOnce(Self::Output) -> R) -> R;
}

pub struct VMMapSnapshotter;

impl VMMapSnapshotter {
    fn maps_path(pid: PID) -> String {
        format!("/proc/{}/maps", pid)
    }

    fn pagemap_path(pid: PID) -> String {
        format!("/proc/{}/pagemap", pid)
    }

    fn maps_contents(pid: PID) -> io::Result<String> {
        read_to_string(Self::maps_path(pid))
    }

    fn pagemap_descriptor(pid: PID) -> io::Result<File> {
        File::open(Self::pagemap_path(pid))
    }

    fn validate_mappings(mut pagemap: &mut File, mappings: Vec<Mapping>) -> Vec<VMMap> {
        mappings
            .into_iter()
            .filter(|m| m.pathname().is_path())
            .map(|m| parse_mapping(&mut pagemap, m))
            .collect()
    }
}

impl Snapshotter for VMMapSnapshotter {
    // Can't do impl Process as it is in unstable features for some reason.
    // TODO: fix this once it is out of unstable.
    type Context = dyn Process;
    type Output = VMMaps;

    fn snapshot(ctx: &Self::Context) -> Self::Output {
        let pid = ctx.pid();
        let _pstopper = ProcessStopper::new(ctx);
        let maps = Self::maps_contents(pid).unwrap();
        let (_, parsed_maps) = parse_maps(&maps).unwrap();

        let mut pm_desc = Self::pagemap_descriptor(pid).unwrap();

        let mappings = Self::validate_mappings(&mut pm_desc, parsed_maps);

        VMMaps::new(mappings)
    }

    fn snapshot_with<R>(ctx: &Self::Context, f: impl FnOnce(Self::Output) -> R) -> R {
        let pid = ctx.pid();
        let _pstopper = ProcessStopper::new(ctx);
        let maps = Self::maps_contents(pid).unwrap();
        let (_, parsed_maps) = parse_maps(&maps).unwrap();

        let mut pm_desc = Self::pagemap_descriptor(pid).unwrap();

        let mappings = Self::validate_mappings(&mut pm_desc, parsed_maps);

        f(VMMaps::new(mappings))
    }

    fn snapshot_stopped_with<R>(ctx: &Self::Context, f: impl FnOnce(Self::Output) -> R) -> R {
        let pid = ctx.pid();
        let maps = Self::maps_contents(pid).unwrap();
        let (_, parsed_maps) = parse_maps(&maps).unwrap();

        let mut pm_desc = Self::pagemap_descriptor(pid).unwrap();

        let mappings = Self::validate_mappings(&mut pm_desc, parsed_maps);

        f(VMMaps::new(mappings))
    }
}

