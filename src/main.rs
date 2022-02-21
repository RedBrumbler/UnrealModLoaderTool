#![feature(windows_process_extensions_raw_arg)]
use clap::{Parser, Subcommand};

mod commands;
mod data;
use commands::{build, copy};

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "RedBrumbler")]
struct Opts {
    #[clap(subcommand)]
    subcmd: MainCommand,
}

#[derive(Subcommand, Debug, Clone)]
enum MainCommand {
    /// cook your project
    Build(build::BuildArgs),
    /// copy over the pak files if you did not specify this in the build command
    Copy,
}

fn main() {
    match (Opts::parse() as Opts).subcmd {
        MainCommand::Build(b) => build::build_game(b),
        MainCommand::Copy => copy::copy_paks(),
    }
}
