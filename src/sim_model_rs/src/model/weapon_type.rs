use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum WeaponType {
    #[default]
    None,
    GreatSword,
    LongSword,
    SwordAndShield,
    DualBlades,
    Lance,
    Gunlance,
    Hammer,
    HuntingHorn,
    SwitchAxe,
    ChargeBlade,
    InsectGlaive,
    LightBowgun,
    HeavyBowgun,
    Bow,
}
