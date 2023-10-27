mod archive;
mod args;
mod builder;
mod handlers;
mod lzrw;
mod utils;

use args::Arguments;
use clap::Parser;
use handlers::{handle_create, handle_extract};

fn main() {
    if colored::control::set_virtual_terminal(true).is_err() {
        utils::display_warning("failed to set virtual terminal.");
    }

    let arguments: Arguments = Arguments::parse();

    if arguments.create as u8 + arguments.extract as u8 > 1 {
        utils::display_error("only one operation can be specified.");
    }

    if let Err(e) = match arguments.create {
        true => handle_create(&arguments),
        _ => handle_extract(&arguments),
    } {
        utils::display_error(&format!("invalid archive - {}.", e))
    }
}
