mod cli;
mod profile;
mod utils;

use crate::utils::Result;

use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    Cli::parse() //
        .validate_args()?
        .execute()
}
