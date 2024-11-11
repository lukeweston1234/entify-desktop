use tokio_rusqlite::{ Connection, params };
use crate::db::schema::{SkillTree, SkillTreeNode};

pub async fn get_skill_tree_nodes(conn: &Connection, skill_tree_id: i32) -> Result<Vec<SkillTreeNode>, tokio_rusqlite::Error> {
    conn.call(move |conn|{
        let mut stmt = conn.prepare(
            "
            with recursive scope_object_tree as (
                select
                    p.id, p.title, p.markdown, p.parent_id, p.skill_tree_id,
                    1 as level
                from skill_tree_node p
                where p.parent_id is null
    
            union all
    
            select c.id, c.title, c.markdown, c.parent_id, c.skill_tree_id, scope_object_tree.level + 1 as level
            from skill_tree_node c
            join scope_object_tree on c.parent_id = scope_object_tree.id
            )
            select * from scope_object_tree where skill_tree_id = ?1;
            ",
        )?;
    
        let skill_tree_nodes = stmt
            .query_map([skill_tree_id], |row| {
                Ok(SkillTreeNode {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    markdown: row.get(2)?,
                    parent_id: row.get(3)?,
                    skill_tree_id: row.get(4)?,
                    depth: Some(row.get(5)?),
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
    
        Ok(skill_tree_nodes)
    }).await
}

pub async fn create_skill_tree(conn: &Connection, title: String, description: String) -> Result<i64, tokio_rusqlite::Error>{
    conn.call(move |conn| {
        conn.execute("INSERT INTO TABLE skill_trees (title, description) values (?1, ?2)", params![title, description])?;
        Ok(conn.last_insert_rowid() as i64)
    })
    .await
}

pub async fn update_skill_tree(conn: &Connection, skill_tree: SkillTree) -> Result<(), tokio_rusqlite::Error> {
    conn.call(move |conn| {
        conn.execute(
            "UPDATE skill_trees SET title = ?1, description = ?2 WHERE id = ?3",
            params![skill_tree.title, skill_tree.description, skill_tree.id]
        ).map(|_| ()).map_err(|e| e.into())
    }).await
}

pub async fn delete_skill_tree(conn: &Connection, skill_tree_id: i32) -> Result<(), tokio_rusqlite::Error> {
    conn.call(move |conn| {
        conn.execute("DELETE FROM skill_trees WHERE id = ?1", params![skill_tree_id])
            .map(|_| ())
            .map_err(|e| e.into())
    }).await
}

