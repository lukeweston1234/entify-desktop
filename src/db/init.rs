use tokio_rusqlite::Connection;
use std::error::Error;

pub async fn initialize_database(conn: &Connection) -> Result<(), Box<dyn Error>> {
    conn.call(|conn| {
        // Create skill_trees table
        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS skill_trees (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT
            );
            ",
            [],
        ).map_err(|e| {
            eprintln!("Error creating skill_trees table: {:?}", e);
            e
        })?;

        // Create skill_tree_node table
        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS skill_tree_node (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                skill_tree_id INTEGER REFERENCES skill_trees NOT NULL,
                parent_id INTEGER REFERENCES skill_tree_node,
                markdown TEXT
            );
            ",
            [],
        ).map_err(|e| {
            eprintln!("Error creating skill_tree_node table: {:?}", e);
            e
        })?;

        Ok(())
    }).await?;

    Ok(())
}