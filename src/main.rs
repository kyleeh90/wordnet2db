use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

// Parser setup
#[derive(Parser)]
#[command(name = "WordNet Parser")]
#[command(version = "0.1.0")]
/// Parse Princeton University's WordNet files.
/// 
/// Get a list of English words & definitions by parsing Princeton's WordNet files
/// 
/// Outputs a sqlite database
struct Args {
    /// Directory where WordNet files are located (index.adj, index.adv, data.adj, etc)
    #[arg(short, long)]
    directory: PathBuf,
    /// Directory to place output file into (default: working directory)
    #[arg(short, long)]
    output_directory: Option<PathBuf>,
    /// Keep words with hyphens (default: false)
    #[arg(short = 'H', long, default_value_t = false)]
    keep_hyphens: bool,
    /// Keep words with numbers (default: false)
    #[arg(short = 'N', long, default_value_t = false)]
    keep_numbers: bool,
    /// Keep words with spaces (default: false)
    #[arg(short = 'S', long, default_value_t = false)]
    keep_spaces: bool,
    /// Only keep words without punctuation or spaces (default: true)
    #[arg(short = 'W', long, default_value_t = true)]
    only_whole_words: bool
}

fn main() -> Result<()> {
    // Parse arguments
    let _args: Args = Args::parse();

    Ok(())
}