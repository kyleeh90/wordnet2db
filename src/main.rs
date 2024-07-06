mod dictionary_handler;
mod file_handler;

use anyhow::{bail, Result};
use clap::Parser;
use colored::Colorize;
use dictionary_handler::WordData;
use file_handler::IndexDataPair;
use rusqlite::{Connection, Transaction};
use std::collections::{hash_map::Entry, HashMap};
use std::env::current_dir;
use std::path::{Path, PathBuf};
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
    /// Only keep words without punctuation or spaces
    #[arg(short = 'W', long, default_value_t = false)]
    only_whole_words: bool
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
            if file_handler::is_valid_dir(&output)?{
                create_database(&output, word_data)?;
            }
        } else{
            create_database(&current_dir()?, word_data)?;
        }
    }

    // Print status message
    println!("{}", String::from("Database created successfully!").green());

    // Print benchmark
    println!("Program complete in {:.2?}", start_time.elapsed());

    Ok(())
}


fn create_database(output_path: &Path, word_data: WordData) -> Result<()>{
    // Print status message
    println!("Creating database...");

    let (definitions, words) = word_data;

    // Open a connection
    let mut conn: Connection = Connection::open(output_path.join("dictionary.sqlite"))?;

    // Start a transaction
    let transaction: Transaction = conn.transaction()?;

    {
        // Create tables
        transaction.execute(
            "CREATE TABLE definition (
                    id              INTEGER PRIMARY KEY AUTOINCREMENT,
                    definition      TEXT,
                    part_of_speech  TEXT NOT NULL
            )",
            ()
        )?;

        transaction.execute(
            "CREATE TABLE word (
                    id      INTEGER PRIMARY KEY AUTOINCREMENT,
                    data    TEXT NOT NULL
            )",
            ()
        )?;

        transaction.execute(
            "CREATE TABLE word_definition (
                    id              INTEGER PRIMARY KEY AUTOINCREMENT,
                    definition_id   INTEGER,
                    word_id         INTEGER,
                    FOREIGN KEY (definition_id) REFERENCES definition(id),
                    FOREIGN KEY (word_id) REFERENCES word(id)
            )",
            ()
        )?;

        // Prepare insert statements
        let mut insert_definition = transaction.prepare("INSERT INTO definition (definition, part_of_speech) VALUES (?, ?)")?;
        let mut insert_word = transaction.prepare("INSERT INTO word (data) VALUES (?)")?;
        let mut insert_word_definition = transaction.prepare("INSERT INTO word_definition (definition_id, word_id) VALUES (?, ?)")?;

        // Create a hashmap to store offset and id values
        let mut definition_ids: HashMap<u64, i64> = HashMap::new();

        // Create an integer to store the IDs
        let mut definition_id: i64 = 1;
        let mut word_id: i64 = 1;

        // Process words and definitions
        for (word, offsets) in words{
            // Insert word
            insert_word.execute([&word])?;

            // Insert definitions
            for offset in offsets{
                if let Entry::Vacant(e) = definition_ids.entry(offset){
                    // Get definition
                    let definition_option = definitions.get(&offset);

                    if let Some(definition) = definition_option{
                        // Insert definition
                        insert_definition.execute([&definition.definition, &definition.part_of_speech])?;

                        // Create an offset/id association
                        e.insert(definition_id);

                        // Increment definition id
                        definition_id += 1;
                    }
                }

                // Add entry to associative table
                if let Some(id) = definition_ids.get(&offset){
                    insert_word_definition.execute([id, &word_id])?;
                }
            }

            // Increment word id
            word_id += 1;
        }
    }

    // Commit transaction
    transaction.commit()?;

    Ok(())
}