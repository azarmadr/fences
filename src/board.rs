use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct U2(bool, bool);

impl U2 {
    fn from_char(arg: char) -> U2 {
        match arg {
            '0' => U2(false, false),
            '1' => U2(false, true),
            '2' => U2(true, false),
            '3' => U2(true, true),
            _ => unreachable!(),
        }
    }
    fn to_char(&self) -> char {
        match self {
            U2(false, false) => '0',
            U2(false, true) => '1',
            U2(true, false) => '2',
            U2(true, true) => '3',
        }
    }
}
impl From<U2> for u8 {
    fn from(arg: U2) -> Self {
        match arg {
            U2(false, false) => 0,
            U2(false, true) => 1,
            U2(true, false) => 2,
            U2(true, true) => 3,
        }
    }
}

impl From<u8> for U2 {
    fn from(arg: u8) -> Self {
        assert!(arg < 4);
        match arg {
            0 => U2(false, false),
            1 => U2(false, true),
            2 => U2(true, false),
            3 => U2(true, true),
            _ => unreachable!(),
        }
    }
}

type Neighbors = [Option<bool>; 4];

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

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}
use Direction::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    width: u8,
    height: u8,
    fences: Vec<Option<bool>>,
    task: Vec<Option<U2>>,
}

impl Board {
    pub fn new(width: u8, height: u8) -> Self {
        Board {
            width,
            height,
            fences: vec![None; (2 * width * height + width + height) as usize],
            task: vec![None; (width * height) as usize],
        }
    }
    pub fn get_fence(&self, direction: Direction, row: u8, col: u8) -> Option<bool> {
        match direction {
            Horizontal => assert!(row <= self.height && col < self.width),
            Vertical => assert!(row < self.height && col <= self.width),
        };
        self.fences[match direction {
            Horizontal => row * self.width + col,
            Vertical => self.width * (self.height + 1) + (row * (self.width + 1) + col),
        } as usize]
    }
    pub fn set_fence(&mut self, direction: Direction, row: u8, col: u8, fence: Option<bool>) {
        match direction {
            Horizontal => assert!(row <= self.height && col < self.width),
            Vertical => assert!(row < self.height && col <= self.width),
        };
        self.fences[match direction {
            Horizontal => row * self.width + col,
            Vertical => self.width * (self.height + 1) + (row * (self.width + 1) + col),
        } as usize] = fence
    }
    pub fn get_fence_char(&self, direction: Direction, row: u8, col: u8) -> char {
        let x = self.get_fence(direction, row, col);
        // println!("{direction:?}, {row}, {col}, {:?}", x);
        let c = match direction {
            Horizontal => BOX_HORIZONTAL,
            Vertical => BOX_VERTICAL,
        };
        match x {
            Some(true) => c,
            Some(false) => CROSS,
            None => ' ',
        }
    }
    pub fn tasks(&self) -> String {
        self.task
            .iter()
            .map(|x| match x {
                Some(x) => x.to_char().to_string(),
                None => " ".to_string(),
            })
            .collect::<Vec<_>>()
            .join("")
    }
    pub fn set_numbers(&mut self, task: &str) {
        self.task = task
            .chars()
            .map(|x| {
                if x == ' ' {
                    None
                } else {
                    Some(U2::from_char(x))
                }
            })
            .collect();
    }
    pub fn set_solution(&mut self, solution: &str) {
        self.fences = solution
            .chars()
            .filter_map(|x| match x {
                'y' => Some(Some(true)),
                'n' => Some(Some(false)),
                'u' => Some(None),
                _ => None,
            })
            .collect()
    }
    pub fn get_solution(&self) -> String {
        self.fences
            .iter()
            .map(|x| match x {
                Some(true) => "y",
                Some(false) => "n",
                None => "u",
            } as &str)
            .collect::<Vec<_>>()
            .join("")
    }
    pub fn height(&self) -> u8 {
        self.height
    }
    pub fn width(&self) -> u8 {
        self.width
    }
    pub fn get_task_neighbors(&self, row: u8, col: u8) -> Neighbors {
        assert!(row < self.width && col < self.height);
        [
            self.get_fence(Horizontal, row, col),
            self.get_fence(Horizontal, row + 1, col),
            self.get_fence(Vertical, row, col),
            self.get_fence(Vertical, row, col + 1),
        ]
    }
    pub fn get_dot_neighbors(&self, row: u8, col: u8) -> Neighbors {
        let mut n = [None; 4];
        if col < self.width {
            n[0] = self.get_fence(Horizontal, row, col);
        }
        if row < self.height {
            n[1] = self.get_fence(Vertical, row, col)
        }
        if col > 0 {
            n[2] = self.get_fence(Horizontal, row, col - 1)
        }
        if row > 0 {
            n[3] = self.get_fence(Vertical, row - 1, col)
        }
        return n;
    }
    pub fn get_dot_char(&self, row: u8, col: u8) -> char {
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
        for row in 0..self.height {
            for col in 0..self.width {
                write!(f, "{}", self.get_dot_char(row, col))?;
                write!(f, "{}", self.get_fence_char(Horizontal, row, col))?;
            }
            write!(f, "{}\n", self.get_dot_char(row, self.width))?;
            for col in 0..self.width {
                write!(
                    f,
                    "{}{}",
                    self.get_fence_char(Vertical, row, col),
                    self.task[(row * self.width + col) as usize]
                        .map_or_else(|| ' ', |x| x.to_char())
                )?;
            }
            write!(f, "{}\n", self.get_fence_char(Vertical, row, self.width))?;
        }
        for col in 0..self.width {
            write!(
                f,
                "{}{}",
                self.get_dot_char(self.height, col),
                self.get_fence_char(Horizontal, self.height, col)
            )?;
        }
        write!(f, "{}\n", self.get_dot_char(self.height, self.width))
    }
}
