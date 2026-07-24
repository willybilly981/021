use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Skill {
    pub name: String,
    pub level: i32,
    pub is_fixed: bool,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Equipment {
    pub name: String,
    pub equip_type: String,
    pub defense: i32,
    pub skills: Vec<Skill>,
    pub slots: Vec<i32>,
}
