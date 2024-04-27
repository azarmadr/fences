use colored::Colorize;
use grid::Grid;
use std::{collections::VecDeque, fmt};

const BOX_HORIZONTAL: char = '─';
const BOX_VERTICAL: char = '│';
const BOX_DOWN_RIGHT: char = '┌';
const BOX_DOWN_LEFT: char = '┐';
const BOX_UP_RIGHT: char = '└';
const BOX_UP_LEFT: char = '┘';
const BOX_VERTICAL_RIGHT: char = '├';
const BOX_VERTICAL_LEFT: char = '┤';
const BOX_HORIZONTAL_DOWN: char = '┬';
const BOX_HORIZONTAL_UP: char = '┴';
const BOX_VERTICAL_HORIZONTAL: char = '┼';
const DOT: char = '∙';
const CROSS: char = '×';

mod items;

#[derive(Debug)]
pub struct Move {
    pub direction: usize,
    pub idx: (usize, usize),
    pub value: bool,
    pub name: String,
}
pub use self::items::{Fence, U2};
#[derive(Debug)]
pub struct Board {
    fences: Fences,
    task: Task,
    moves: Vec<Move>,
}

pub type Fences = [Grid<Fence>; 2];
pub type Task = Grid<U2>;

pub fn print_board(task: &Task, fences: &Fences, color: bool) -> String {
    let paths = if color {
        let mut paths = get_paths(fences);
        paths.sort_by(|a, b| (b.len(), b[0]).cmp(&(a.len(), a[0])));
        paths
    } else {
        vec![]
    };
    let get_edge = |dir: usize, row, col| -> String {
        if let Some(edge) = fences[dir][(row, col)].0 {
            if edge {
                let e = format!(
                    "{}",
                    if dir == 1 {
                        BOX_VERTICAL
                    } else {
                        BOX_HORIZONTAL
                    }
                );
                if let Some(color) = paths.iter().position(|r| r.contains(&(dir, row, col))) {
                    format!(
                        "{}",
                        e.color(["white", "green", "yellow", "cyan", "purple", "red"][color % 6])
                    )
                } else {
                    e
                }
            } else {
                if color {
                    format!("{}", CROSS.to_string().truecolor(108, 108, 108))
                } else {
                    format!("{CROSS}")
                }
            }
        } else {
            " ".to_string()
        }
    };

    let (rows, cols) = task.size();
    let get_dot_char = |row, col| {
        let mut n = [Fence::default(); 4];
        if col < cols {
            n[0] = fences[0][(row, col)];
        }
        if row < rows {
            n[1] = fences[1][(row, col)]
        }
        if col > 0 {
            n[2] = fences[0][(row, col - 1)]
        }
        if row > 0 {
            n[3] = fences[1][(row - 1, col)]
        }
        let n = n.map(|v| v.is_some_and(|x| x));
        if n == [true; 4] {
            BOX_VERTICAL_HORIZONTAL
        } else if n == [true, true, false, false] {
            BOX_DOWN_RIGHT
        } else if n == [false, true, true, false] {
            BOX_DOWN_LEFT
        } else if n == [false, false, true, true] {
            BOX_UP_LEFT
        } else if n == [true, false, false, true] {
            BOX_UP_RIGHT
        } else if n == [false, true, true, true] {
            BOX_VERTICAL_LEFT
        } else if n == [true, false, true, true] {
            BOX_HORIZONTAL_UP
        } else if n == [true, true, false, true] {
            BOX_VERTICAL_RIGHT
        } else if n == [true, true, true, false] {
            BOX_HORIZONTAL_DOWN
        } else if n == [true, false, true, false] {
            BOX_HORIZONTAL
        } else if n == [false, true, false, true] {
            BOX_VERTICAL
        } else {
            DOT
        }
    };
    let mut f = String::default();
    for row in 0..rows {
        for col in 0..cols {
            f += &format!("{}", get_dot_char(row, col));
            f += &format!("{}", get_edge(0, row, col));
        }
        f += &format!("{}\n", get_dot_char(row, cols));
        for col in 0..cols {
            f += &format!(
                "{}{}",
                get_edge(1, row, col),
                char::from(task[(row, col)].clone())
            );
        }
        f += &format!("{}\n", get_edge(1, row, cols),);
    }
    for col in 0..cols {
        f += &format!("{}{}", get_dot_char(rows, col), get_edge(0, rows, col),);
    }
    f += &format!("{}", get_dot_char(rows, cols));
    f
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "      {}\n",
            (0..self.task.cols()).fold("".to_string(), |acc, x| format!("{acc}{x:2}"))
        )?;
        for (i, x) in print_board(&self.task, &self.fences, true)
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

