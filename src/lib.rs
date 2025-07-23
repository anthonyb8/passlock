pub mod cli;
pub mod config;
pub mod entry;
pub mod error;

pub use error::{Error, Result};

use crate::config::Config;
use clap::{Parser, Subcommand};
use clap_builder::builder::styling::{AnsiColor, Styles};
use cli::{CreateArgs, DeleteArgs, ReadArgs, UpdateArgs};

pub const CLAP_STYLING: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Cyan.on_default())
    .placeholder(AnsiColor::Cyan.on_default());

//  Cli commands
#[derive(Debug, Parser)]
#[command(styles = CLAP_STYLING)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create new entry
    Create(CreateArgs),
    /// Read password.
    Read(ReadArgs),
    /// Update entry
    Update(UpdateArgs),
    /// Delete entry or instance of entry.
    Delete(DeleteArgs),
}

impl Commands {
    pub fn process_command(&self, config: &mut Config) -> Result<()> {
        match self {
            Commands::Create(args) => Ok(args.process_command(config)?),
            Commands::Read(args) => Ok(args.process_command(config)?),
            Commands::Update(args) => Ok(args.process_command(config)?),
            Commands::Delete(args) => Ok(args.process_command(config)?),
        }
    }
}
