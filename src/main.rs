mod dictionary_handler;
mod file_handler;

use anyhow::Result;
use clap::Parser;
use file_handler::IndexDataPair;
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
    /// Keep words with numbers
    #[arg(short, long, default_value_t = false)]
    keep_numbers: bool,
    /// Only keep words without punctuation or spaces
    #[arg(short, long, default_value_t = false)]
    only_whole_words: bool
}

fn main() -> Result<()> {
    // Parse arguments
    let args: Args = Args::parse();

    // Verify input directory is valid before proceeding
    if file_handler::is_valid_dir(&args.directory)?{
        // Get file paths
        let path_pairs: Vec<IndexDataPair> = file_handler::get_paths(&args.directory)?;

        // Get word data
        dictionary_handler::get_word_data(&path_pairs, &args)?;
    }

    Ok(())
}