impl Board {
    pub fn set_solution(&mut self, solution: &str) {
        self.fences
            .iter_mut()
            .flat_map(|f| f.iter_mut())
            .zip(solution.chars())
            .for_each(|(f, v)| *f = v.try_into().unwrap());
        /*
        let (cols, rows) = (self.cols(), self.rows());
        let b = cols * (rows + 1);
        for (i, c) in solution.chars().enumerate() {
            let (dir, row, col) = if i < b {
                (0, i % cols, i / cols)
            } else {
                (1, (i - b) % (cols + 1), (i - b) / (cols + 1))
            };
            self.fences[dir][(row, col)] = c.try_into().unwrap();
        }
        */
        log::info!("set_solution\n{self}");
    }
    pub fn solution(&self) -> String {
        self.fences
            .iter()
            .flat_map(|f| {
                f.iter().map(|e| match e.0 {
                    Some(true) => 'y',
                    _ => 'n',
                })
            })
            .collect()
    }
    #[inline]
    pub fn size(&self) -> (usize, usize) {
        self.task.size()
    }
    #[inline]
    pub fn rows(&self) -> usize {
        self.task.rows()
    }
    #[inline]
    pub fn cols(&self) -> usize {
        self.task.cols()
    }
    pub fn fences(&self) -> &Fences {
        &self.fences
    }
    pub fn task(&self) -> &Grid<U2> {
        &self.task
    }
    pub fn play(&mut self, direction: usize, idx: (usize, usize), value: bool, name: &str) {
        if let Some(curr) = self.fences[direction][idx].0 {
            if curr == value {
                log::trace!("Trying to overwrite an existing fence at [{direction}][{idx:?}]");
                return;
            } else {
                println!("Overwriting an existing fence {curr} with {value} by {name}")
            }
        }
        *self.fences[direction][idx] = Some(value);
        let m = Move {
            direction,
            idx,
            value,
            name: name.into(),
        };
        log::trace!("{:?}\n{}{self}", (m.direction, m.idx, m.value), m.name);
        self.moves.push(m);
    }
    pub fn edge(&self, e: &Edge) -> &Fence {
        &self.fences[e.0][(e.1, e.2)]
    }
    pub fn moves(&self) -> &Vec<Move> {
        &self.moves
    }
    pub fn reset_to(&mut self, to: usize) -> anyhow::Result<()> {
        if to > self.moves.len() {
            anyhow::bail!("Invalid reset entry")
        }
        while self.moves.len() > to {
            let e = self.moves.pop().unwrap();
            self.fences[e.direction][e.idx].0 = None;
        }
        anyhow::Ok(())
    }
    // #[inline]
    // pub fn get_dot_fences(&self, idx: (usize, usize)) -> Vec<(usize, usize, usize)> {}
    pub fn result(&self) -> Option<bool> {
        let (rows, cols) = self.size();
        #[derive(Debug)]
        struct Count {
            xs: usize,
            dashes: usize,
        }
        impl Count {
            fn incr(&mut self, value: Option<bool>) {
                match value {
                    Some(true) => self.dashes += 1,
                    Some(false) => self.xs += 1,
                    _ => (),
                }
            }
        }

        #[cfg(test)]
        print!("{self}");
        // Check for dots and tasks within the board
        for row in 0..=rows {
            for col in 0..=cols {
                let node = &mut Count { xs: 0, dashes: 0 };
                if col < cols {
                    node.incr(self.fences[0][(row, col)].0);
                }
                if row < rows {
                    node.incr(self.fences[1][(row, col)].0);
                }
                if col > 0 {
                    node.incr(self.fences[0][(row, col - 1)].0);
                }
                if row > 0 {
                    node.incr(self.fences[1][(row - 1, col)].0);
                }
                #[cfg(test)]
                println!("Node at {:?} -> {node:?}", (row, col));
                if node.dashes > 2 || node.dashes == 1 && node.xs == 3 {
                    return Some(false);
                }
            }
        }
        for row in 0..rows {
            for col in 0..cols {
                let task = &mut Count { xs: 0, dashes: 0 };
                task.incr(self.fences[0][(row, col)].0);
                task.incr(self.fences[1][(row, col)].0);
                task.incr(self.fences[0][(row + 1, col)].0);
                task.incr(self.fences[1][(row, col + 1)].0);
                #[cfg(test)]
                println!("Task at {:?} -> {task:?}", (row, col));
                if !self.task[(row, col)].is_ok(task.xs, task.dashes) {
                    return Some(false);
                }
            }
        }

        if self.task.indexed_iter().all(|((row, col), val)| {
            if val.0.is_none() {
                return true;
            }
            let task = &mut Count { xs: 0, dashes: 0 };
            task.incr(self.fences[0][(row, col)].0);
            task.incr(self.fences[1][(row, col)].0);
            task.incr(self.fences[0][(row + 1, col)].0);
            task.incr(self.fences[1][(row, col + 1)].0);
            #[cfg(test)]
            println!("Task at {:?} -> {task:?}", (row, col));
            self.task[(row, col)].0.is_some_and(|x| task.dashes as u8 == x)
        }) && has_one_path_and_is_circular(&self.fences)
        {
            return Some(true);
        } else {
        let paths = get_paths(&self.fences);
        if paths.iter()
            .any(|p| p.len() > 2 && are_linked(&p[0], p.last().unwrap())) {
                return Some(false)
            }
        }
        None
    }
}

