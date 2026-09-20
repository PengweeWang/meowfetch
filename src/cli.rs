//! Command-line arguments.

use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "meowfetch",
    version,
    about = "A cute system information fetch tool"
)]
pub(crate) struct Args {
    /// Which built-in cat logo to show (1-3).
    #[arg(short = 't', long = "type", value_name = "N", default_value_t = 3)]
    pub(crate) cat_type: u8,

    /// Use a custom logo file instead of the built-in cats.
    #[arg(long = "logo", value_name = "PATH")]
    pub(crate) logo_file: Option<PathBuf>,

    /// Disable colored output.
    #[arg(long = "no-color")]
    pub(crate) no_color: bool,
}
