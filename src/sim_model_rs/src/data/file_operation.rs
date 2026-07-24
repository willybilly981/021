use crate::data::masters::MASTERS;
use crate::model::{Equipment, EquipKind, Skill, WeaponType};
use std::error::Error;
use std::path::Path;
use std::collections::HashMap;

// CSVのデシリアライズ用構造体 (防具用)
#[derive(serde::Deserialize)]
struct EquipCsvRow {
    #[serde(rename = "名前")] // 実際のCSVヘッダに合わせて変更
    name: String,
    #[serde(rename = "レア度")]
    rare: i32,
    #[serde(rename = "スロット1")]
    slot1: i32,
    #[serde(rename = "スロット2")]
    slot2: i32,
    #[serde(rename = "スロット3")]
    slot3: i32,
    // ... 他のフィールド ...
}

pub fn load_equip_csv<P: AsRef<Path>>(path: P, kind: EquipKind) -> Result<Vec<Equipment>, Box<dyn Error>> {
    let mut equips = Vec::new();
    // 実際はシフトJISなどのエンコーディング対応が必要な場合があります
    // let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
    // for result in rdr.deserialize::<EquipCsvRow>() {
    //     let record = result?;
    //     let equip = Equipment {
    //         name: record.name,
    //         rare: record.rare,
    //         slot1: record.slot1,
    //         slot2: record.slot2,
    //         slot3: record.slot3,
    //         kind,
    //         ..Equipment::new()
    //     };
    //     equips.push(equip);
    // }
    Ok(equips)
}

pub fn load_head_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let equips = load_equip_csv(path, EquipKind::Head)?;
    MASTERS.write().unwrap().heads = equips;
    Ok(())
}

pub fn load_body_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let equips = load_equip_csv(path, EquipKind::Body)?;
    MASTERS.write().unwrap().bodys = equips;
    Ok(())
}

pub fn load_arm_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let equips = load_equip_csv(path, EquipKind::Arm)?;
    MASTERS.write().unwrap().arms = equips;
    Ok(())
}

pub fn load_waist_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let equips = load_equip_csv(path, EquipKind::Waist)?;
    MASTERS.write().unwrap().waists = equips;
    Ok(())
}

pub fn load_leg_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let equips = load_equip_csv(path, EquipKind::Leg)?;
    MASTERS.write().unwrap().legs = equips;
    Ok(())
}

// ... 同様に weapon, charm, deco, skill を実装 ...

