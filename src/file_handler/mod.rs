use anyhow::{bail, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

pub struct IndexDataPair{
    index_path: PathBuf,
    data_path: PathBuf
}

impl IndexDataPair{
    pub fn new(index_path: PathBuf, data_path: PathBuf) -> IndexDataPair{
        IndexDataPair { index_path, data_path }
    }

    pub fn get_data_path(&self) -> &PathBuf{
        &self.data_path
    }

    pub fn get_index_path(&self) -> &PathBuf{
        &self.index_path
    }
}


pub fn get_paths(dir_path: &Path) -> Result<Vec<IndexDataPair>>{
    // Print status message
    println!("Searching for WordNet files...");
    
    // Create string for the index file we don't need
    let ignored_index: &str = "index.sense";

    // Create mutable vector for index paths
    let mut index_paths: Vec<PathBuf> = vec![];

    // Create mutable vector for data paths
    let mut data_paths: Vec<PathBuf> = vec![];

    // Loop through the files searching for index files
    for file_entry in fs::read_dir(dir_path)?.filter_map(|entry| entry.ok()){
        // Get file name
        let file_name: String = file_entry.file_name().to_string_lossy().to_string();

        // Skip iteration if ignored_index is found
        if file_name == ignored_index{
            continue;
        }

        // Add path to vector if file name contains index
        if file_name.contains("index"){
            println!("Found {}...", file_name.green());
            index_paths.push(file_entry.path());
        }

        // Add path to vector if file name contains data
        if file_name.contains("data"){
            println!("Found {}...", file_name.green());
            data_paths.push(file_entry.path());
        }
    }

    // Create a vector to be returned of IndexDataPair
    let mut index_data_vec: Vec<IndexDataPair> = vec![];

    // Match files based on extension
    for index_path in &index_paths{
        for data_path in &data_paths{
            if data_path.extension() == index_path.extension(){
                index_data_vec.push(IndexDataPair::new(index_path.to_owned(), data_path.to_owned()));
                continue;
            }
        }
    }

    Ok(index_data_vec)
}


pub fn is_valid_dir(dir_path: &Path) -> Result<bool>{
    // Check if directory exists. Needs to have multiple error messages in case existence can't be confirmed or denied
    if dir_path.try_exists().is_err(){
        bail!(format!("Unable to check existence of {}", dir_path.to_string_lossy()).red())
    } else if dir_path.try_exists().is_ok_and(| exists | !exists){
        bail!(format!("{} does not exist!", dir_path.to_string_lossy()).red())
    }

    // Check if input is a directory. Throw error if is isn't.
    if !dir_path.is_dir(){
        bail!(format!("{} is not a directory!", dir_path.to_string_lossy()).red())
    }

    Ok(true)
}