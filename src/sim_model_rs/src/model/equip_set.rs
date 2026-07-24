use super::{Equipment, Skill, WeaponType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipSet {
    pub weapon: Equipment,
    pub head: Equipment,
    pub body: Equipment,
    pub arm: Equipment,
    pub waist: Equipment,
    pub leg: Equipment,
    pub charm: Equipment,
    pub decos: Vec<Equipment>,
    pub is_transcending: bool,
    pub glpk_row_name: String,
}

impl EquipSet {
    pub fn new() -> Self {
        Self {
            weapon: Equipment::with_kind(crate::model::EquipKind::Weapon),
            head: Equipment::with_kind(crate::model::EquipKind::Head),
            body: Equipment::with_kind(crate::model::EquipKind::Body),
            arm: Equipment::with_kind(crate::model::EquipKind::Arm),
            waist: Equipment::with_kind(crate::model::EquipKind::Waist),
            leg: Equipment::with_kind(crate::model::EquipKind::Leg),
            charm: Equipment::with_kind(crate::model::EquipKind::Charm),
            decos: Vec::new(),
            is_transcending: false,
            glpk_row_name: String::new(),
        }
    }

    pub fn existing_equips_with_out_decos(&self) -> Vec<Equipment> {
        let mut equips = Vec::new();
        if !self.weapon.name.is_empty() {
            equips.push(self.weapon.clone());
        }
        if !self.head.name.is_empty() {
            equips.push(self.head.clone());
        }
        if !self.body.name.is_empty() {
            equips.push(self.body.clone());
        }
        if !self.arm.name.is_empty() {
            equips.push(self.arm.clone());
        }
        if !self.waist.name.is_empty() {
            equips.push(self.waist.clone());
        }
        if !self.leg.name.is_empty() {
            equips.push(self.leg.clone());
        }
        if !self.charm.name.is_empty() {
            equips.push(self.charm.clone());
        }
        equips
    }

    pub fn sort_decos(&mut self) {
        self.decos.sort_by(|a, b| b.slot1.cmp(&a.slot1)); // スロットサイズ降順ソート等の想定
    }
}
