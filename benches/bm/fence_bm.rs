use fences::{board::*, solver::*, *};
use grid::Grid;
use std::fmt;

#[derive(Debug)]
pub struct Board1 {
    fences: Vec<Fence>,
    task: Tasks,
    moves: Vec<Move>,
}
impl fmt::Display for Board1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "      {}\n",
            (0..self.task.cols()).fold("".to_string(), |acc, x| format!("{acc}{x:2}"))
        )?;
        let b = self.task.cols() * (self.task.rows() + 1);
        for (i, x) in print_board(
            &self.task,
            &[
                Grid::from_vec(self.fences[0..b].to_vec(), self.task.cols()),
                Grid::from_vec(self.fences[b..].to_vec(), self.task.cols() + 1),
            ],
            true,
        )
        .lines()
        .enumerate()
        {
            write!(
                f,
                "{}",
                if i % 2 == 1 {
                    format!("{:3} ║ {x} ║{0:3}\n", i / 2)
                } else {
                    format!("    ║ {x} ║\n")
                }
            )?;
        }
        write!(
            f,
            "      {}\n",
            (0..self.task.cols()).fold("".to_string(), |acc, x| format!("{acc}{x:2}"))
        )?;
        Ok(())
    }
}
impl BoardGeom for Board1 {
    fn size(&self) -> (usize, usize) {
        self.task.size()
    }
    fn rotate(&mut self) {
        unimplemented!()
    }
}
impl FencesSolver for Board1 {
    fn set_solution(&mut self, solution: &str) {
        self.fences
            .iter_mut()
            .zip(solution.chars())
            .for_each(|(f, v)| *f = v.try_into().unwrap());
        log::info!("set_solution\n{self}");
    }
    fn play(&mut self, direction: usize, idx: (usize, usize), value: bool, name: String) {
        let e = self.e2id(direction, idx);
        if let Some(curr) = *self.fences[e] {
            if curr == value {
                log::trace!("Trying to overwrite an existing fence at [{direction}][{idx:?}]");
                return;
            } else {
                log::warn!("Overwriting an existing fence {curr} with {value} by {name}")
            }
        }
        *self.fences[e] = Some(value);
        let m = Move {
            direction,
            idx,
            value,
            name,
        };
        log::trace!("{:?}\n{}{self}", (m.direction, m.idx, m.value), m.name);
        self.moves.push(m);
    }
    fn tasks(&self) -> &Tasks {
        &self.task
    }
    fn edge(&self, dir: usize, idx: Idx) -> &Fence {
        let id = self.e2id(dir, idx);
        &self.fences[id]
    }
    fn task(&self, idx: Idx) -> Task {
        self.task[idx]
    }
    fn fences_iter(&self) -> impl Iterator<Item = (Edge, &Fence)> {
        self.fences
            .iter()
            .enumerate()
            .map(|(id, v)| (self.id2e(id), v))
    }
}
impl Board1 {
    #[inline]
    fn id2e(&self, id: usize) -> Edge {
        let b = self.cols() * (self.rows() + 1);
        let c = self.cols();
        if id < b {
            (0, id / c, id % c)
        } else {
            let id = id - b;
            (1, id / (c + 1), id % (c + 1))
        }
    }
    #[inline]
    fn e2id(&self, dir: usize, idx: (usize, usize)) -> usize {
        let c = self.task.cols();
        match dir {
            0 => c * idx.0 + idx.1,
            1 => self.task.cols() * (self.task.rows() + 1) + (c + 1) * idx.0 + idx.1,
            _ => unreachable!(),
        }
    }
}
impl core::str::FromStr for Board1 {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('#') {
            let mut mat = s.lines();
            let mut head = mat.next().expect("Header missing").split('#');
            let cols: usize = head.next().unwrap().parse().unwrap();
            let task = Tasks::from_vec(
                head.next()
                    .unwrap()
                    .chars()
                    .map(|x| x.to_string().parse().ok())
                    .collect(),
                cols,
            );
            let mut fences = vec![];
            fences.resize_with(
                2 * task.rows() * task.cols() + task.rows() + task.cols(),
                Fence::default,
            );
            let mut board = Self {
                fences,
                task,
                moves: vec![],
            };
            for l in mat {
                if l.is_empty() {
                    continue;
                }
                if l.starts_with(|c| matches!(c, '0' | '1')) {
                    let mut m = l.split_whitespace();
                    let dir = m.next().unwrap().parse().unwrap();
                    let row = m.next().unwrap().parse().unwrap();
                    let col = m.next().unwrap().parse().unwrap();
                    let val = match m.next().unwrap() {
                        "y" | "-" => true,
                        "n" | "x" => false,
                        _ => unreachable!("Invalid value"),
                    };
                    board.play(dir, (row, col), val, "".to_string());
                } else {
                    board.set_solution(l)
                }
            }
            Ok(board)
        } else {
            let mat: Vec<_> = s.lines().collect();
            assert!(mat.iter().all(|l| l.len() == mat.last().unwrap().len()));
            let task = Tasks::from_vec(
                mat.join("")
                    .chars()
                    .map(|x| x.to_string().parse().ok())
                    .collect(),
                mat[0].len(),
            );
            let mut fences = vec![];
            fences.resize_with(
                2 * task.rows() * task.cols() + task.rows() + task.cols(),
                Fence::default,
            );
            let board = Self {
                fences,
                task,
                moves: vec![],
            };

            Ok(board)
        }
    }
}

macro_rules! b_parse {($a:ident, $b:ident) => {
#[divan::bench(consts = [2, 5, 4])]
fn $a<const N: usize>() -> Result<$b, &'static str> {
    match N {
        2 => "2#22\nyyyynnn",
        5 => "5#         2  3331 0 1 3  3
yyyyynyyyynnynynynynnynynynynyynnnnyyynnnnyyyyyyynnnnyyyyyyy",
        4 => "4#1  0    1 21 23 
0 0 0 n
1 3 2 y
1 3 3 y
0 2 0 n
0 3 1 y
nynnnnnnnnynnynnnnynnyynnnyynnnynynnnyyn",
_ => unreachable!()
    }.parse()
}
}}
macro_rules! b_solver {
    ($a:ident, $b:ident) => {
        #[divan::bench(consts = [2, 5, 4])]
        fn $a<const N: usize>() {
            let b: &mut $b = &mut match N {
                2 => "2#33",
                4 => "4#1  0    1 21 23 ",
                5 => "5#         2  3331 0 1 3  3",
                _ => unreachable!(),
            }
            .parse()
            .unwrap();
            fences::solver::solve(b);
        }
    };
}

b_parse! {parse_b, Board}
b_parse! {parse_b1, Board1}
b_solver! {solver_b, Board}
b_solver! {solver_b1, Board1}
