use std::process::Command;
use std::time::Duration;
use std::{env::args, thread::sleep};

use pan::process::process::ChildProcess;
use pan::snapshot::{Snapshotter, VMMapSnapshotter};
use pan::stats::pagemap_stats::PageMapStats;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() < 2 {
        panic!("not enough args");
    }
    let mut cmd = Command::new(args[1].clone());
    cmd.args(&args[2..args.len()]);
    let ps = ChildProcess::new(cmd).unwrap();
    sleep(Duration::new(0, 5));
    VMMapSnapshotter::snapshot_with(&ps, |snapshot| {
        let desc = PageMapStats::stats_description(&snapshot);
        println!("{}", desc);
    });
}
