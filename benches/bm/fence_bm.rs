use fences::{board::*, solver::*, *};
use grid::Grid;
use std::fmt;

#[derive(Debug)]
pub struct Board1 {
    fences: Vec<Fence>,
    tasks: Vec<Task>,
    moves: Vec<Move>,
    cols: usize,
}
impl fmt::Display for Board1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "      {}\n",
            (0..self.cols).fold("".to_string(), |acc, x| format!("{acc}{x:2}"))
        )?;
        let b = self.cols * (self.rows() + 1);
        for (i, x) in print_board(
            &Grid::from_vec(self.tasks.clone(), self.cols),
            &[
                Grid::from_vec(self.fences[0..b].to_vec(), self.cols),
                Grid::from_vec(self.fences[b..].to_vec(), self.cols + 1),
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
            (0..self.cols).fold("".to_string(), |acc, x| format!("{acc}{x:2}"))
        )?;
        Ok(())
    }
}
impl BoardGeom for Board1 {
    fn size(&self) -> (usize, usize) {
        (self.tasks.len() / self.cols, self.cols)
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
            }
            log::warn!("Overwriting an existing fence {curr} with {value} by {name}")
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
    fn edge(&self, dir: usize, idx: Idx) -> &Fence {
        let id = self.e2id(dir, idx);
        &self.fences[id]
    }
    fn task(&self, idx: Idx) -> &Task {
        &self.tasks[self.p2id(idx)]
    }
    fn fences_iter(&self) -> impl Iterator<Item = (Edge, &Fence)> {
        self.fences
            .iter()
            .enumerate()
            .map(|(id, v)| (self.id2e(id), v))
    }
    fn tasks_iter(&self) -> impl Iterator<Item = (Idx, &Task)> {
        self.tasks
            .iter()
            .enumerate()
            .map(|(id, v)| (self.id2p(id), v))
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
    pub fn p2id(&self, p: Idx) -> usize {
        p.0 * self.cols + p.1
    }
    #[inline]
    pub fn id2p(&self, id: usize) -> Idx {
        (id / self.cols, id % self.cols)
    }
    #[inline]
    fn e2id(&self, dir: usize, idx: (usize, usize)) -> usize {
        let c = self.cols;
        match dir {
            0 => c * idx.0 + idx.1,
            1 => self.cols * (self.rows() + 1) + (c + 1) * idx.0 + idx.1,
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
            let tasks: Vec<_> = head
                .next()
                .unwrap()
                .chars()
                .map(|x| x.to_string().parse().ok())
                .collect();
            let rows = tasks.len() / cols;
            let mut fences = vec![];
            fences.resize_with(2 * rows * cols + rows + cols, Fence::default);
            let mut board = Self {
                fences,
                tasks,
                moves: vec![],
                cols,
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
            let cols = mat[0].len();
            let rows = mat.len();
            let tasks = mat
                .join("")
                .chars()
                .map(|x| x.to_string().parse().ok())
                .collect();
            let mut fences = vec![];
            fences.resize_with(2 * rows * cols + rows + cols, Fence::default);
            let board = Self {
                cols,
                fences,
                tasks,
                moves: vec![],
            };

            Ok(board)
        }
    }
}

macro_rules! b_parse {($a:ident, $b:ident) => {
#[divan::bench(consts = [2, 4, 5, 15, 30])]
fn $a<const N: usize>() -> Result<$b, &'static str> {
    match N {
        2 => "2#22\nyyyynnn",
        5 => "5#         2  3331 0 1 3  3
yyyyynyyyynnynynynynnynynynynyynnnnyyynnnnyyyyyyynnnnyyyyyyy",
        15 => "15#3 32 1   23  3  3   2    23  3  0322  0    3 3       3  3  21  33 3    1 3 2 1 121  2  3      3    0   23 222     2   0 2  2 3       32 3    21 3   2     32221   2  3     1 31  2  3021231   2222     222   23    222 1232 3    ",
                30 => "30#   32 32 332 23 32 23 3 12 1  2  22               2   022  23   2211  23 2 2   2212   2  32    3      02 02 3     3    22   0  123  32  3 1   1     2 32  0   2 0 2 1 32 1 3 0  222 3  2 31 1  3 2 3 2212    22   3 2213  2   3 1  3    3  21 2  112131  23   2    20   2  3   1   3 33  2      3   2 1 1213  22 2 2 23 2  22  1 3 2 2   2321 213  13222   1     3 3  1        2 21 23 233 23 122 02 131 33 32  2 1 20  203  01  2   2 1 22  3   33 212    0    2 3   21 1  2  2 22  131 11 3     1 22 3    122 3 22   22 0 1 11 3   212 3   2 1   2 22 3223 231    1    1  3     1 10  22 12  3 202 2110        31 1 2   2   22      10  1 322    22  1  2  32  3   2  2  2  3   213     321      1113 21    32 32  3   2  12122       3  3 12 3  3  220 122 3 0211  12  0 1 2 2  2 0   3     3 12 2223 3 1 3 3 2   1 231 2  32      22  12  22  2 3 123  32  2322   3  313  13 3 3    1 21 1  2 1       13       32   2 3   1222 3",
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
        #[divan::bench(consts = [2, 4, 5, 15, 30])]
        fn $a<const N: usize>() {
            let b: &mut $b = &mut match N {
                2 => "2#33",
                4 => "4#1  0    1 21 23 ",
                5 => "5#         2  3331 0 1 3  3",
                15 => "15#3 32 1   23  3  3   2    23  3  0322  0    3 3       3  3  21  33 3    1 3 2 1 121  2  3      3    0   23 222     2   0 2  2 3       32 3    21 3   2     32221   2  3     1 31  2  3021231   2222     222   23    222 1232 3    ",
                30 => "30#   32 32 332 23 32 23 3 12 1  2  22               2   022  23   2211  23 2 2   2212   2  32    3      02 02 3     3    22   0  123  32  3 1   1     2 32  0   2 0 2 1 32 1 3 0  222 3  2 31 1  3 2 3 2212    22   3 2213  2   3 1  3    3  21 2  112131  23   2    20   2  3   1   3 33  2      3   2 1 1213  22 2 2 23 2  22  1 3 2 2   2321 213  13222   1     3 3  1        2 21 23 233 23 122 02 131 33 32  2 1 20  203  01  2   2 1 22  3   33 212    0    2 3   21 1  2  2 22  131 11 3     1 22 3    122 3 22   22 0 1 11 3   212 3   2 1   2 22 3223 231    1    1  3     1 10  22 12  3 202 2110        31 1 2   2   22      10  1 322    22  1  2  32  3   2  2  2  3   213     321      1113 21    32 32  3   2  12122       3  3 12 3  3  220 122 3 0211  12  0 1 2 2  2 0   3     3 12 2223 3 1 3 3 2   1 231 2  32      22  12  22  2 3 123  32  2322   3  313  13 3 3    1 21 1  2 1       13       32   2 3   1222 3",
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
