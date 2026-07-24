use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipKind {
    Head,
    Body,
    Arm,
    Waist,
    Leg,
    Deco,
    Charm,
    Weapon,
    Error,
}

impl EquipKind {
    pub fn to_string(&self) -> &'static str {
        match self {
            EquipKind::Weapon => "武器",
            EquipKind::Head => "頭",
            EquipKind::Body => "胴",
            EquipKind::Arm => "腕",
            EquipKind::Waist => "腰",
            EquipKind::Leg => "足",
            EquipKind::Deco => "装飾品",
            EquipKind::Charm => "護石",
            EquipKind::Error => "",
        }
    }

    pub fn to_string_with_colon(&self) -> String {
        format!("{}：", self.to_string())
    }

    pub fn from_str(s: &str) -> EquipKind {
        match s {
            "頭" => EquipKind::Head,
            "胴" => EquipKind::Body,
            "腕" => EquipKind::Arm,
            "腰" => EquipKind::Waist,
            "足" | "脚" => EquipKind::Leg,
            "装飾品" => EquipKind::Deco,
            "護石" => EquipKind::Charm,
            "武器" => EquipKind::Weapon,
            _ => EquipKind::Error,
        }
    }
}
