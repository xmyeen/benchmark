extern crate fs2;

use fs2::FileExt;
use std::io::Result;
use std::env::args;
use std::fs::File;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    run().unwrap();
}

fn run() -> Result<()> {
    let sleep_seconds = args().nth(1).and_then(|arg| arg.parse().ok()).unwrap_or(0);
    let sleep_duration = Duration::from_secs(sleep_seconds);

    let file = File::open("file.lock")?;

    println!("{}: Preparing to lock file.", sleep_seconds);
    file.lock_exclusive()?; // block until this process can lock the file
    println!("{}: Obtained lock.", sleep_seconds);

    sleep(sleep_duration);

    println!("{}: Sleep completed", sleep_seconds);
    file.unlock()?;
    println!("{}: Released lock, returning", sleep_seconds);

    Ok(())
}


