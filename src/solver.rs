use crate::{
    add_idx,
    board::{print_board, Fence, Fences, Task, U2},
    sub_idx, Board,
};
use grid::Grid;
use serde::Deserialize;
use serde_yaml;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

#[derive(Debug, Clone)]
pub struct BoardRule {
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

        let cols = task.lines().last().unwrap().chars().count();
        let task: Task =
            Grid::from_vec(task.replace('\n', "").chars().map(U2::from).collect(), cols);
        let size = task.size();
        let boundary = (size.0 + 1) * size.1;
        let fences: Vec<Fence> = fences.replace('_', "").chars().map(Fence::from).collect();
        let fences: Fences = [
            Grid::from_vec(fences[0..boundary].to_vec(), task.cols()),
            Grid::from_vec(fences[boundary..].to_vec(), task.cols() + 1),
        ];
        let solution: Vec<Fence> = solution.replace('_', "").chars().map(Fence::from).collect();
        let solution: Fences = [
            Grid::from_vec(solution[0..boundary].to_vec(), task.cols()),
            Grid::from_vec(solution[boundary..].to_vec(), task.cols() + 1),
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
            task.iter().map(|x| char::from(x.clone())).collect(),
            fences[0].iter().map(|&x| char::from(x)).collect(),
            fences[1].iter().map(|&x| char::from(x)).collect(),
            solution[0].iter().map(|&x| char::from(x)).collect(),
            solution[1].iter().map(|&x| char::from(x)).collect(),
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
            let mut rot = ret.last().unwrap().clone();
            rot.task.rotate_right();
            rot.fences[1].rotate_right();
            rot.fences[0].rotate_right();
            rot.fences.rotate_right(1);
            rot.solution[1].rotate_right();
            rot.solution[0].rotate_right();
            rot.solution.rotate_right(1);
            if let Some(v) = rot.corner.as_mut() {
                *v += 1;
            }
            if set.insert(rot.to_hash()) {
                ret.push(rot)
            } else {
                break;
            }
        }
        ret
    }
    pub fn apply_at(&self, board: &mut Board, idx: (usize, usize)) -> Option<bool> {
        let size = self.task.size();
        let bounds = sub_idx(board.size(), size);
        let origin = (0, 0);
        let (row, col) = idx;
        if row > bounds.0
            || col > bounds.1
            || self.corner.map_or(false, |x| {
                [origin, (0, bounds.1), bounds, (bounds.0, 0)][x] != idx
            })
        {
            return None;
        }
        let task_match = self
            .task
            .indexed_iter()
            .filter(|x| x.1.is_some())
            .all(|(i, x)| *x == board.task()[add_idx(i, idx)])
            && [0usize, 1].iter().any(|&dir| {
                self.solution[dir]
                    .indexed_iter()
                    .filter_map(|x| x.1.map(|_| x.0))
                    .any(|i| board.fences()[dir][add_idx(i, idx)].is_none())
            });

        if task_match
            && [0usize, 1].iter().all(|&dir| {
                self.fences[dir]
                    .indexed_iter()
                    .filter(|x| x.1.is_some())
                    .all(|(i, x)| *x == board.fences()[dir][add_idx(i, idx)])
            })
        {
            log::trace!(
                "match at idx: {idx:?} size: {size:?} bounds: {bounds:?} {:?}",
                self.task
                    .indexed_iter()
                    .map(|(i, _)| board.task()[add_idx(i, idx)].clone())
                    .collect::<Vec<_>>()
            );
            for dir in [0, 1] {
                self.solution[dir]
                    .indexed_iter()
                    .filter(|x| x.1.is_some())
                    .for_each(|(i, x)| board.fences_mut()[dir][add_idx(i, idx)] = *x)
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
        if let Some(x) = self.corner {
            write!(f, "corner: {x}\n")?;
        }
        let from = print_board(&self.task, &self.fences)
            .lines()
            .zip(print_board(&self.task, &self.solution).lines())
            .map(|(x, y)| format!("{x} ║ {y}\n"))
            .fold(String::new(), |a, b| a + &b);
        let from = from.trim_end();
        write!(f, "{from}\n")?;
        Ok(())
    }
}

pub use solve2 as solve;
pub fn solve1(board: &mut Board) {
    let mut rules = BoardRule::read_rules_from_yaml("assets/rules.yml");
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
        log::info!("Rules retained:{}", rules.len());
        if rules.is_empty() || is_done {
            break;
        }
    }
}
pub fn solve2(board: &mut Board) {
    let rules = BoardRule::read_rules_from_yaml("assets/rules.yml");
    let keys: Vec<_> = board.task().indexed_iter().map(|x| x.0).collect();
    let mut hm: HashMap<_, _> = keys
        .iter()
        .map(|&k| (k, (0..rules.len()).collect::<Vec<_>>()))
        .collect();
    loop {
        log::info!("Solving..");
        let mut board_not_changed = true;
        for &k in keys.iter() {
            if let Some(idxs) = hm.get_mut(&k) {
                idxs.retain(|i| {
                    if let Some(x) = rules[*i].apply_at(board, k) {
                        board_not_changed &= x;
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
        if board_not_changed {
            break;
        }
        log::trace!("{hm:?}");
    }
}
