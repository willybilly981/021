use crate::data::masters::MASTERS;
use crate::model::{Equipment, EquipKind, Skill, WeaponType};
use std::error::Error;
use std::path::Path;

// TODO: 実際のCSVのフォーマットに合わせてSerde構造体を定義し、読み込む処理を実装する
pub fn load_def_upgrade_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    // let mut rdr = csv::Reader::from_path(path)?;
    // for result in rdr.deserialize() { ... }
    Ok(())
}

pub fn load_head_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn load_body_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn load_arm_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn load_waist_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn load_leg_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn load_charm_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn load_deco_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn load_weapon_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn load_skill_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    Ok(())
}
