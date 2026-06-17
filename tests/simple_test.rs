use std::{env, ffi::CString, path::Path, process::Command};

use pan::{process::tracer::ProcessTracer, snapshot::{Snapshotter, VMMapSnapshotter}, stats::pagemap_stats::PageMapStats, symbols::elf::Elf, vm_maps::vm_maps::VMMaps};

fn cmd(c: &str) -> Command {
    Command::new(c)
}

fn ccompile_bin(inp: &[&str], out: &str) {
    let args = &[inp, &["-o", out]].concat();
    cmd("gcc")
        .args(args)
        .status()
        .expect("compilation was unsuccesful");
}

fn ccompile_bin_with_so(inp: &[&str], so: &str, out: &str) {
    cmd("gcc")
        .args(&[inp, &["-L.", &("-l".to_string() + so), "-o", out]].concat())
        .status()
        .expect("oopsie-daisy");
}

fn ccompile_so(inp: &[&str], out: &str) {
    let compile_args = &[inp, &["-c", "-fPIC", "-o", "obj.o"]].concat();
    let link_args    = &["obj.o","-shared", "-o", &("lib".to_string() + out)];

    cmd("gcc")
        .args(compile_args)
        .status()
        .expect("shared lib compilation was unsuccesful");

    cmd("gcc")
        .args(link_args)
        .status()
        .expect("shared lib linking was unsuccesful");
}

#[test]
fn simple_test() {
    env::set_current_dir("./tests/simple_test").unwrap();
    ccompile_so(&["so.c"], "so.so");
    ccompile_bin_with_so(&["main.c"], "so", "test-bin");

    cmd("patchelf")
        .args(&["--set-rpath", ".", "test-bin"])
        .status()
        .expect("the unexpected");

    let bin = String::from("./test-bin");

    let bin_path = std::fs::canonicalize(Path::new(&bin)).expect("oopsie");

    let proc = ProcessTracer::new(&CString::from(c"./test-bin"), &[]);


    VMMapSnapshotter::snapshot_stopped_with(&proc, |snapshot| {
        let ss = snapshot.snapshot_of(&bin_path);
        assert!(!ss.is_empty());
        let first = ss[0].clone();
        let addr_start = first.maps().address_start();
        
        let elf = Elf::create(first);
        let k = elf.unwrap().find_symbol("main");
        let desc = PageMapStats::stats_description(&VMMaps::new(ss));
        println!("{}", desc);
    });
}
