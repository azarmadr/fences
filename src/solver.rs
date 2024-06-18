use crate::{board::*, geom::BoardGeom, *};
use grid::Grid;
use rules::TaskType;
use serde::Deserialize;
use serde_yaml;
use std::collections::{HashMap, HashSet, VecDeque};

pub type Rules = HashSet<Rule>;
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Rule {
    in_h: Grid<Fence>,
    in_v: Grid<Fence>,
    out_h: Grid<Fence>,
    out_v: Grid<Fence>,
    location: TaskType,
}
impl Rule {
    fn new(
        size: (usize, usize),
        fences: Vec<Fence>,
        solution: Vec<Fence>,
        location: TaskType,
    ) -> Self {
        let boundary = (size.0 + 1) * size.1;
        Self {
            in_h: Grid::from_vec(fences[0..boundary].to_vec(), size.1),
            in_v: Grid::from_vec(fences[boundary..].to_vec(), size.1 + 1),
            out_h: Grid::from_vec(solution[0..boundary].to_vec(), size.1),
            out_v: Grid::from_vec(solution[boundary..].to_vec(), size.1 + 1),
            location,
        }
    }
    fn rotate(&mut self) {
        // println!("<{self:?}");
        std::mem::swap(&mut self.in_h, &mut self.in_v);
        std::mem::swap(&mut self.out_h, &mut self.out_v);
        self.in_h.rotate_right();
        self.in_v.rotate_right();
        self.out_h.rotate_right();
        self.out_v.rotate_right();

        self.location.rotate();
        // println!(">{self:?}");
    }
    fn reorder(&mut self) {
        let cols = self.in_h.cols();
        self.in_h = Grid::from_vec(self.in_h.clone().into_vec(), cols);
        self.out_h = Grid::from_vec(self.out_h.clone().into_vec(), cols);
        self.in_v = Grid::from_vec(self.in_v.clone().into_vec(), cols + 1);
        self.out_v = Grid::from_vec(self.out_v.clone().into_vec(), cols + 1);
    }
    pub fn print(&self) -> String {
        let f = |x: &Grid<Fence>| -> String {
            x.iter_rows()
                .map(|r| {
                    r.map(|&c| char::from(c).to_string())
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("|")
        };
        format!(
            "Rule {:?}
  in_h: {} out_h: {}
  in_v: {} out_v: {}",
            self.location,
            f(&self.in_h),
            f(&self.out_h),
            f(&self.in_v),
            f(&self.out_v)
        )
    }
}

#[derive(Debug)]
pub struct BoardRules(pub HashMap<Grid<Task>, Rules>);
impl BoardRules {
    pub fn new(file: &str) -> Self {
        let f = std::fs::File::open(file).expect("Couldn't open file");
        serde_yaml::from_reader(f).expect("Couldn't obtain rules")
    }
    fn add_rule(
        &mut self,
        clues: &Tasks,
        fences: Vec<Fence>,
        solution: Vec<Fence>,
        task_type: TaskType,
    ) {
        let clues = &mut clues.clone();
        let rule = &mut Rule::new(clues.size(), fences, solution, task_type);

        self.0.entry(clues.clone()).or_insert([].into());
        let rules = self.0.get_mut(clues).unwrap();
        rules.insert(rule.clone());

        for i in 0..3 {
            clues.rotate_right();
            rule.rotate();
            let mut clues = clues.clone();
            let mut rule = rule.clone();
            if i % 2 == 0 {
                clues = Grid::from_vec(clues.clone().into_vec(), clues.cols());
                rule.reorder();
            }
            self.0.entry(clues.clone()).or_insert([].into());
            let rules = self.0.get_mut(&clues).unwrap();
            rules.insert(rule.clone());
        }
    }
}

impl<'de> Deserialize<'de> for BoardRules {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct Rules {
            fences: String,
            solution: String,
            #[serde(default)]
            corner: bool,
            #[serde(default)]
            edge: bool,
            #[serde(default)]
            mirror: bool,
        }

        #[derive(Debug, Deserialize)]
        struct Helper(HashMap<String, Vec<Rules>>);

        let helper = Helper::deserialize(deserializer)?;
        println!("{helper:?}");
        let mut ret = BoardRules(HashMap::from([]));
        for (k, v) in helper.0 {
            let tasks: Tasks = k
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| c.to_string().parse().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
                .into();

            v.iter().for_each(|r| {
                let fences: Vec<Fence> =
                    r.fences.chars().filter_map(|x| x.try_into().ok()).collect();
                let solution: Vec<Fence> = r
                    .solution
                    .chars()
                    .filter_map(|x| x.try_into().ok())
                    .collect();
                if r.mirror {
                    ret.add_rule(&tasks, solution.clone(), fences.clone(), TaskType::new(r.corner, r.edge));
                }
                ret.add_rule(&tasks, fences, solution, TaskType::new(r.corner, r.edge));
            });
        }
        Ok(ret)
    }
}

pub fn solve(board: &mut impl FencesSolver) {
    let rules = rules::BoardRule::read_rules_from_yaml("assets/rules.yml");
    rules.iter().for_each(|r| log::trace!("\n{r}"));
    let keys: Vec<_> = board.tasks_iter().map(|x| x.0).collect();
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

        is_done &= !block_closed_paths(board);
        if is_done {
            break;
        }
        log::trace!("{hm:?}");
    }
}

