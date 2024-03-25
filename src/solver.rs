use crate::Board;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{collections::HashSet, usize};

fn rotate_string(input: &str, row: usize, col: usize) -> String {
    let mut output = String::with_capacity(input.len());
    for c in 0..col {
        for r in (0..row).rev() {
            output.push(input.as_bytes()[r * col + c] as char)
        }
    }
    output
}
pub fn rotate_fence(f: &str, row: usize, col: usize) -> String {
    format!(
        "{}{}",
        rotate_string(
            &f[col * (row + 1)..(2 * row * col + row + col)],
            row,
            col + 1,
        ),
        rotate_string(&f[0..col * (row + 1)], row + 1, col),
    )
}
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
struct BoardRule {
    task: String,
    fences: String,
    solution: String,
    #[serde(default)]
    corner: bool,
    #[serde(default)]
    rotation: usize,
}

impl BoardRule {
    fn _rotate(&self, row: usize, col: usize) -> Self {
        Self {
            task: rotate_string(&self.task.replace("\n", ""), row, col),
            fences: rotate_fence(&self.fences, row, col),
            solution: rotate_fence(&self.solution, row, col),
            rotation: self.rotation + if self.corner { 1 } else { 0 },
            corner: self.corner,
        }
    }
    fn _get_rotations(&self) -> Vec<Self> {
        let (row, col) = (
            self.task.lines().count(),
            self.task.lines().last().unwrap().len(),
        );
        let mut ret = vec![self.clone()];
        let mut set = HashSet::from([self.clone()]);
        for _ in 1..4 {
            let rot = ret.last().unwrap()._rotate(row, col);
            if set.insert(rot.clone()) {
                ret.push(rot)
            } else {
                break;
            }
        }
        ret
    }
    fn apply(&self, board: &mut Board) -> bool {
        let rule_board = Board::from_task_lines(&self.task, Some(&self.fences.replace('_', "")));
        let size = (rule_board.height(), rule_board.width());
        let bounds = (board.height() - size.0, board.width() - size.1);
        println!("Trying rule:\n{rule_board}");
        let mut retain = false;
        for row in 0..=bounds.1 {
            for col in 0..=bounds.0 {
                let idx = (row, col);
                if self.corner
                    && [(0, 0), (0, bounds.1), (bounds.0, 0), bounds][self.rotation] != idx
                {
                    continue;
                }
                let task_match = board
                    .task()
                    .subgrid_iter(idx, size)
                    .zip(rule_board.task().subgrid_iter((0, 0), size))
                    .all(|(a, b)| b.is_none() || a == b);
                retain |= task_match;
                if task_match
                    && board.fences()[0]
                        .subgrid_iter(idx, (size.0 + 1, size.1))
                        .zip(rule_board.fences()[0].subgrid_iter((0, 0), (size.0 + 1, size.1)))
                        .all(|(a, b)| b.is_none() || a == b)
                    && board.fences()[1]
                        .subgrid_iter(idx, (size.0, size.1 + 1))
                        .zip(rule_board.fences()[1].subgrid_iter((0, 0), (size.0, size.1 + 1)))
                        .all(|(a, b)| b.is_none() || a == b)
                {
                    let mut solution =
                        Board::from_task_lines(&self.task, Some(&self.solution.replace('_', "")));
                    println!(
                        "{:?} {idx:?} {size:?} {bounds:?}",
                        board.task().subgrid_iter(idx, size).collect::<Vec<_>>()
                    );
                    retain &= false;
                    println!("match at ({row},{col})");
                    board.fences_mut()[0]
                        .subgrid_iter_mut(idx, (size.0 + 1, size.1))
                        .zip(
                            solution.fences_mut()[0].subgrid_iter_mut((0, 0), (size.0 + 1, size.1)),
                        )
                        .for_each(|(a, b)| {
                            if !b.is_none() {
                                *a = *b
                            }
                        });
                    board.fences_mut()[1]
                        .subgrid_iter_mut(idx, (size.0, size.1 + 1))
                        .zip(
                            solution.fences_mut()[1].subgrid_iter_mut((0, 0), (size.0, size.1 + 1)),
                        )
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
        // rules.iter().flat_map(|x| x.get_rotations()).collect()
        rules
    }
}

pub fn solve(board: &mut Board) {
    let mut rules = BoardRule::read_rules_from_yaml("assets/rules.yml");

    for _ in 0..3 {
        println!("Rules retained:{}", rules.len());
        rules.retain(|x| x.apply(board));
    }
}
