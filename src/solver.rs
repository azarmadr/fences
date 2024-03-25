use crate::{
    add_idx,
    board::{Fences, Task},
    grid::Grid,
    sub_idx, Board,
};
use serde::Deserialize;
use serde_yaml;
use std::{collections::HashSet, usize};

#[derive(Debug)]
struct BoardRule {
    task: Task,
    fences: Fences,
    solution: Fences,
    corner: Option<usize>,
}

impl<'de> Deserialize<'de> for BoardRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            task: String,
            fences: String,
            solution: String,
            #[serde(default)]
            corner: bool,
        }

        let Helper {
            task,
            fences,
            solution,
            corner,
        } = Helper::deserialize(deserializer)?;

        let task = Grid::from_lines(&task);
        let size = task.size();
        let boundary = (size.0 + 1) * size.1;
        let fences = fences.replace('_', "");
        let fences = [
            Grid::from_string(size.1, &fences[0..boundary]),
            Grid::from_string(size.1 + 1, &fences[boundary..]),
        ];
        let solution = solution.replace('_', "");
        let solution = [
            Grid::from_string(size.1, &solution[0..boundary]),
            Grid::from_string(size.1 + 1, &solution[boundary..]),
        ];
        let corner = if corner { Some(0) } else { None };
        Ok(Self {
            task,
            fences,
            solution,
            corner,
        })
    }
}

impl BoardRule {
    fn to_hash(&self) -> String {
        let Self {
            task,
            fences,
            solution,
            corner,
        } = self;
        [
            task.to_string(),
            fences[0].to_string(),
            fences[1].to_string(),
            solution[0].to_string(),
            solution[1].to_string(),
            corner.map_or("".to_string(), |x| x.to_string()),
        ]
        .join("|")
    }
    fn get_rotations(&self) -> Vec<Self> {
        let Self {
            task,
            fences,
            solution,
            corner,
        } = self;
        let mut ret: Vec<Self> = vec![Self {
            task: task.clone(),
            fences: [fences[0].clone(), fences[1].clone()],
            solution: [solution[0].clone(), solution[1].clone()],
            corner: *corner,
        }];
        let mut set = HashSet::from([self.to_hash()]);
        for _ in 1..4 {
            let rot = Self {
                task: ret.last().unwrap().task.rotate(),
                fences: [
                    ret.last().unwrap().fences[1].rotate(),
                    ret.last().unwrap().fences[0].rotate(),
                ],
                solution: [
                    ret.last().unwrap().solution[1].rotate(),
                    ret.last().unwrap().solution[0].rotate(),
                ],
                corner: ret.last().unwrap().corner.map(|x| x + 1),
            };
            if set.insert(rot.to_hash()) {
                ret.push(rot)
            } else {
                break;
            }
        }
        ret
    }
    fn apply(&self, board: &mut Board) -> bool {
        let size = self.task.size();
        let bounds = sub_idx(board.size(), size);
        let origin = (0, 0);
        let f0_size = add_idx(size, (1, 0));
        let f1_size = add_idx(size, (0, 1));
        println!("Trying rule:");
        Board::print(&self.task, &self.fences);
        let mut retain = false;
        for row in 0..=bounds.0 {
            for col in 0..=bounds.1 {
                let idx = (row, col);
                if self.corner.map_or(false, |x| {
                    [origin, (0, bounds.1), bounds, (bounds.0, 0)][x] != idx
                }) {
                    continue;
                }
                let task_match = self
                    .task
                    .subgrid_iter(origin, size)
                    .zip(board.task().subgrid_iter(idx, size))
                    .all(|(a, b)| a.is_none() || a == b);
                retain |= task_match;
                if task_match
                    && board.fences()[0]
                        .subgrid_iter(idx, f0_size)
                        .zip(self.fences[0].subgrid_iter(origin, f0_size))
                        .all(|(a, b)| b.is_none() || a == b)
                    && board.fences()[1]
                        .subgrid_iter(idx, f1_size)
                        .zip(self.fences[1].subgrid_iter(origin, f1_size))
                        .all(|(a, b)| b.is_none() || a == b)
                {
                    println!(
                        "match at idx: {idx:?} size: {size:?} bounds: {bounds:?} {:?}",
                        board.task().subgrid_iter(idx, size).collect::<Vec<_>>()
                    );
                    retain &= false;
                    board.fences_mut()[0]
                        .subgrid_iter_mut(idx, f0_size)
                        .zip(self.solution[0].clone().subgrid_iter_mut(origin, f0_size))
                        .for_each(|(a, b)| {
                            if !b.is_none() {
                                *a = *b
                            }
                        });
                    board.fences_mut()[1]
                        .subgrid_iter_mut(idx, f1_size)
                        .zip(self.solution[1].clone().subgrid_iter_mut(origin, f1_size))
                        .for_each(|(a, b)| {
                            if !b.is_none() {
                                *a = *b
                            }
                        });
                }
            }
        }
        retain
    }
    fn read_rules_from_yaml(file: &str) -> Vec<Self> {
        let f = std::fs::File::open(file).expect("Couldn't open file");
        let rules: Vec<BoardRule> = serde_yaml::from_reader(f).expect("Couldn't obtain rules");
        rules.iter().flat_map(|x| x.get_rotations()).collect()
        // rules
    }
}
impl core::fmt::Display for BoardRule {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Board::print(&self.task, &self.fences);
        Board::print(&self.task, &self.solution);
        if let Some(x) = self.corner {
            println!("corner: {x}");
        }
        Ok(())
    }
}

pub fn solve(board: &mut Board) {
    let mut rules = BoardRule::read_rules_from_yaml("assets/rules.yml");
    for rule in &rules {
        print!("rule:\n{rule}");
    }

    for _ in 0..3 {
        println!("Rules retained:{}", rules.len());
        rules.retain(|x| x.apply(board));
    }
}
