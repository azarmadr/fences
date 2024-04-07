use grid::Grid;
use std::{fmt, usize};

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

pub(crate) mod items;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
    /*
    Up,
    Down,
    Right,
    Left
    */
}

use Direction::*;

use self::items::{Fence, U2};
#[derive(Debug)]
pub struct Board {
    fences: [Grid<Fence>; 2],
    task: Grid<U2>,
}

pub type Fences = [Grid<Fence>; 2];
pub type Task = Grid<U2>;

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
        }
    }
    /*
    pub fn from_task_lines(task: &str, solution: Option<&str>) -> Self {
        let task = Grid::<U2>::from_lines(task);
        let (width, rows) = (task.width(), task.rows());
        Board {
            fences: if let Some(sol) = solution {
                [
                    Grid::from_string(width, &sol[0..(width * (rows + 1))]),
                    Grid::from_string(width + 1, &sol[(width * (rows + 1))..]),
                ]
            } else {
                [
                    Grid::<Fence>::new(task.rows() + 1, task.width()),
                    Grid::<Fence>::new(task.rows(), task.width() + 1),
                ]
            },
            task,
        }
    }
    */
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
    pub fn size(&self) -> (usize, usize) {
        self.task.size()
    }
    pub fn rows(&self) -> usize {
        self.task.rows()
    }
    pub fn cols(&self) -> usize {
        self.task.cols()
    }
    pub fn fences(&self) -> &Fences {
        &self.fences
    }
    pub fn task(&self) -> &Grid<U2> {
        &self.task
    }
    pub fn fences_mut(&mut self) -> &mut Fences {
        &mut self.fences
    }
    pub fn get_fence(&self, direction: Direction, row: usize, col: usize) -> Fence {
        self.fences[usize::from(matches!(direction, Vertical))][(row, col)]
    }
    /*
    pub fn get_solution(&self) -> String {
        self.fences
            .iter()
            .fold(String::new(), |a, x| a + "\n" + &x.to_string())
    }
    */
    pub fn result(&self) -> Option<bool> {
        unimplemented!();
    }
}

pub fn print_board(task: &Task, fences: &Fences) -> String {
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
                fences[0][(row, col)]
                    .map_or_else(|| ' ', |v| if v { BOX_HORIZONTAL } else { CROSS })
            );
        }
        f += &format!("{}\n", get_dot_char(row, cols));
        for col in 0..cols {
            f += &format!(
                "{}{}",
                fences[1][(row, col)].map_or_else(|| ' ', |v| if v { BOX_VERTICAL } else { CROSS }),
                char::from(task[(row, col)].clone())
            );
        }
        f += &format!(
            "{}\n",
            fences[1][(row, cols)].map_or_else(|| ' ', |v| if v { BOX_VERTICAL } else { CROSS })
        );
    }
    for col in 0..cols {
        f += &format!(
            "{}{}",
            get_dot_char(rows, col),
            fences[0][(rows, col)].map_or_else(|| ' ', |v| if v { BOX_HORIZONTAL } else { CROSS })
        );
    }
    f += &format!("{}", get_dot_char(rows, cols));
    f
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", print_board(&self.task, &self.fences))
    }
}
