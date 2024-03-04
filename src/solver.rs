use crate::{Board, Direction::*};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::{HashMap, HashSet};
use std::iter;

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
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BoardRule {
    task: String,
    fences: String,
    solution: String,
}

fn transpose_string(s: &str, t: usize) -> String {
    format!("{}{}", &s[t..], &s[0..t]).to_string()
}

impl BoardRule {
    fn get_rotations(&self) -> Vec<Self> {
        let mut ret = vec![self.clone(); 4];
        let mut set = HashSet::from([self.fences.clone()]);
        for i in 1..4 {
            let rotated_fence = transpose_string(&self.fences, i);
            if set.insert(rotated_fence.clone()) {
                ret.push(Self {
                    task: self.task.clone(),
                    fences: rotated_fence,
                    solution: transpose_string(&self.solution, i),
                })
            } else {
                break;
            }
        }
        ret
    }
    fn apply(&self, board: &mut Board) {
        let tasks = board.tasks();
        // println!("tasks: {tasks}, {self:?}");
        if let Some(idx) = tasks.find(&self.task) {
            let (row, col) = (idx as u8 / board.width(), idx as u8 % board.width());
            let n = board
                .get_task_neighbors(row, col)
                .map(|x| fence_to_char(x).to_string())
                .join("");

            for r in self.get_rotations() {
                if n == r.fences {
                    println!("{n},{idx} {r:?}");
                    r.solution.chars().enumerate().for_each(|(i, c)| match i {
                        0 => board.set_fence(Horizontal, row, col, char_to_fence(c)),
                        1 => board.set_fence(Horizontal, row + 1, col, char_to_fence(c)),
                        2 => board.set_fence(Vertical, row, col, char_to_fence(c)),
                        3 => board.set_fence(Vertical, row, col + 1, char_to_fence(c)),
                        _ => unreachable!(),
                    })
                }
            }
        }
    }
}

pub fn solve(board: &mut Board) {
    let f = std::fs::File::open("assets/rules.yml").expect("Couldn't open file");
    let rules: Vec<BoardRule> = serde_yaml::from_reader(f).expect("Couldn't obtain rules");
    for rule in &rules {
        rule.apply(board)
    }
    println!("{rules:?}");
}
