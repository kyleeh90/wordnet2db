use anyhow::Result;
use crate::dictionary_handler::WordData;
use rusqlite::{Connection, Transaction};
use std::collections::{hash_map::Entry, HashMap};
use std::fs::File;
use std::io::Write;
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
                    data            TEXT,
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
        let mut insert_definition = transaction.prepare("INSERT INTO definition (data, part_of_speech) VALUES (?, ?)")?;
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


pub fn dump_sql(output_path: &Path, word_data: WordData) -> Result<()>{
    // Print status message
    println!("Creating SQL...");

    // Get words and definitions
    let (definitions, words) = word_data;

    // Create a hashmap to store offset and id values
    let mut definition_ids: HashMap<u64, i64> = HashMap::new();

    // Create an integer to store the IDs
    let mut definition_id: i64 = 1;
    let mut word_id: i64 = 1;
    let mut word_definition_id: i64 = 1;

    // Create string to store SQL
    let mut sql: String = String::new();

    // Write create statements
    sql.push_str("PRAGMA defer_foreign_keys=ON;\n");

    sql.push_str("BEGIN TRANSACTION;\n");

    sql.push_str("CREATE TABLE definition (
                            id              INTEGER PRIMARY KEY AUTOINCREMENT,
                            data            TEXT,
                            part_of_speech  TEXT NOT NULL
    );\n");

    sql.push_str("CREATE TABLE word (
                            id      INTEGER PRIMARY KEY AUTOINCREMENT,
                            data    TEXT NOT NULL
    );\n");

    sql.push_str("CREATE TABLE word_definition (
                            id              INTEGER PRIMARY KEY AUTOINCREMENT,
                            definition_id   INTEGER,
                            word_id         INTEGER,
                            FOREIGN KEY (definition_id) REFERENCES definition(id),
                            FOREIGN KEY (word_id) REFERENCES word(id)
    );\n");

    // Process words and definitions
    for (word, offsets) in words{
        // Insert word
        sql.push_str(&format!("INSERT INTO word VALUES({},'{}');\n", word_id, word));

        // Insert definitions
        for offset in offsets{
            if let Entry::Vacant(e) = definition_ids.entry(offset){
                // Get definition
                let definition_option = definitions.get(&offset);

                if let Some(definition) = definition_option{
                    // Insert definition
                    sql.push_str(&format!("INSERT INTO definition VALUES({},'{}','{}');\n", definition_id, definition.data, definition.part_of_speech));

                    // Create an offset/id association
                    e.insert(definition_id);

                    // Increment definition id
                    definition_id += 1;
                }
            }

            // Add entry to associative table
            if let Some(id) = definition_ids.get(&offset){
                sql.push_str(&format!("INSERT INTO word_definition VALUES({},{},{});\n", word_definition_id, id, word_id));
                word_definition_id += 1;
            }
        }

        // Increment word id
        word_id += 1;
    }

    // Add extra necessary SQL after loop
    sql.push_str("DELETE FROM sqlite_sequence;\n");
    sql.push_str(&format!("INSERT INTO sqlite_sequence VALUES('definition',{});\n", definition_id - 1));
    sql.push_str(&format!("INSERT INTO sqlite_sequence VALUES('word',{});\n", word_id - 1));
    sql.push_str(&format!("INSERT INTO sqlite_sequence VALUES('word_definition',{});\n", word_definition_id - 1));
    sql.push_str("COMMIT;");

    // Save to file
    let mut file = File::create(output_path.join("dictionary_dump.sql"))?;
    file.write_all(sql.as_bytes())?;

    Ok(())
}