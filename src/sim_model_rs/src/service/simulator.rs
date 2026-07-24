use crate::domain::Searcher;
use crate::model::{EquipSet, SearchCondition, Skill};
use rayon::prelude::*;

pub struct Simulator {
    pub is_searched_all: bool,
    pub is_canceling: bool,
    // Note: Rustでは各スレッドが独立したSearcherインスタンスを持つように設計する
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            is_searched_all: false,
            is_canceling: false,
        }
    }

    pub fn search(&mut self, condition: &SearchCondition, limit: usize) -> Vec<EquipSet> {
        self.is_canceling = false;
        let mut searcher = Searcher::new(condition.clone());
        self.is_searched_all = searcher.exec_search(limit);
        
        searcher.result_sets
    }

    pub fn cancel(&mut self) {
        self.is_canceling = true;
    }

    // rayon を使った並列追加スキル検索のモック実装
    pub fn search_extra_skill(&self, condition: &SearchCondition) -> Vec<Skill> {
        let skills = crate::data::MASTERS.read().unwrap().skills.clone();
        
        let ex_skills: Vec<Skill> = skills.into_par_iter().flat_map(|skill| {
            if self.is_canceling {
                return vec![];
            }
            
            let mut sub_result = Vec::new();
            for i in 1..=skill.level {
                let mut ex_condition = condition.clone();
                let ex_skill = Skill::new(skill.name.clone(), i, Some(skill.category.clone()), false, skill.can_with_artian, skill.specific_names.clone());
                
                let is_new_skill = ex_condition.add_skill(ex_skill.clone());
                if is_new_skill {
                    let mut ex_searcher = Searcher::new(ex_condition);
                    ex_searcher.exec_search(1);
                    if !ex_searcher.result_sets.is_empty() {
                        sub_result.push(ex_skill);
                    }
                }
            }
            sub_result
        }).collect();

        // 実際にはソート処理等が入る
        ex_skills
    }
}
