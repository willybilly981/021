use super::{EquipKind, Skill};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Equipment {
    pub name: String,
    pub rare: i32,
    pub slot1: i32,
    pub slot2: i32,
    pub slot3: i32,
    pub slot_type1: i32, // 0: 防御, 1: 攻撃, 2: 両方
    pub slot_type2: i32,
    pub slot_type3: i32,
    pub mindef: i32,
    pub maxdef: i32,
    pub transcending_def: i32,
    pub fire: i32,
    pub water: i32,
    pub thunder: i32,
    pub ice: i32,
    pub dragon: i32,
    pub is_one_set: bool,
    pub row_no: i32,
    pub skills: Vec<Skill>,
    pub kind: EquipKind,
    pub is_virtual: bool,
    pub disp_name_override: Option<String>,
    pub attack: i32,
    pub weapon_type: crate::model::WeaponType,
}

impl Equipment {
    pub fn new() -> Self {
        Self {
            row_no: i32::MAX,
            kind: EquipKind::Error,
            ..Default::default()
        }
    }

    pub fn with_kind(kind: EquipKind) -> Self {
        Self {
            row_no: i32::MAX,
            kind,
            ..Default::default()
        }
    }

    pub fn disp_name(&self) -> &str {
        if let Some(ref disp) = self.disp_name_override {
            disp
        } else {
            &self.name
        }
    }

    pub fn is_transcending_slot_target(&self) -> bool {
        (matches!(
            self.kind,
            EquipKind::Head | EquipKind::Body | EquipKind::Arm | EquipKind::Waist | EquipKind::Leg
        )) && (self.rare == 5 || self.rare == 6)
    }

    pub fn transcending_slot1(&self) -> i32 {
        if !self.is_transcending_slot_target() {
            return self.slot1;
        }
        std::cmp::min(self.slot1 + 1, 3)
    }

    pub fn transcending_slot2(&self) -> i32 {
        if !self.is_transcending_slot_target() {
            return self.slot2;
        }
        std::cmp::min(self.slot2 + 1, 3)
    }

    pub fn transcending_slot3(&self) -> i32 {
        if !self.is_transcending_slot_target() {
            return self.slot3;
        }
        if self.rare == 6 {
            self.slot3
        } else {
            std::cmp::min(self.slot3 + 1, 3)
        }
    }
}
