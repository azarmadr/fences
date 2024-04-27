use crate::{
    board::{are_linked, get_paths},
    sub_idx, Board,
};
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
    rules.iter().for_each(|r| log::trace!("\n{r}"));
    let keys: Vec<_> = board.task().indexed_iter().map(|x| x.0).collect();
    let mut hm: HashMap<_, _> = keys
        .iter()
        .map(|&k| (k, (0..rules.len()).collect::<Vec<_>>()))
        .collect();
    loop {
        log::trace!("Solving..");
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

        let paths = get_paths(board.fences());
        if paths.len() > 1 {
            paths.iter().for_each(|p| {
                if p.len() < 3 {
                    return;
                }
                let (f, l) = (p[0], p.last().unwrap());
                if f.0 == l.0 {
                    if if f.0 == 0 {
                        f.2 == l.2 && f.1.abs_diff(l.1) == 1
                    } else {
                        f.1 == l.1 && f.2.abs_diff(l.2) == 1
                    } {
                        let possible_edges = if f.0 == 0 {
                            [(1, f.1.min(l.1), f.2), (1, f.1.min(l.1), f.2 + 1)]
                        } else {
                            [(0, f.1, f.2.min(l.2)), (0, f.1 + 1, f.2.min(l.2))]
                        };
                        possible_edges.iter().for_each(|e| {
                            if board.edge(e).is_none() {
                                board.play(e.0, (e.1, e.2), false, "open closed box")
                            }
                        })
                    }
                } else {
                    let mut a = [f, *l];
                    a.sort();
                    log::trace!("Link Edges: {a:?}");

                    for x in a[0].0..=a[1].0 {
                        for y in a[0].1..=a[1].1 {
                            for z in a[0].2..=a[1].2 {
                                if !a.contains(&(x, y, z))
                                    && a.iter().all(|a| are_linked(a, &(x, y, z)))
                                {
                                    board.play(x, (y, z), false, "open closed box")
                                }
                            }
                        }
                    }
                }
            })
        }

        if is_done {
            break;
        }
        log::trace!("{hm:?}");
    }
}
