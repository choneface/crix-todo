mod cli;
mod commands;
mod storage;
mod tui;

use crate::storage::FileStorage;

fn main() {
    let storage = FileStorage::new("todo.json");
    commands::edit::run(storage);
}
