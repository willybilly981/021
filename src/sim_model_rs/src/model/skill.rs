use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref DISPLAY_RESTRICT_CATEGORIES: Vec<&'static str> = vec!["グループスキル", "シリーズスキル"];
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Skill {
    pub name: String,
    pub level: i32,
    pub is_fixed: bool,
    pub category: String,
    pub can_with_artian: bool,
    pub specific_names: HashMap<i32, String>,
}

impl Skill {
    pub fn new(name: String, level: i32, category: Option<String>, is_fixed: bool, can_with_artian: bool, specific_names: HashMap<i32, String>) -> Self {
        Self {
            name,
            level,
            is_fixed,
            category: category.unwrap_or_else(|| "未分類".to_string()),
            can_with_artian,
            specific_names,
        }
    }

    pub fn description(&self) -> String {
        if self.name.is_empty() || self.level == 0 {
            return String::new();
        }
        if let Some(specific_name) = self.specific_names.get(&self.level) {
            format!("{}({}Lv{})", specific_name, self.name, self.level)
        } else {
            format!("{}Lv{}", self.name, self.level)
        }
    }

    pub fn is_hide_level(&self, level: Option<i32>) -> bool {
        let check_level = level.unwrap_or(self.level);
        DISPLAY_RESTRICT_CATEGORIES.contains(&self.category.as_str())
            && !self.specific_names.contains_key(&check_level)
    }
}
