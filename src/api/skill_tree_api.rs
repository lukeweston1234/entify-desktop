use tokio_rusqlite::{ Connection};
use crate::db::schema::SkillTreeNode;

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