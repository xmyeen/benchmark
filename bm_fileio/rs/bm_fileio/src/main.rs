// use std::env::temp_dir;
use std::{convert::{AsRef, TryInto}, fs::File, io::Write, path::{Path, PathBuf}};
use clap::Parser;
use tokio::{fs::{File as TokioFile}, io::{AsyncWriteExt}};

#[derive(Parser, Debug)]
#[clap(author = "xmyeen<xmyeen@126.com>", version = "1.0", about, long_about = None)]
struct Opts {
    #[clap(short, long, default_value="3")]
    times: u32,

    #[clap(long, default_value="1024")]
    bb: u32,

    #[clap(long, default_value="104857600")]
    tb: u32,

    #[clap(short, long, default_value=".")]
    dir: PathBuf,
}

fn write_sync(file_path:&Path, times:u32, step:u32, total:u32) {
    let now = std::time::Instant::now();
    let vec = vec![0u8; step.try_into().unwrap()];

    for _ in 0..times {
        let mut file = File::create(file_path).unwrap();
        for _ in (0..total).step_by(step.try_into().unwrap()) {
            file.write(&vec);
        };
    }
    let elapsed = now.elapsed();
    println!("Syn-Elapsed : {:.2?}", elapsed / times);

    if file_path.exists() {
        std::fs::remove_file(file_path).unwrap();
    }
}

async fn write_async(file_path:&Path, times:u32, step:u32, total:u32) {
    let now = std::time::Instant::now();
    let vec = vec![0u8; step.try_into().unwrap()];

    for _ in 0..times {
        let mut file = TokioFile::create(file_path).await.unwrap();
        for _ in (0..total).step_by(step.try_into().unwrap()) {
            file.write(&vec).await.unwrap();
        };
    }
    let elapsed = now.elapsed();
    println!("Asyn-Elapsed: {:.2?}", elapsed / times);

    if file_path.exists() {
        tokio::fs::remove_file(file_path).await.unwrap();
    }
}


fn main() {
    let opts: Opts = Opts::parse();
    println!("Batch-Bytes : {}", opts.bb);
    println!("Total-Bytes : {}", opts.tb);
    println!("Times : {}", opts.times);
    println!("Output-Directory : {}", &opts.dir.display());
    // u32::pow(2, 20)

    let dir_path = PathBuf::from(&opts.dir);
    if !dir_path.exists() {
        std::fs::create_dir_all(&dir_path).expect(&format!("Create directory failed: {}", &opts.dir.display()));
    }

    let mut file_path = opts.dir.clone();
    file_path.push("file_1");

    write_sync(file_path.as_path(), opts.times, opts.bb, opts.tb);

    let tr = tokio::runtime::Runtime::new().unwrap();
    tr.block_on(write_async(file_path.as_path(), opts.times, opts.bb, opts.tb));
}
