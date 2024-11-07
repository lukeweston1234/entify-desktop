use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct SkillTree {
    id: i32,
    title: String,
    description: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillTreeNode {
    pub id: i32,
    pub title: String,
    pub parent_id: Option<i32>,
    pub skill_tree_id: i32,
    pub markdown: String,
    pub depth: Option<i32>
}
