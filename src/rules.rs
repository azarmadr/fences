use crate::{
    add_idx,
    board::{print_board, Fences, Tasks},
    sub_idx, Fence,
};
use grid::Grid;
use serde::Deserialize;
use serde_yaml;
use std::collections::HashSet;

use super::FencesSolver;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TaskType {
    Corner(usize),
    Edge(usize),
    None,
}
impl TaskType {
    pub fn new(corner: bool, edge: bool) -> Self {
        if corner && edge {
            unreachable!(
                r"rule can be either `corner` or `edge`, or neither.
                         Please use any one of them or none of them"
            )
        }
        if corner {
            TaskType::Corner(0)
        } else if edge {
            TaskType::Edge(0)
        } else {
            TaskType::None
        }
    }
    pub fn rotate(&mut self) {
        if let TaskType::Corner(x) | TaskType::Edge(x) = self {
            *x = (*x + 1) % 4
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoardRule {
    pub task: Tasks,
    pub variant: TaskType,
    pub fences: Fences,
    pub solution: Fences,
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
            #[serde(default)]
            edge: bool,
        }

        let helper = Helper::deserialize(deserializer)?;
        let Helper {
            task,
            fences,
            solution,
            corner,
            edge,
        } = helper;

        let cols = task.lines().last().unwrap().chars().count();
        let task: Tasks = Grid::from_vec(
            task.replace('\n', "")
                .chars()
                .map(|x| x.to_string().parse().ok())
                .collect(),
            cols,
        );
        let size = task.size();
        let boundary = (size.0 + 1) * size.1;
        let fences: Vec<Fence> = fences.chars().filter_map(|c| c.try_into().ok()).collect();
        let fences: Fences = [
            Grid::from_vec(fences[0..boundary].to_vec(), task.cols()),
            Grid::from_vec(fences[boundary..].to_vec(), task.cols() + 1),
        ];
        let solution: Vec<Fence> = solution.chars().filter_map(|c| c.try_into().ok()).collect();
        let solution: Fences = [
            Grid::from_vec(solution[0..boundary].to_vec(), task.cols()),
            Grid::from_vec(solution[boundary..].to_vec(), task.cols() + 1),
        ];
        Ok(Self {
            task,
            fences,
            solution,
            variant: TaskType::new(corner, edge),
        })
    }
}

impl BoardRule {
    pub(crate) fn to_hash(&self) -> String {
        let Self {
            task,
            fences,
            solution,
            variant,
        } = self;
        [
            task.iter().map(|x| x.map_or(' ', char::from)).collect(),
            fences[0].iter().map(|&x| char::from(x)).collect(),
            fences[1].iter().map(|&x| char::from(x)).collect(),
            solution[0].iter().map(|&x| char::from(x)).collect(),
            solution[1].iter().map(|&x| char::from(x)).collect(),
            format!("{variant:?}"),
        ]
        .join("|")
    }
    pub(crate) fn get_rotations(&self) -> Vec<Self> {
        let Self {
            task,
            fences,
            solution,
            variant,
        } = self;
        let mut ret: Vec<Self> = vec![Self {
            task: task.clone(),
            fences: [fences[0].clone(), fences[1].clone()],
            solution: [solution[0].clone(), solution[1].clone()],
            variant: variant.clone(),
        }];
        let mut set = HashSet::from([self.to_hash()]);
        for _ in 1..4 {
            let mut rot = ret.last().unwrap().clone();
            rot.task.rotate_right();
            rot.fences[1].rotate_right();
            rot.fences[0].rotate_right();
            rot.fences.rotate_right(1);
            rot.solution[1].rotate_right();
            rot.solution[0].rotate_right();
            rot.solution.rotate_right(1);
            match &mut rot.variant {
                TaskType::Corner(v) => *v += 1,
                TaskType::Edge(v) => *v += 1,
                _ => (),
            };
            if set.insert(rot.to_hash()) {
                ret.push(rot)
            } else {
                break;
            }
        }
        ret
    }
    pub fn apply_at(&self, board: &mut impl FencesSolver, idx: (usize, usize)) -> Option<bool> {
        let size = self.task.size();
        if board.cols() < size.1 || board.rows() < size.0 {
            return None;
        }
        let bounds = sub_idx(board.size(), size);
        if idx.0 > bounds.0
            || idx.1 > bounds.1
            || match self.variant {
                TaskType::Corner(x) => [(0, 0), (0, bounds.1), bounds, (bounds.0, 0)][x] != idx,
                TaskType::Edge(x) => {
                    [idx.0 != 0, idx.1 != bounds.1, idx.0 != bounds.0, idx.1 != 0][x]
                }
                _ => false,
            }
        {
            return None;
        }
        let task_match = self
            .task
            .indexed_iter()
            .filter(|x| x.1.is_some())
            .all(|(i, x)| x == board.task(add_idx(i, idx)))
            && [0usize, 1].iter().any(|&dir| {
                self.solution[dir]
                    .indexed_iter()
                    .filter_map(|x| x.1.map(|_| x.0))
                    .any(|i| board.edge(dir, add_idx(i, idx)).is_none())
            });
        if !task_match {
            return None;
        }

        if task_match
            && [0usize, 1].iter().all(|&dir| {
                self.fences[dir]
                    .indexed_iter()
                    .filter(|x| x.1.is_some())
                    .all(|(i, x)| x == board.edge(dir, add_idx(i, idx)))
            })
        {
            log::trace!(
                "match at idx: {idx:?} size: {size:?} bounds: {bounds:?} {:?}",
                self.task
                    .indexed_iter()
                    .map(|(i, _)| board.task(add_idx(i, idx)).clone())
                    .collect::<Vec<_>>()
            );
            for dir in [0, 1] {
                self.solution[dir]
                    .indexed_iter()
                    .filter_map(|x| x.1.map(|v| (x.0, v)))
                    .for_each(|(i, x)| board.play(dir, add_idx(i, idx), x, format!("{self}")))
            }
            Some(false)
        } else {
            Some(true)
        }
    }
    pub fn read_rules_from_yaml(file: &str) -> Vec<Self> {
        let f = std::fs::File::open(file).expect("Couldn't open file");
        let rules: Vec<BoardRule> = serde_yaml::from_reader(f).expect("Couldn't obtain rules");
        rules.iter().flat_map(|x| x.get_rotations()).collect()
    }
}

impl core::fmt::Display for BoardRule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let TaskType::Corner(x) = self.variant {
            write!(f, "corner: {x}\n")?;
        } else if let TaskType::Edge(x) = self.variant {
            write!(f, "edge: {x}\n")?;
        }
        let from = print_board(&self.task, &self.fences, false)
            .lines()
            .zip(print_board(&self.task, &self.solution, false).lines())
            .fold(String::new(), |a, (b, c)| format!("{a}{b} ║ {c}\n"));
        write!(f, "{from}")?;
        Ok(())
    }
}
