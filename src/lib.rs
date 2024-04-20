pub mod board;

pub use crate::board::Board;
pub use solver::BoardRule;

pub mod solver;

pub fn add_idx(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    (a.0 + b.0, a.1 + b.1)
}

pub fn sub_idx(a: (usize, usize), b: (usize, usize)) -> (usize, usize) {
    (a.0 - b.0, a.1 - b.1)
}
