use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct SkillTree {
    id: i32,
    title: String,
    description: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillTreeNode {
    id: i32,
    title: String,
    parent_id: Option<i32>,
    markdown: String,
    depth: Option<i32>
}
