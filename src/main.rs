use api::skill_tree_api::get_skill_tree_nodes;
use db::init::initialize_database;
use tokio_rusqlite::Connection;
use std::error::Error;

mod db;
mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let conn = Connection::open("./db/app.db")
        .await
        .expect("Could not open db");

    initialize_database(&conn).await.expect("Could not initialize db");

    let nodes = get_skill_tree_nodes(&conn, 1).await;

    nodes.into_iter().for_each(|x| println!("{:?}", x));

    Ok(())
}




