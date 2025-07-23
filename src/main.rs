use std::{thread, time::Duration};

use clap::Parser;
use passlock_lib::config::Config;
use passlock_lib::{Cli, Result};

fn main() -> Result<()> {
    let mut config = Config::new()?;
    let args = Cli::parse();

    args.command.process_command(&mut config)?;
    thread::sleep(Duration::from_secs(12));

    Ok(())
}
