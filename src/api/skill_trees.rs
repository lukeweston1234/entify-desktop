use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillTree {
    id: i32,
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillTreeNode {
    id: i32,
    skill_tree_id: i32,
    title: String,
    markdown: String,
    parent_node: Option<i32>,
    depth: Option<i32>
}