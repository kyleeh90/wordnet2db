use anyhow::Result;
use crate::dictionary_handler::WordData;
use rusqlite::{Connection, Transaction};
use std::collections::{hash_map::Entry, HashMap};
use std::path::Path;


pub fn create_word_database(output_path: &Path, word_data: WordData) -> Result<()>{
    // Print status message
    println!("Creating database...");

    let (definitions, words) = word_data;

    // Open a connection
    let mut conn: Connection = Connection::open(output_path.join("dictionary.sqlite3"))?;

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
                        insert_definition.execute([&definition.data, &definition.part_of_speech])?;

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