mod db_handler;
mod dictionary_handler;
mod file_handler;

use anyhow::{bail, Result};
use clap::Parser;
use colored::Colorize;
use dictionary_handler::WordData;
use file_handler::IndexDataPair;
use std::env::current_dir;
use std::path::PathBuf;
use std::time::Instant;

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
    /// Minimum character count of a word to save (default: 0)
    #[arg(short = 'm', long, default_value_t = 0)]
    min_chars: usize,
    /// Maximum character count of a word to save (default: 45)
    #[arg(short = 'M', long, default_value_t = 45)]
    max_chars: usize,
    /// Keep words with numbers
    #[arg(short, long, default_value_t = false)]
    keep_numbers: bool,
    /// Comma seperated list of character counts to save
    #[arg(short, long, value_delimiter = ',', num_args = 0.., conflicts_with_all = ["min_chars", "max_chars"])]
    char_counts: Vec<usize>,
    /// Only keep words without punctuation or spaces
    #[arg(short = 'W', long, default_value_t = false)]
    only_whole_words: bool,
    /// Renders database as SQL statements rather than an SQLite database
    #[arg(short, long, default_value_t = false)]
    dump_sql: bool
}


fn main() -> Result<()> {
    // Start benchmark
    let start_time: Instant = Instant::now();

    // Parse arguments
    let args: Args = Args::parse();

    // Verify input directory is valid before proceeding
    if file_handler::is_valid_dir(&args.directory)?{
        // Get file paths
        let path_pairs: Vec<IndexDataPair> = file_handler::get_paths(&args.directory)?;

        // Get word data
        let word_data: WordData = dictionary_handler::get_word_data(&path_pairs, &args)?;

        // Throw an error if no words found
        if word_data.1.is_empty(){
            bail!("No words found for given arguments!".red())
        }

        // Check if an output path is specified and is valid and create database
        if let Some(output) = args.output_directory{
            if file_handler::is_valid_dir(&output)? && args.dump_sql{
                db_handler::dump_sql(&output, word_data)?;
            } else{
                db_handler::create_word_database(&output, word_data)?;
            }
        } else{
            if args.dump_sql{
                db_handler::dump_sql(&current_dir()?, word_data)?;
            } else{
                db_handler::create_word_database(&current_dir()?, word_data)?;
            }
        }
    }

    // Print status message
    if args.dump_sql{
        println!("{}", String::from("SQL created successfully!").green());
    } else{
        println!("{}", String::from("Database created successfully!").green());
    }

    // Print benchmark
    println!("Program complete in {:.2?}", start_time.elapsed());

    Ok(())
}