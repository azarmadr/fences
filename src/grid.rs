use std::ops::IndexMut;

use std::ops::Index;

use std;

#[derive(Debug)]
struct _Idx(usize, usize);

type GridV<T> = Vec<Vec<T>>;
#[derive(Debug)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T: Default + Clone> Grid<T> {
    pub fn default(width: usize, height: usize) -> Self {
        Grid(vec![vec![T::default(); width]; height])
    }
}
impl<T: Clone> Grid<T> {
    pub fn update_subgrid(&mut self, idx: (usize, usize), new_subgrid: &Grid<T>) {
        for (i, row) in new_subgrid.0.iter().enumerate() {
            for (j, value) in row.into_iter().enumerate() {
                self.0[idx.0 + i][idx.1 + j] = value.clone();
            }
        }
    }
}

impl<T> Grid<T>
where
    // Vec<T>: FromIterator<char>,
    T: From<char>,
    char: From<T>,
    T: Clone,
{
    pub fn from_lines(text: &str) -> Self {
        Grid(
            text.lines()
                .map(|x| x.chars().map(|x| T::from(x)).collect::<Vec<T>>())
                .collect::<Vec<Vec<T>>>(),
        )
    }
    pub fn from_string(width: usize, text: &str) -> Self {
        let mut result = String::with_capacity(text.len() + text.len() / width); // Rough estimate of the resulting string length

        for (i, c) in text.chars().enumerate() {
            if i > 0 && i % width == 0 {
                result.push('\n');
            }
            result.push(c);
        }
        Self::from_lines(&result)
    }
    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .flat_map(|x| {
                x.iter()
                    .map(|x| char::from(x.clone()))
                    .chain(std::iter::once('\n'))
            })
            .collect()
    }
}

impl<T> Grid<T>
where
    T: PartialEq,
{
    pub fn is_same(&self, other: &Grid<T>) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        for (row1, row2) in self.0.iter().zip(other.0.iter()) {
            if row1.len() != row2.len() {
                return false;
            }
            for (elem1, elem2) in row1.iter().zip(row2.iter()) {
                if elem1 != elem2 {
                    return false;
                }
            }
        }

        true
    }
    pub fn is_same_with_references(&self, other: &GridV<&T>) -> bool {
        if self.0.len() != other.len() {
            return false;
        }

        for (row1, row2) in self.0.iter().zip(other.iter()) {
            if row1.len() != row2.len() {
                return false;
            }
            for (elem1, elem2) in row1.iter().zip(row2.iter()) {
                if *elem1 != **elem2 {
                    return false;
                }
            }
        }

        true
    }
}

impl<T> Grid<T> {
    #[inline]
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn subgrid_iter_mut(
        &mut self,
        idx: (usize, usize),
        size: (usize, usize),
    ) -> impl Iterator<Item = &mut T> {
        // if idx.0 + size.0 > self.width() || idx.1 + size.1 > self.height() {
        // return false;
        // }
        self.0
            .iter_mut()
            .skip(idx.0)
            .take(size.0)
            .flat_map(move |x| x.iter_mut().skip(idx.1).take(size.1))
    }
    pub fn subgrid_iter(
        &self,
        idx: (usize, usize),
        size: (usize, usize),
    ) -> impl Iterator<Item = &T> {
        // if idx.0 + size.0 > self.width() || idx.1 + size.1 > self.height() {
        // return false;
        // }
        self.0
            .iter()
            .skip(idx.0)
            .take(size.0)
            .flat_map(move |x| x.iter().skip(idx.1).take(size.1))
    }
    pub fn iter_subgrids(
        &self,
        subgrid_width: usize,
        subgrid_height: usize,
        corners: bool,
    ) -> SubGridIterator<T> {
        SubGridIterator {
            grid: &self.0,
            subgrid_width,
            subgrid_height,
            corners,
            current_row: 0,
            current_col: 0,
        }
    }
}

// let grid = Grid(vec![
//     vec![1, 2, 3, 4],
//     vec![5, 6, 7, 8],
//     vec![9, 10, 11, 12],
//     vec![13, 14, 15, 16],
// ]);
//
// let subgrid_width = 2;
// let subgrid_height = 2;
//
// let mut subgrid_iter = grid.iter_subgrids(subgrid_width, subgrid_height);
//
// while let Some(subgrid) = subgrid_iter.next() {
//     println!("{:?}", subgrid);
// }
pub struct SubGridIterator<'a, T> {
    grid: &'a GridV<T>,
    subgrid_width: usize,
    subgrid_height: usize,
    current_row: usize,
    current_col: usize,
    corners: bool,
}

impl<'a, T> Iterator for SubGridIterator<'a, T> {
    type Item = GridV<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row + self.subgrid_height > self.grid.len() {
            return None;
        }

        let subgrid: GridV<&T> = self
            .grid
            .iter()
            .skip(self.current_row)
            .take(self.subgrid_height)
            .map(|row| {
                row.iter()
                    .skip(self.current_col)
                    .take(self.subgrid_width)
                    .collect()
            })
            .collect();

        if self.corners {
            self.current_col = if self.current_col == 0 {
                self.grid[0].len() - self.subgrid_width
            } else {
                self.current_row += self.grid.len() - self.subgrid_height;
                0
            }
        } else {
            if self.current_col + self.subgrid_width < self.grid[0].len() {
                self.current_col += 1;
            } else {
                self.current_col = 0;
                self.current_row += 1;
            }
        }

        Some(subgrid)
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.0[row][col]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.0[row][col]
    }
}
