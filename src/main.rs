mod cli;
mod commands;
mod storage;
mod tui;

use crate::storage::FileStorage;
use clap::Parser;

fn main() {
    let args = cli::Cli::parse();

    let path = if args.local {
        "todo.json"
    } else {
        "~/.todo.json"
    };

    let storage = FileStorage::new(path);
    commands::edit::run(storage);
}
