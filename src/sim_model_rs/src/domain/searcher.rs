use crate::model::{Equipment, EquipSet, SearchCondition};
use crate::data::MASTERS;
use good_lp::{variables, default_solver, variable, Expression, SolverModel, Solution, ProblemVariables, Variable};
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

    fn score(&self, equip: &Equipment) -> i32 {
        let mut slot1 = 0;
        let mut slot2 = 0;
        let mut slot3 = 0;

        if equip.kind != crate::model::EquipKind::Deco {
            slot1 = equip.slot1;
            slot2 = equip.slot2;
            slot3 = equip.slot3;
        } else {
            slot1 = -equip.slot1;
        }

        let mut score = 0;

        // 防御力
        if self.condition.is_transcending {
            score += equip.transcending_def;
        } else {
            score += equip.maxdef;
        }

        // スロット数
        score *= 20;
        score += if slot1 > 0 { 1 } else if slot1 < 0 { -1 } else { 0 };
        score += if slot2 > 0 { 1 } else if slot2 < 0 { -1 } else { 0 };
        score += if slot3 > 0 { 1 } else if slot3 < 0 { -1 } else { 0 };

        // スロット大きさ
        score *= 80;
        score += slot1 + slot2 + slot3;

        score
    }

    fn slot_calc(&self, equip: &Equipment, is_count_weapon: bool, is_count_armor: bool) -> [i32; 4] {
        let (slot1, slot2, slot3) = if self.condition.is_transcending {
            (equip.transcending_slot1(), equip.transcending_slot2(), equip.transcending_slot3())
        } else {
            (equip.slot1, equip.slot2, equip.slot3)
        };

        let mut slot_cond = [0; 4];
        if (is_count_weapon && equip.slot_type1 != 0) || (is_count_armor && equip.slot_type1 != 1) {
            for i in 0..slot1 { if i < 4 { slot_cond[i as usize] += 1; } }
        }
        if (is_count_weapon && equip.slot_type2 != 0) || (is_count_armor && equip.slot_type2 != 1) {
            for i in 0..slot2 { if i < 4 { slot_cond[i as usize] += 1; } }
        }
        if (is_count_weapon && equip.slot_type3 != 0) || (is_count_armor && equip.slot_type3 != 1) {
            for i in 0..slot3 { if i < 4 { slot_cond[i as usize] += 1; } }
        }
        slot_cond
    }

    pub fn exec_search(&mut self, limit: usize) -> bool {
        let target = self.result_sets.len() + limit;
        
        while self.result_sets.len() < target {
            if self.is_canceling {
                return false;
            }

            let mut vars = variables!();
            let mut equip_vars = HashMap::new();
            
            // 各装備の変数を追加
            let add_vars = |equips: &Vec<Equipment>, vars: &mut ProblemVariables, equip_vars: &mut HashMap<String, Variable>| {
                for eq in equips {
                    equip_vars.insert(eq.name.clone(), vars.add(variable().integer().min(0).max(1)));
                }
            };
            
            add_vars(&self.weapons, &mut vars, &mut equip_vars);
            add_vars(&self.heads, &mut vars, &mut equip_vars);
            add_vars(&self.bodys, &mut vars, &mut equip_vars);
            add_vars(&self.arms, &mut vars, &mut equip_vars);
            add_vars(&self.waists, &mut vars, &mut equip_vars);
            add_vars(&self.legs, &mut vars, &mut equip_vars);
            add_vars(&self.charms, &mut vars, &mut equip_vars);
            
            // 装飾品の変数を追加 (所持数を上限とする)
            let decos = MASTERS.read().unwrap().decos.clone();
            for deco in &decos {
                // TODO: 実際の所持数を上限にする
                equip_vars.insert(deco.name.clone(), vars.add(variable().integer().min(0).max(100)));
            }
            
            let all_equips = self.weapons.iter()
                .chain(self.heads.iter())
                .chain(self.bodys.iter())
                .chain(self.arms.iter())
                .chain(self.waists.iter())
                .chain(self.legs.iter())
                .chain(self.charms.iter())
                .chain(decos.iter())
                .collect::<Vec<_>>();

            // 目的関数 (防御力 + スロット等によるスコア)
            let mut obj_expr = Expression::from(0.0);
            for eq in &all_equips {
                if let Some(&var) = equip_vars.get(&eq.name) {
                    obj_expr += (self.score(eq) as f64) * var;
                }
            }
            
            let mut problem = vars.maximise(obj_expr).using(default_solver);

            // 【制約式】 部位ごとの制約
            let eq_sum = |equips: &Vec<Equipment>| -> Expression {
                equips.iter().filter_map(|e| equip_vars.get(&e.name).copied()).sum()
            };
            
            problem = problem.with(eq_sum(&self.weapons).eq(if self.condition.is_specific_weapon { 1 } else { 0..=1 }));
            problem = problem.with(eq_sum(&self.heads).eq(0..=1));
            problem = problem.with(eq_sum(&self.bodys).eq(0..=1));
            problem = problem.with(eq_sum(&self.arms).eq(0..=1));
            problem = problem.with(eq_sum(&self.waists).eq(0..=1));
            problem = problem.with(eq_sum(&self.legs).eq(0..=1));
            problem = problem.with(eq_sum(&self.charms).eq(0..=1));

            // 【制約式】 ステータス
            let stat_sum = |get_stat: fn(&Equipment) -> i32| -> Expression {
                all_equips.iter().filter_map(|e| equip_vars.get(&e.name).map(|&v| (get_stat(e) as f64) * v)).sum()
            };
            problem = problem.with(stat_sum(|e| e.maxdef).ge(self.condition.def.unwrap_or(0) as f64));
            if let Some(fire) = self.condition.fire { problem = problem.with(stat_sum(|e| e.fire).ge(fire as f64)); }
            if let Some(water) = self.condition.water { problem = problem.with(stat_sum(|e| e.water).ge(water as f64)); }
            if let Some(thunder) = self.condition.thunder { problem = problem.with(stat_sum(|e| e.thunder).ge(thunder as f64)); }
            if let Some(ice) = self.condition.ice { problem = problem.with(stat_sum(|e| e.ice).ge(ice as f64)); }
            if let Some(dragon) = self.condition.dragon { problem = problem.with(stat_sum(|e| e.dragon).ge(dragon as f64)); }
            if let Some(min_atk) = self.condition.min_attack { problem = problem.with(stat_sum(|e| e.attack).ge(min_atk as f64)); }

            // 【制約式】 スキル
            for cond_skill in &self.condition.skills {
                let skill_expr: Expression = all_equips.iter()
                    .filter_map(|e| {
                        e.skills.iter().find(|s| s.name == cond_skill.name).and_then(|eq_skill| {
                            equip_vars.get(&e.name).map(|&v| (eq_skill.level as f64) * v)
                        })
                    }).sum();
                
                if cond_skill.is_fixed {
                    // 固定スキルの場合
                    problem = problem.with(skill_expr.eq(cond_skill.level as f64));
                } else {
                    problem = problem.with(skill_expr.ge(cond_skill.level as f64));
                }
            }

            // 【制約式】 スロット
            for s_idx in 0..4 {
                // 武器スキルスロット
                let w_slot_expr: Expression = all_equips.iter().filter_map(|e| {
                    let mut calc = self.slot_calc(e, true, false);
                    if e.kind == crate::model::EquipKind::Deco { for c in &mut calc { *c *= -1; } }
                    equip_vars.get(&e.name).map(|&v| (calc[s_idx] as f64) * v)
                }).sum();
                problem = problem.with(w_slot_expr.ge(0.0));

                // 防具スキルスロット
                let a_slot_expr: Expression = all_equips.iter().filter_map(|e| {
                    let mut calc = self.slot_calc(e, false, true);
                    if e.kind == crate::model::EquipKind::Deco { for c in &mut calc { *c *= -1; } }
                    equip_vars.get(&e.name).map(|&v| (calc[s_idx] as f64) * v)
                }).sum();
                problem = problem.with(a_slot_expr.ge(0.0));

                // 全スキルスロット
                let all_slot_expr: Expression = all_equips.iter().filter_map(|e| {
                    let mut calc = self.slot_calc(e, true, true);
                    if e.kind == crate::model::EquipKind::Deco { for c in &mut calc { *c *= -1; } }
                    equip_vars.get(&e.name).map(|&v| (calc[s_idx] as f64) * v)
                }).sum();
                problem = problem.with(all_slot_expr.ge(0.0));
            }

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
                    return true;
                }
            };

            // 結果から EquipSet を構築
            let mut new_set = EquipSet::new();
            let mut has_data = false;
            for (name, var) in &equip_vars {
                if solution.value(*var) > 0.5 {
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
                return true;
            }
        }
        false
    }
}
