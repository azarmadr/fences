use grid::Grid;
use std::{collections::VecDeque, fmt};
use colored::Colorize;

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
    let paths = if color { get_paths(fences) } else { vec![] };
    let get_edge = |dir: usize, row, col| -> String {
        if let Some(edge) = fences[dir][(row, col)].0 {
            if edge {
                let e = format!("{}",if dir == 1 {BOX_VERTICAL} else {BOX_HORIZONTAL});
                if let Some(color) = paths.iter().position(|r| r.contains(&(dir, row, col))) {
                   format!("{}", e.color(["green","blue", "cyan"][color % 3]))
                } else {
                    e
                }
            }
            else {
                if color {format!("{}",CROSS.to_string().red())} else {format!("{CROSS}")}
            }
        } else {" ".to_string()}
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
            f += &format!(
                "{}",
                get_edge(0, row, col)
            );
        }
        f += &format!("{}\n", get_dot_char(row, cols));
        for col in 0..cols {
            f += &format!(
                "{}{}",
                get_edge(1, row, col),
                char::from(task[(row, col)].clone())
            );
        }
        f += &format!(
            "{}\n",
            get_edge(1, row, cols),
        );
    }
    for col in 0..cols {
        f += &format!(
            "{}{}",
            get_dot_char(rows, col),
            get_edge(0, rows, col),
        );
    }
    f += &format!("{}", get_dot_char(rows, cols));
    f
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "   {}\n",
            (0..self.task.cols()).fold("".to_string(), |acc, x| format!("{acc}{x:2}"))
        )?;
        for (i, x) in print_board(&self.task, &self.fences, true).lines().enumerate() {
            write!(
                f,
                "{}",
                if i % 2 == 1 {
                    format!("{:3}{x}\n", i / 2)
                } else {
                    format!("   {x}\n")
                }
            )?;
        }
        Ok(())
    }
}

impl Board {
    pub fn from_task_string(rows: usize, task: &str, solution: Option<&str>) -> Self {
        let task = Grid::<U2>::from_vec(task.chars().map(U2::from).collect(), rows);
        let bound = task.cols() * (task.rows() + 1);
        Board {
            fences: if let Some(sol) =
                solution.map(|s| s.chars().map(Fence::from).collect::<Vec<Fence>>())
            {
                [
                    Grid::<Fence>::from_vec(sol[0..bound].to_vec(), task.cols()),
                    Grid::<Fence>::from_vec(sol[bound..].to_vec(), task.cols() + 1),
                ]
            } else {
                [
                    Grid::<Fence>::new(task.rows() + 1, task.cols()),
                    Grid::<Fence>::new(task.rows(), task.cols() + 1),
                ]
            },
            task,
            moves: vec![],
        }
    }

    pub fn set_solution(&mut self, solution: &str) {
        let (cols, rows) = (self.cols(), self.rows());
        let b = cols * (rows + 1);
        for (i, c) in solution.chars().enumerate() {
            let (dir, row, col) = if i < b {
                (0, i % cols, i / cols)
            } else {
                (1, (i - b) % (cols + 1), (i - b) / (cols + 1))
            };
            self.fences[dir][(row, col)] = c.into();
        }
    }
    #[inline]
    pub fn size(&self) -> (usize, usize) {
        self.task.size()
    }
    #[inline]
    fn rows(&self) -> usize {
        self.task.rows()
    }
    #[inline]
    fn cols(&self) -> usize {
        self.task.cols()
    }
    pub fn fences(&self) -> &Fences {
        &self.fences
    }
    pub fn task(&self) -> &Grid<U2> {
        &self.task
    }
    /*
    pub fn get_solution(&self) -> String {
        self.fences
            .iter()
            .fold(String::new(), |a, x| a + "\n" + &x.to_string())
    }
    */
    pub fn play(
        &mut self,
        direction: usize,
        idx: (usize, usize),
        value: bool,
        name: impl Into<String>,
    ) {
        if self.fences[direction][idx].is_some_and(|x| x == value) {
            log::trace!("Trying to overwrite an existing fence at [{direction}][{idx:?}]");
            return;
        }
        *self.fences[direction][idx] = Some(value);
        self.moves.push(Move {
            direction,
            idx,
            value,
            name: name.into(),
        });
    }
    pub fn moves(&self) -> &Vec<Move> {
        &self.moves
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
            if val.is_none() {
                return true;
            }
            let task = &mut Count { xs: 0, dashes: 0 };
            task.incr(self.fences[0][(row, col)].0);
            task.incr(self.fences[1][(row, col)].0);
            task.incr(self.fences[0][(row + 1, col)].0);
            task.incr(self.fences[1][(row, col + 1)].0);
            #[cfg(test)]
            println!("Task at {:?} -> {task:?}", (row, col));
            self.task[(row, col)].0.is_some_and(|x| {
                let mut val = 0;
                if x[0] {
                    val += 1
                }
                if x[1] {
                    val += 2
                }
                val == task.dashes
            })
        }) && has_one_path_and_is_circular(&self.fences)
        {
            return Some(true);
        }
        None
    }
}

type Edge = (usize, usize, usize);
fn are_linked(l: &Edge, r: &Edge) -> bool {
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
fn get_paths(fences: &Fences) -> Vec<Vec<(usize, usize, usize)>> {
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

#[test]
fn check_board_result() {
    assert_eq!(Board::from_task_string(2, "32  ", None).result(), None);
    assert_eq!(
        Board::from_task_string(2, "32  ", Some("...-...-....")).result(),
        None
    );
    assert_eq!(
        Board::from_task_string(2, "32  ", Some("..--...-....")).result(),
        Some(false)
    );
    assert_eq!(
        Board::from_task_string(2, "32  ", Some("..x-...x....")).result(),
        Some(false)
    );
    assert_eq!(
        Board::from_task_string(2, "32  ", Some("-.-...--....")).result(),
        Some(false)
    );
    assert_eq!(
        Board::from_task_string(2, "32  ", Some("---..--.-.--")).result(),
        Some(true)
    );
}
