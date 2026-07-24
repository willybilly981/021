use crate::model::{Equipment, EquipSet, SearchCondition};
use crate::data::MASTERS;
use good_lp::{variables, default_solver, variable, Expression, SolverModel, Solution, ProblemVariables};
use std::collections::HashMap;

pub struct Searcher {
    pub condition: SearchCondition,
    pub result_sets: Vec<EquipSet>,
    pub is_canceling: bool,
    
    weapons: Vec<Equipment>,
    heads: Vec<Equipment>,
    bodys: Vec<Equipment>,
    arms: Vec<Equipment>,
    waists: Vec<Equipment>,
    legs: Vec<Equipment>,
    charms: Vec<Equipment>,
}

impl Searcher {
    pub fn new(condition: SearchCondition) -> Self {
        let weapons = if condition.is_specific_weapon {
            let masters = MASTERS.read().unwrap();
            masters.weapons.iter()
                .filter(|w| w.name == condition.weapon_name)
                .cloned()
                .collect()
        } else {
            let masters = MASTERS.read().unwrap();
            // TODO: Artianや派生を考慮したフィルタリング
            masters.weapons.iter()
                .filter(|w| w.weapon_type == condition.weapon_type)
                .cloned()
                .collect()
        };

        let heads = MASTERS.read().unwrap().heads.clone();
        let bodys = MASTERS.read().unwrap().bodys.clone();
        let arms = MASTERS.read().unwrap().arms.clone();
        let waists = MASTERS.read().unwrap().waists.clone();
        let legs = MASTERS.read().unwrap().legs.clone();
        
        let charms = if let Some(ref fix_charm) = condition.fix_charm {
            vec![fix_charm.clone()]
        } else {
            let masters = MASTERS.read().unwrap();
            // TODO: 理論値護石の考慮
            masters.charms.clone()
        };

        Self {
            condition,
            result_sets: Vec::new(),
            is_canceling: false,
            weapons,
            heads,
            bodys,
            arms,
            waists,
            legs,
            charms,
        }
    }

    pub fn exec_search(&mut self, limit: usize) -> bool {
        let target = self.result_sets.len() + limit;
        
        while self.result_sets.len() < target {
            if self.is_canceling {
                return false;
            }

            // --- MILP モデルの構築 ---
            let mut vars = variables!();

            // 変数の定義
            // 各防具、武器、護石は 0 か 1 (使用するかどうか)
            // ここでは変数と装備の対応付けを保持する辞書を想定
            let mut equip_vars = HashMap::new();
            
            // 例: 武器
            for w in &self.weapons {
                equip_vars.insert(&w.name, vars.add(variable().integer().min(0).max(1)));
            }
            for h in &self.heads {
                equip_vars.insert(&h.name, vars.add(variable().integer().min(0).max(1)));
            }
            // ... 同様に body, arm, waist, leg, charm も追加 ...
            
            // 制約式の構築 (good_lp)
            // ex: 武器は1つ
            let weapon_constraint: Expression = self.weapons.iter()
                .map(|w| equip_vars[&w.name])
                .sum();
            
            let mut problem = vars.maximise(0.0 /* TODO: 防御力などの目的関数 */)
                .using(default_solver)
                .with(weapon_constraint.eq(if self.condition.is_specific_weapon { 1 } else { 0..=1 })); // TODO: eq(1) or le(1)

            // TODO: 他の制約式を追加 (各部位1つ以下、スロット数計算、スキル計算、耐性計算など)
            
            // 前回までの検索結果を除外する制約 (No-good cut)
            for set in &self.result_sets {
                let existing_equips = set.existing_equips_with_out_decos();
                let sum_expr: Expression = existing_equips.iter()
                    .filter_map(|e| equip_vars.get(&e.name).copied())
                    .sum();
                problem = problem.with(sum_expr.le((existing_equips.len() - 1) as i32));
            }

            // ソルバー実行
            let solution = match problem.solve() {
                Ok(sol) => sol,
                Err(_) => {
                    // 解なし、検索終了
                    return true;
                }
            };

            // 結果から EquipSet を構築
            let mut new_set = EquipSet::new();
            let mut has_data = false;
            for (name, var) in &equip_vars {
                if solution.value(*var) > 0.5 {
                    // 採用された装備
                    let masters = MASTERS.read().unwrap();
                    if let Some(equip) = masters.get_equip_by_name(name) {
                        match equip.kind {
                            crate::model::EquipKind::Weapon => new_set.weapon = equip.clone(),
                            crate::model::EquipKind::Head => new_set.head = equip.clone(),
                            crate::model::EquipKind::Body => new_set.body = equip.clone(),
                            crate::model::EquipKind::Arm => new_set.arm = equip.clone(),
                            crate::model::EquipKind::Waist => new_set.waist = equip.clone(),
                            crate::model::EquipKind::Leg => new_set.leg = equip.clone(),
                            crate::model::EquipKind::Charm => new_set.charm = equip.clone(),
                            crate::model::EquipKind::Deco => {
                                let count = solution.value(*var).round() as i32;
                                for _ in 0..count {
                                    new_set.decos.push(equip.clone());
                                }
                            }
                            _ => {}
                        }
                        has_data = true;
                    }
                }
            }

            if has_data {
                new_set.sort_decos();
                self.result_sets.push(new_set);
            } else {
                return true; // 空データなら終了
            }
        }
        false
    }
}
