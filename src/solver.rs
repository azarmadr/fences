use crate::{Board, Direction::*};
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
fn char_to_fence(c: char) -> Option<bool> {
    match c {
        'y' => Some(true),
        'n' => Some(false),
        'u' => None,
        _ => unreachable!(),
    }
}
fn fence_to_char(fence: Option<bool>) -> char {
    match fence {
        Some(true) => 'y',
        Some(false) => 'n',
        None => 'u',
    }
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
    fn rotate(&self, row: usize, col: usize) -> Self {
        Self {
            task: rotate_string(&self.task.replace("\n", ""), row, col),
            fences: rotate_fence(&self.fences, row, col),
            solution: rotate_fence(&self.solution, row, col),
            rotation: self.rotation + if self.corner { 1 } else { 0 },
            corner: self.corner,
        }
    }
    fn get_rotations(&self) -> Vec<Self> {
        let (row, col) = (
            self.task.lines().count(),
            self.task.lines().last().unwrap().len(),
        );
        let mut ret = vec![self.clone()];
        let mut set = HashSet::from([self.clone()]);
        for _ in 1..4 {
            let rot = ret.last().unwrap().rotate(row, col);
            if set.insert(rot.clone()) {
                ret.push(rot)
            } else {
                break;
            }
        }
        ret
    }
    fn apply(&self, board: &mut Board) {
        let tasks = board.tasks();
        let (width, height) = (board.width(), board.height());
        let rows: Vec<&str> = tasks.split_terminator('\n').collect();
        for row in rows.iter().enumerate() {
            let matches = row.1.match_indices(&self.task).collect::<Vec<_>>();
            if matches.len() > 0 {
                println!("found {:?} in {row:?} at {matches:?}", self.task);
            }
        }
        tasks
            .chars()
            .filter(|&c| c != '\n')
            .enumerate()
            .filter(|(idx, c)| {
                (!self.corner
                    || &[0, width - 1, width * height - 1, width * (height - 1)][self.rotation]
                        == &(*idx as u8))
                    && self.task == c.to_string()
            })
            .for_each(|(idx, _c)| {
                let (row, col) = (idx as u8 / width, idx as u8 % width);
                let n = board
                    .get_task_neighbors(idx as u8)
                    .map(|x| fence_to_char(x).to_string())
                    .join("");

                if n == self.fences {
                    self.solution
                        .chars()
                        .enumerate()
                        .for_each(|(i, c)| match i {
                            0 => board.set_fence(Horizontal, row, col, char_to_fence(c)),
                            1 => board.set_fence(Horizontal, row + 1, col, char_to_fence(c)),
                            2 => board.set_fence(Vertical, row, col, char_to_fence(c)),
                            3 => board.set_fence(Vertical, row, col + 1, char_to_fence(c)),
                            _ => unreachable!(),
                        })
                }
            })
    }
    fn read_rules_from_yaml(file: &str) -> Vec<Self> {
        let f = std::fs::File::open(file).expect("Couldn't open file");
        let rules: Vec<BoardRule> = serde_yaml::from_reader(f).expect("Couldn't obtain rules");
        rules.iter().flat_map(|x| x.get_rotations()).collect()
    }
}

pub fn solve(board: &mut Board) {
    let rules = BoardRule::read_rules_from_yaml("assets/rules.yml");
    for rule in dbg!(&rules) {
        rule.apply(board)
    }
}
