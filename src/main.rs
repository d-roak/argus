
extern crate dotenv;
use dotenv::dotenv;

mod crossterm;
mod global;
mod screens;

use crate::crossterm::run;

use argh::FromArgs;
use std::{error::Error, time::Duration};

#[derive(Debug, FromArgs)]
#[argh(description = "Client for ratatui")]
struct Cli {
    /// time in ms between two ticks.
    #[argh(option, default = "250")]
    tick_rate: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    run(tick_rate)?;
    Ok(())
}