type Edge = (usize, usize, usize);
pub fn are_linked(l: &Edge, r: &Edge) -> bool {
    if r.0 != l.0 {
        let (r, l) = if r.0 == 0 { (r, l) } else { (l, r) };
        [(0, 0), (0, -1), (1, -1), (1, 0)]
            .contains(&(r.1 as isize - l.1 as isize, r.2 as isize - l.2 as isize))
    } else {
        let diff = (r.1.abs_diff(l.1), r.2.abs_diff(l.2));
        if r.0 == 0 {
            diff == (0, 1)
        } else {
            diff == (1, 0)
        }
    }
    // println!("are_linked(r: {r:?}, l: {l:?}) -> {a}");
}

fn has_one_path_and_is_circular(fences: &Fences) -> bool {
    let paths = get_paths(fences);
    paths.len() == 1 && are_linked(&paths[0][0], paths[0].last().unwrap())
}
pub fn get_paths(fences: &Fences) -> Vec<Vec<(usize, usize, usize)>> {
    let mut dashes: Vec<_> = (0usize..2)
        .into_iter()
        .flat_map(|dir| {
            fences[dir]
                .indexed_iter()
                .filter_map(move |((row, col), val)| {
                    if val.is_some_and(|x| x) {
                        Some((dir, row, col))
                    } else {
                        None
                    }
                })
        })
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

impl core::str::FromStr for Board {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('#') {
            let mut mat = s.lines();
            let mut head = mat.next().expect("Header missing").split('#');
            let cols: usize = head.next().unwrap().parse().unwrap();
            let task =
                Grid::<U2>::from_vec(head.next().unwrap().chars().map(U2::from).collect(), cols);
            let mut board = Board {
                fences: [
                    Grid::<Fence>::new(task.rows() + 1, task.cols()),
                    Grid::<Fence>::new(task.rows(), task.cols() + 1),
                ],
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
                Grid::<U2>::from_vec(mat.join("").chars().map(U2::from).collect(), mat[0].len());
            let board = Board {
                fences: [
                    Grid::<Fence>::new(task.rows() + 1, task.cols()),
                    Grid::<Fence>::new(task.rows(), task.cols() + 1),
                ],
                task,
                moves: vec![],
            };

            Ok(board)
        }
    }
}

mod tests {
    use super::*;
#[test]
fn check_board_result() {
    assert_eq!("2#32  ".parse::<Board>().unwrap().result(), None);
    assert_eq!(
        "2#32  \n...-...-....".parse::<Board>().unwrap().result(),
        None
    );
    assert_eq!(
        "2#32  \n..--...-....".parse::<Board>().unwrap().result(),
        Some(false)
    );
    assert_eq!(
        "2#32  \n..x-...x....".parse::<Board>().unwrap().result(),
        Some(false)
    );
    assert_eq!(
        "2#32  \n-.-...--....".parse::<Board>().unwrap().result(),
        Some(false)
    );
    assert_eq!(
        "2#32  \n---..--.-.--".parse::<Board>().unwrap().result(),
        Some(true)
    );

    assert_eq!(
        "3#4  \n..-..-..--".parse::<Board>().unwrap().result(),
        Some(false)
    );
    assert_eq!(
        "3#4  \n-.--.-----".parse::<Board>().unwrap().result(),
        Some(false)
    );
    assert_eq!(
        "3#4  \n-..-..--..".parse::<Board>().unwrap().result(),
        Some(true)
    );
}
}
