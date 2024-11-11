use tokio_rusqlite::{ Connection, params, OptionalExtension };
use crate::db::schema::{SkillTree, SkillTreeNode};

pub async fn get_skill_tree_node(conn: &Connection, node_id: i32) -> Result<Option<SkillTreeNode>, tokio_rusqlite::Error> {
    conn.call(move |conn| {
        let mut stmt = conn.prepare(
            "SELECT id, title, markdown, parent_id, skill_tree_id, 1 AS level FROM skill_tree_node WHERE id = ?1"
        )?;

        let skill_tree_node = stmt.query_row([node_id], |row| {
            Ok(SkillTreeNode {
                id: row.get(0)?,
                title: row.get(1)?,
                markdown: row.get(2)?,
                parent_id: row.get(3)?,
                skill_tree_id: row.get(4)?,
                depth: Some(row.get(5)?),
            })
        }).optional()?;

        Ok(skill_tree_node)
    }).await
}

pub async fn create_skill_tree_node(conn: &Connection, node: SkillTreeNode) -> Result<i32, tokio_rusqlite::Error> {
    conn.call(move |conn| {
        conn.execute(
            "INSERT INTO skill_tree_node (title, markdown, parent_id, skill_tree_id) VALUES (?1, ?2, ?3, ?4)",
            params![node.title, node.markdown, node.parent_id, node.skill_tree_id]
        )?;
        Ok(conn.last_insert_rowid() as i32)
    }).await
}

pub async fn update_skill_tree_node(conn: &Connection, node: SkillTreeNode) -> Result<(), tokio_rusqlite::Error> {
    conn.call(move |conn| {
        conn.execute(
            "UPDATE skill_tree_node SET title = ?1, markdown = ?2, parent_id = ?3 WHERE id = ?4",
            params![node.title, node.markdown, node.parent_id, node.id]
        )?;
        Ok(())
    }).await
}

pub async fn delete_skill_tree_node(conn: &Connection, id: i32) -> Result<(), tokio_rusqlite::Error> {
    conn.call(move |conn| {
        conn.execute("DELETE FROM skill_tree_node WHERE id = ?1", params![id])?;
        Ok(())
    }).await
}