use tokio_rusqlite::Connection;
use std::error::Error;
use std::env::current_dir;

mod api;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", current_dir());
    let conn = Connection::open("./db/app.db").await?;

    initialize_database(&conn).await;

    Ok(())
}

async fn initialize_database(conn: &Connection){
    let _ = conn.call(|conn| {
        let _ = conn.execute(
            "
            CREATE TABLE IF NOT EXISTS skill_trees (
                id integer primary key autoincrement,
                title text not null,
                description text
            );
            ",
            []
        );
        let _ = conn.execute(
            "
            CREATE TABLE IF NOT EXISTS skill_tree_node (
                id integer primary key autoincrement,
                skill_tree_id integer references skill_trees not null,
                parent_id integer references skill_tree_node,
                markdown text
            );
            ",
            []
        );
        Ok(())
    }).await;
} 