use crate::grid::Grid;
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

mod items;

type Neighbors = [Fence; 4];

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

use Direction::*;

use self::items::{Fence, U2};
#[derive(Debug)]
pub struct Board {
    fences: [Grid<Fence>; 2],
    task: Grid<U2>,
}

type Fences = [Grid<Fence>; 2];

impl Board {
    pub fn from_task_string(width: usize, task: &str, solution: Option<&str>) -> Self {
        let task = Grid::<U2>::from_string(width, task);
        let (width, height) = (task.width(), task.height());
        Board {
            fences: if let Some(sol) = solution {
                [
                    Grid::from_string(width, &sol[0..width * (height + 1)]),
                    Grid::from_string(width + 1, &sol[(width * (height + 1))..]),
                ]
            } else {
                [
                    Grid::<Fence>::default(task.width(), task.height() + 1),
                    Grid::<Fence>::default(task.width() + 1, task.height()),
                ]
            },
            task,
        }
    }
    pub fn from_task_lines(task: &str, solution: Option<&str>) -> Self {
        let task = Grid::<U2>::from_lines(task);
        let (width, height) = (task.width(), task.height());
        Board {
            fences: if let Some(sol) = solution {
                [
                    Grid::from_string(width, &sol[0..(width * (height + 1))]),
                    Grid::from_string(width + 1, &sol[(width * (height + 1))..]),
                ]
            } else {
                [
                    Grid::<Fence>::default(task.width(), task.height() + 1),
                    Grid::<Fence>::default(task.width() + 1, task.height()),
                ]
            },
            task,
        }
    }
    pub fn set_solution(&mut self, solution: &str) {
        let (width, height) = (self.width(), self.height());
        let b = width * (height + 1);
        for (i, c) in solution.chars().enumerate() {
            let (dir, row, col) = if i < b {
                (0, i % width, i / width)
            } else {
                (1, (i - b) % (width + 1), (i - b) / (width + 1))
            };
            self.fences[dir][(row, col)] = c.into();
        }
    }
    pub fn width(&self) -> usize {
        self.task.width()
    }
    pub fn height(&self) -> usize {
        self.task.height()
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
    pub fn get_fence_char(&self, direction: Direction, row: usize, col: usize) -> char {
        let x = self.get_fence(direction, row, col);
        // println!("{direction:?}, {row}, {col}, {:?}", x);
        let c = match direction {
            Horizontal => BOX_HORIZONTAL,
            Vertical => BOX_VERTICAL,
        };
        match *x {
            Some(true) => c,
            Some(false) => CROSS,
            None => ' ',
        }
    }
    pub fn get_solution(&self) -> String {
        self.fences
            .iter()
            .fold(String::new(), |a, x| a + "\n" + &x.to_string())
    }
    pub fn get_dot_neighbors(&self, row: usize, col: usize) -> Neighbors {
        let mut n = [Fence::default(); 4];
        if col < self.width() {
            n[0] = self.get_fence(Horizontal, row, col);
        }
        if row < self.height() {
            n[1] = self.get_fence(Vertical, row, col)
        }
        if col > 0 {
            n[2] = self.get_fence(Horizontal, row, col - 1)
        }
        if row > 0 {
            n[3] = self.get_fence(Vertical, row - 1, col)
        }
        n
    }
    pub fn get_dot_char(&self, row: usize, col: usize) -> char {
        let n = self
            .get_dot_neighbors(row, col)
            .map(|v| v.is_some_and(|x| x));
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
    }
    pub fn result(&self) -> Option<bool> {
        unimplemented!();
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height() {
            for col in 0..self.width() {
                write!(f, "{}", self.get_dot_char(row, col))?;
                write!(f, "{}", self.get_fence_char(Horizontal, row, col))?;
            }
            write!(f, "{}\n", self.get_dot_char(row, self.width()))?;
            for col in 0..self.width() {
                write!(
                    f,
                    "{}{}",
                    self.get_fence_char(Vertical, row, col),
                    char::from(self.task[(row, col)])
                )?;
            }
            write!(f, "{}\n", self.get_fence_char(Vertical, row, self.width()))?;
        }
        for col in 0..self.width() {
            write!(
                f,
                "{}{}",
                self.get_dot_char(self.height(), col),
                self.get_fence_char(Horizontal, self.height(), col)
            )?;
        }
        write!(f, "{}\n", self.get_dot_char(self.height(), self.width()))
    }
}
