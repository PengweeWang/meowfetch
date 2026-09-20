//! meowfetch: a cute system information fetch tool.

mod art;
mod cli;
mod info;
mod logo;
mod render;

use clap::Parser;

use crate::cli::Args;

fn main() {
    let args = Args::parse();

    if args.no_color {
        colored::control::set_override(false);
    }

    let logo = logo::load(&args);
    let rows = info::collect();
    render::render(&logo, &rows);
}
