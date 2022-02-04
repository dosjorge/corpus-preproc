#[macro_use]
extern crate lazy_static;
extern crate alloc;

use clap::Parser;
use tracing_subscriber;

use crate::cli::CommandLine;
use crate::server::Serve;

mod cleaners;
mod cli;
mod normalizers;
mod pipeline;
mod server;
mod ucd;

#[derive(Parser, Debug, Clone)]
#[clap(version = "0.1", author = "Jorge Silva <dosjorgesilva@gmail.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser, Debug, Clone)]
pub enum SubCommand {
    #[clap(about = "Preprocess a file or directory")]
    Clean(CommandLine),
    #[clap(about = "Start a preprocessing server")]
    Serve(Serve),
}

fn main() {
    // Set default logging policy.
    // See: https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html
    if std::env::var_os("RUST_LOG").is_none() {
        if cfg!(debug_assertions) {
            std::env::set_var("RUST_LOG", "trace");
        } else {
            std::env::set_var("RUST_LOG", "preproc=info,tower_http=info");
        }
    }
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Clean(cli_opts) => cli::run(cli_opts),
        SubCommand::Serve(serve_opts) => server::run(serve_opts),
    }
}
