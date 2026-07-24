use std::sync::RwLock;
use crate::model::{Equipment, Skill, WeaponType};

lazy_static::lazy_static! {
    pub static ref MASTERS: RwLock<Masters> = RwLock::new(Masters::new());
}

#[derive(Debug, Default)]
pub struct Masters {
    pub skills: Vec<Skill>,
    pub weapons: Vec<Equipment>,
    pub heads: Vec<Equipment>,
    pub bodys: Vec<Equipment>,
    pub arms: Vec<Equipment>,
    pub waists: Vec<Equipment>,
    pub legs: Vec<Equipment>,
    pub charms: Vec<Equipment>,
    pub decos: Vec<Equipment>,
    // TODO: cludes (固定/除外), 追加護石など必要に応じて追加
}

impl Masters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn skill_max_level(&self, name: &str) -> i32 {
        self.skills.iter()
            .find(|s| s.name == name)
            .map(|s| {
                if s.specific_names.is_empty() {
                    s.level
                } else {
                    *s.specific_names.keys().max().unwrap_or(&s.level)
                }
            })
            .unwrap_or(0)
    }

    pub fn get_equip_by_name(&self, name: &str) -> Option<Equipment> {
        self.heads.iter()
            .chain(self.bodys.iter())
            .chain(self.arms.iter())
            .chain(self.waists.iter())
            .chain(self.legs.iter())
            .chain(self.charms.iter())
            .chain(self.decos.iter())
            .chain(self.weapons.iter())
            .find(|e| e.name == name)
            .cloned()
    }
}