pub fn block_closed_paths(board: &mut impl FencesSolver) -> bool {
    let mut changed = false;
    let paths = board.paths();
    if paths.len() > 1 {
        paths.iter().for_each(|p| {
            if p.len() < 3 {
                return;
            }
            let (f, l) = (p[0], p.last().unwrap());
            if f.0 == l.0 {
                let diff = (f.1.abs_diff(l.1), f.2.abs_diff(l.2));
                if if f.0 == 0 {
                    matches!(diff, (1, 0) | (0, 2))
                } else {
                    matches!(diff, (0, 1) | (2, 0))
                } {
                    let min = if f.0 == 0 { f.1.min(l.1) } else { f.2.min(l.2) };
                    let possible_edges = if f.0 == 0 {
                        [(0, min, (f.2 + l.2) / 2), (1, min, f.2), (1, min, f.2 + 1)]
                    } else {
                        [(1, (f.1 + l.1) / 2, min), (0, f.1, min), (0, f.1 + 1, min)]
                    };
                    possible_edges.iter().for_each(|e| {
                        if board.edge(e.0, (e.1, e.2)).is_none() {
                            board.play(e.0, (e.1, e.2), false, "open closed box".to_string());
                            changed = true
                        }
                    })
                }
            } else {
                let a = sorted_tuples(f, *l);
                log::trace!("Link Edges: {a:?}");

                for x in a.0 .0..=a.1 .0 {
                    for y in a.0 .1..=a.1 .1 {
                        for z in a.0 .2..=a.1 .2 {
                            let c = (x, y, z);
                            if !p.contains(&c)
                                && are_linked(&f, &c)
                                && are_linked(l, &c)
                                && board.edge(c.0, (c.1, c.2)).is_none()
                            {
                                log::info!("{c:?}");
                                board.play(x, (y, z), false, "open closed box".to_string());
                                changed = true
                            }
                        }
                    }
                }
            }
        })
    }
    changed
}

fn sorted_tuples(a: Edge, b: Edge) -> (Edge, Edge) {
    let mut res = (a.clone(), b.clone());
    if a.0 > b.0 {
        std::mem::swap(&mut res.0 .0, &mut res.1 .0)
    };
    if a.1 > b.1 {
        std::mem::swap(&mut res.0 .1, &mut res.1 .1)
    };
    if a.2 > b.2 {
        std::mem::swap(&mut res.0 .2, &mut res.1 .2)
    };
    res
}

pub type Edge = (usize, usize, usize);
pub type Idx = (usize, usize);
pub trait FencesSolver: BoardGeom {
    fn set_solution(&mut self, solution: &str);
    fn fences_iter(&self) -> impl Iterator<Item = (Edge, &Fence)>;
    fn tasks_iter(&self) -> impl Iterator<Item = (Idx, &Task)>;
    fn task(&self, idx: Idx) -> &Task;
    fn edge(&self, dir: usize, idx: Idx) -> &Fence;
    fn play(&mut self, dir: usize, idx: Idx, val: bool, id: String);
    fn paths(&self) -> Vec<Vec<Edge>> {
        let mut dashes: Vec<_> = self
            .fences_iter()
            .filter_map(|(e, v)| if v.is_some_and(|x| x) { Some(e) } else { None })
            .collect();
        #[cfg(test)]
        println!("Dashes:{dashes:?}",);
        let mut ret = vec![];
        let mut row = VecDeque::new();
        while !dashes.is_empty() {
            if row.is_empty() {
                row.push_back(dashes.pop().unwrap());
            }
            let mut row_changed = false;
            if let Some(index) = dashes
                .iter()
                .position(|l| are_linked(l, row.front().unwrap()))
            {
                row.push_front(dashes.swap_remove(index));
                row_changed = true;
            }
            if let Some(index) = dashes
                .iter()
                .position(|l| are_linked(l, row.back().unwrap()))
            {
                row.push_back(dashes.swap_remove(index));
                row_changed = true;
            }
            if !row_changed || dashes.is_empty() {
                row.make_contiguous();
                ret.push(row.as_slices().0.to_vec());
                row.clear();
            }
            #[cfg(test)]
            println!("Row: {row:?}\nDashes: {dashes:?}",);
        }
        #[cfg(test)]
        println!(
            "Ret:\n{}",
            ret.iter()
                .map(|x| format!("{x:?}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        ret
    }
}
