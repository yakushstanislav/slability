use std::error::Error;

use clap::Parser;

use slability::application;
use slability::config::Config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    application::run(config)?;

    Ok(())
}
