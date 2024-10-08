use anyhow::Result;
use crate::Args;
use crate::file_handler::IndexDataPair;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;

pub type Definitions = HashMap<u64, Definition>;
pub type Words = BTreeMap<String, HashSet<u64>>;
pub type WordData = (Definitions, Words);

#[derive(Serialize, Deserialize)]
struct WordDataJson{
    word: String,
    definitions: Vec<Definition>
}

#[derive(Serialize, Deserialize)]
pub struct Definition{
    pub data: String,
    pub part_of_speech: String
}

pub fn get_word_data(index_data_pairs: &Vec<IndexDataPair>, args: &Args) -> Result<WordData>{
    // Print status message
    println!("Getting words and definitions...");

    // Create definition and word collections
    let mut definitions: Definitions = Definitions::new();
    let mut words: Words = Words::new();

    // Create a regex for getting the definition
    let definition_regex: Regex = Regex::new(r"\|\s{1}(?<definition>[^;]+[^\s;]+)")?;

    // Create a regex to detect any number
    let number_regex: Regex = Regex::new(r"\d")?;

    // Create regex to detect byte offsets
    let offset_regex: Regex = Regex::new(r"\s{1}(?<offset>\d{8})")?;

    // Create a regex to detect whole words
    let whole_regex: Regex = Regex::new(r"[[:punct:]]|\s")?;

    // Loop through files and update dictionary
    for pair in index_data_pairs{
        // Get a reader for reading the lines from the data
        let mut data_reader: BufReader<File> = BufReader::new(File::open(pair.get_data_path())?);

        // Create a string to hold the current line of the data file
        let mut data_current_line: String = String::new();

        // Create a reader for reading the lines from the index
        let mut index_reader: BufReader<File> = BufReader::new(File::open(pair.get_index_path())?);

        // Create a string to store current line
        let mut index_line: String = String::new();

        // Loop until EOF in index
        while index_reader.read_line(&mut index_line)? != 0{
            // Skip license lines (start with two spaces)
            if index_line.starts_with("  "){
                index_line.clear();
                continue;
            }

            // Get the word (double up on single quotes if present)
            let found_word: String = index_line.split(' ').collect::<Vec<&str>>()[0].to_string().replace("'", "''");

            // Check for numbers
            if !args.keep_numbers && number_regex.is_match(&found_word){
                index_line.clear();
                continue;
            }

            // Check for whole words
            if args.only_whole_words && whole_regex.is_match(&found_word){
                index_line.clear();
                continue;
            }

            // Check word length against arguments
            let word_length: usize = found_word.len();

            if args.char_counts.len() > 0{
                if !args.char_counts.contains(&word_length){
                    index_line.clear();
                    continue;
                }
            } else if word_length < args.min_chars || word_length > args.max_chars{
                index_line.clear();
                continue;
            }

            // Get every offset as a u64
            let byte_offsets: Vec<u64> = offset_regex.captures_iter(&index_line)
                .filter_map(|captures| captures["offset"].parse::<u64>().ok())
                .collect();

            // Modify if present
            if let Some(entry) = words.get_mut(&found_word){
                for offset in byte_offsets.clone(){
                    entry.insert(offset);
                }
            } else{
                // Add it otherwise
                words.insert(found_word, HashSet::from_iter(byte_offsets.clone()));
            }

            // Loop over vector and add them to the HashMap if not present
            for offset in byte_offsets{
                // Seek to the byte offset in the data file
                data_reader.seek(SeekFrom::Start(offset))?;

                // Read the line at the offset
                data_reader.read_line(&mut data_current_line)?;

               // Create an empty definition string
               let mut definition: String = String::new();

               // Get any definition that was found (double up on single quotes if found)
               if let Some(captures) = definition_regex.captures(&data_current_line){
                   definition = captures["definition"].to_string().replace("'", "''");
               }

               // Add it to the definitions if it wasn't present
               definitions.entry(offset).or_insert_with(|| Definition { data: definition, part_of_speech: pair.get_part_of_speech().clone() });

               // Clear current line
               data_current_line.clear();
           }
           // Clear current line
           index_line.clear();
        }
    }

    Ok((definitions, words))
}


pub fn word_data_to_json(output_path: &Path, word_data: WordData) -> Result<()>{
    // Print status message
    println!("Creating JSON...");

    // Get words and definitions
    let (definitions, words) = word_data;
    
    // Create vec to store words/definitions
    let mut data_vec: Vec<WordDataJson> = vec![];

    // Process words and definitions
    for (word, offsets) in words{
        // Create a WordDataJson object to work on
        let mut word_json: WordDataJson = WordDataJson{word, definitions: vec![]};

        // Get every definition for the word
        for offset in offsets{
            if let Some(definition) = definitions.get(&offset){
                word_json.definitions.push(Definition{data: definition.data.clone(), part_of_speech: definition.part_of_speech.clone()});
            }
        }

        data_vec.push(word_json);
    }

    // Save to file
    let json: String = serde_json::to_string_pretty(&data_vec)?;
    let mut file = File::create(output_path.join("dictionary.json"))?;
    file.write_all(json.as_bytes())?;
    
    Ok(())
}