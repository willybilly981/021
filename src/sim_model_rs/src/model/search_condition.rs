use super::{Equipment, Skill, WeaponType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCondition {
    pub skills: Vec<Skill>,
    pub is_specific_weapon: bool,
    pub weapon_name: String,
    pub weapon_type: WeaponType,
    pub min_attack: Option<i32>,
    pub def: Option<i32>,
    pub fire: Option<i32>,
    pub water: Option<i32>,
    pub thunder: Option<i32>,
    pub ice: Option<i32>,
    pub dragon: Option<i32>,
    pub fix_charm: Option<Equipment>,
    pub is_best_charm_search: bool,
    pub is_best_artian_search: bool,
    pub is_transcending: bool,
}

impl SearchCondition {
    pub fn new() -> Self {
        Self {
            skills: Vec::new(),
            is_specific_weapon: false,
            weapon_name: String::new(),
            weapon_type: WeaponType::None,
            min_attack: None,
            def: None,
            fire: None,
            water: None,
            thunder: None,
            ice: None,
            dragon: None,
            fix_charm: None,
            is_best_charm_search: false,
            is_best_artian_search: false,
            is_transcending: false,
        }
    }

    pub fn make_related_charms(&self) -> Vec<Equipment> {
        // 理論値護石生成ロジック(仮実装)
        Vec::new()
    }

    pub fn make_related_artians(&self) -> Vec<Equipment> {
        // 理論値アーティア生成ロジック(仮実装)
        Vec::new()
    }

    pub fn add_skill(&mut self, skill: Skill) -> bool {
        if let Some(existing) = self.skills.iter_mut().find(|s| s.name == skill.name) {
            if existing.level < skill.level {
                existing.level = skill.level;
                true
            } else {
                false
            }
        } else {
            self.skills.push(skill);
            true
        }
    }
}
