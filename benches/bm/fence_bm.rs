use fences::board::*;
use grid::Grid;
use std::fmt;

#[derive(Debug)]
pub struct Board1 {
    fences: Vec<Fence>,
    task: Task,
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
impl Board1 {
    pub fn set_solution(&mut self, solution: &str) {
        self.fences
            .iter_mut()
            .zip(solution.chars())
            .for_each(|(f, v)| *f = v.try_into().unwrap());
        log::info!("set_solution\n{self}");
    }
    pub fn play(&mut self, direction: usize, idx: (usize, usize), value: bool, name: &str) {
        let e = self.e2id(&direction, &idx);
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
            name: name.into(),
        };
        log::trace!("{:?}\n{}{self}", (m.direction, m.idx, m.value), m.name);
        self.moves.push(m);
    }
    #[inline]
    pub fn edge(&self, e: &Edge) -> &Fence {
        let e = self.e2id(&e.0, &(e.1, e.2));
        &self.fences[e]
    }
    #[inline]
    fn e2id(&self, dir: &usize, idx: &(usize, usize)) -> usize {
        let b = self.task.cols() * (self.task.rows() + 1);
        let c = self.task.cols();
        match dir {
            0 => c * idx.1 + idx.0,
            1 => b + (c + 1) * idx.1 + idx.0,
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
            let task =
                Task::from_vec(
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
                    board.play(dir, (row, col), val, "");
                } else {
                    board.set_solution(l)
                }
            }
            Ok(board)
        } else {
            let mat: Vec<_> = s.lines().collect();
            assert!(mat.iter().all(|l| l.len() == mat.last().unwrap().len()));
            let task =
                Task::from_vec(
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
            let b = &mut match N {
                2 => "2#33",
                5 => "5#         2  3331 0 1 3  3",
                4 => "4#1  0    1 21 23",
                _ => unreachable!(),
            }
            .parse()
            .unwrap();
            fences::solver::solve(b);
        }
    };
}

b_parse! {b_parse, Board}
b_parse! {b1_parse, Board1}
b_solver! {b_solver, Board}
b_solver! {b1_solver, Board1}
