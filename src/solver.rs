use crate::{sub_idx, Board};
use std::collections::HashMap;
mod rules;

pub use solve2 as solve;
pub fn solve1(board: &mut Board) {
    let mut rules = rules::BoardRule::read_rules_from_yaml("assets/rules.yml");
    loop {
        let mut is_done = true;
        rules.retain(|r| {
            let size = r.task.size();
            let bounds = sub_idx(board.size(), size);
            log::trace!("Trying rule:\n{r}");
            let mut retain = false;
            for idx in (0..=bounds.0)
                .into_iter()
                .flat_map(|row| (0..=bounds.1).into_iter().map(move |col| (row, col)))
            {
                if let Some(x) = r.apply_at(board, idx) {
                    retain |= x;
                    is_done &= x;
                }
            }
            retain
        });
        if rules.is_empty() || is_done {
            break;
        }
        log::info!("Rules retained:{}", rules.len());
    }
}
pub fn solve2(board: &mut Board) {
    let rules = rules::BoardRule::read_rules_from_yaml("assets/rules.yml");
    rules.iter().for_each(|r| log::trace!("{r}"));
    let keys: Vec<_> = board.task().indexed_iter().map(|x| x.0).collect();
    let mut hm: HashMap<_, _> = keys
        .iter()
        .map(|&k| (k, (0..rules.len()).collect::<Vec<_>>()))
        .collect();
    loop {
        log::info!("Solving..");
        let mut is_done = true;
        for &k in keys.iter() {
            if let Some(idxs) = hm.get_mut(&k) {
                idxs.retain(|i| {
                    if let Some(x) = rules[*i].apply_at(board, k) {
                        is_done &= x;
                        x
                    } else {
                        false
                    }
                });
                if idxs.is_empty() {
                    hm.remove(&k);
                }
            }
        }
        if is_done {
            break;
        }
        log::trace!("{hm:?}");
    }
}
