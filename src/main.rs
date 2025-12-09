mod scan_sets;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use scan_sets::{ScanSet, ScanSetPreview, load_config_from_path};

#[derive(Parser)]
#[command(author, version, about = "Hela scan set manager")]
struct Cli {
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand)]
enum Command {
	ScanSet {
		#[command(subcommand)]
		command: ScanSetCommand,
	},
}

#[derive(Subcommand)]
enum ScanSetCommand {
	Preview {
		#[arg(long, value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
		file: PathBuf,
	},
}

fn main() -> Result<()> {
	let cli = Cli::parse();
	match cli.command {
		Command::ScanSet { command } => handle_scan_set(command),
	}
}

fn handle_scan_set(command: ScanSetCommand) -> Result<()> {
	match command {
		ScanSetCommand::Preview { file } => {
			let config = load_config_from_path(&file)?;
			let scan_set = ScanSet::try_from(config)?;
			let preview = ScanSetPreview::from_scan_set(&scan_set);
			let serialized = serde_json::to_string_pretty(&preview)?;
			println!("{}", serialized);
			Ok(())
		}
	}
}
