use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use indicatif::ProgressBar;
use num_cpus;
use std::sync::mpsc::*;
use walkdir::WalkDir;

use crate::pipeline::{file_run, PreprocOpts};

lazy_static! {
    static ref NUM_CPUS: String = num_cpus::get().to_string();
}

#[derive(Parser, Debug, Clone)]
pub struct CommandLine {
    #[clap(short='t', default_value=NUM_CPUS.as_str(), parse(try_from_str = usize::from_str), help = "Number of threads to use")]
    threads: usize,
    #[clap(flatten)]
    preproc_opts: PreprocOpts,
    #[clap(parse(from_os_str))]
    input: PathBuf,
    #[clap(parse(from_os_str))]
    output: PathBuf,
}

pub fn run(opts: CommandLine) {
    if !opts.input.exists() {
        panic!("Given input file or directory does not exist.");
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build()
        .unwrap();
    let walker = WalkDir::new(opts.input).into_iter();
    let mut output = fs::File::create(opts.output).unwrap();
    // TODO: Use https://github.com/fosskers/linya instead, as indicatif concurrency is inneficient
    let progress = ProgressBar::new(0);

    pool.install(|| {
        let (sender, receiver): (Sender<String>, Receiver<String>) = channel();
        for entry in walker {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                progress.inc_length(1);
                let tx = sender.clone();
                let preproc_opts = opts.preproc_opts.clone();
                pool.spawn(move || {
                    tx.send(file_run(preproc_opts, entry)).unwrap();
                });
            }
        }
        drop(sender);

        while let Ok(text) = receiver.recv() {
            progress.inc(1);
            output.write_all(text.as_bytes()).unwrap();
        }
    });
}
