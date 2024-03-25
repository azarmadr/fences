use either::Either;
use std;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
struct _Idx(usize, usize);

#[derive(Debug)]
pub struct Grid<T> {
    data: Vec<T>,
    rows: usize,
}

impl<T> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self
    where
        T: Default,
    {
        let mut data = Vec::new();
        data.resize_with(rows.checked_mul(cols).unwrap(), T::default);
        Grid { rows, data }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.data.len() / self.rows
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.rows
    }
    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.width())
    }

    pub fn subgrid_iter_mut(
        &mut self,
        idx: (usize, usize),
        size: (usize, usize),
    ) -> impl Iterator<Item = &mut T> {
        let bounds = (idx.0 + size.0, idx.1 + size.1);
        let size = self.width();
        let is_inside = move |i| {
            i / size >= idx.0 && i / size < bounds.0 && i % size >= idx.1 && i % size < bounds.1
        };
        self.data
            .iter_mut()
            .enumerate()
            .filter_map(move |(i, v)| if is_inside(i) { Some(v) } else { None })
    }
    pub fn subgrid_iter(
        &self,
        idx: (usize, usize),
        size: (usize, usize),
    ) -> impl Iterator<Item = &T> {
        let bounds = (idx.0 + size.0, idx.1 + size.1);
        let size = self.width();
        let is_inside = move |i| {
            i / size >= idx.0 && i / size < bounds.0 && i % size >= idx.1 && i % size < bounds.1
        };
        self.data
            .iter()
            .enumerate()
            .filter_map(move |(i, v)| if is_inside(i) { Some(v) } else { None })
    }
    /*
    pub fn iter_subgrids(
        &self,
        subgrid_width: usize,
        subgrid_height: usize,
        corners: bool,
    ) -> SubGridIterator<T> {
        SubGridIterator {
            grid: &self.data,
            subgrid_width,
            subgrid_height,
            corners,
            current_row: 0,
            current_col: 0,
        }
    }
    */
}

impl<T: Clone> Grid<T> {
    /*
    pub fn update_subgrid(&mut self, idx: (usize, usize), new_subgrid: &Grid<T>) {
        for (i, row) in new_subgrid.data.iter().enumerate() {
            for (j, value) in row.into_iter().enumerate() {
                self.data[idx.0 + i][idx.1 + j] = value.clone();
            }
        }
    }
    */
    pub fn rotate(&self) -> Self {
        let mut data = vec![];
        let size = self.size();
        let rows = self.data.len() / self.rows;
        for col in 0..size.1 {
            for row in (0..size.0).rev() {
                data.push(self[(row, col)].clone())
            }
        }
        Grid { data, rows }
    }
    pub fn clone(&self) -> Self {
        Grid {
            data: self.data.clone(),
            rows: self.rows,
        }
    }
}

impl<T> Grid<T>
where
    T: From<char>,
    char: From<T>,
    T: Clone,
{
    pub fn from_lines(text: &str) -> Self {
        Grid {
            data: text
                .lines()
                .flat_map(|x| x.chars().map(|x| T::from(x)).collect::<Vec<T>>())
                .collect::<Vec<T>>(),
            rows: text.lines().count(),
        }
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
        self.data
            .iter()
            .map(|x| char::from(x.clone()))
            .enumerate()
            .flat_map(|(i, c)| {
                if i % self.width() == 0 {
                    Either::Left(['\n', c].into_iter())
                } else {
                    Either::Right(std::iter::once(c))
                }
            })
            .skip(1)
            .collect()
    }
}

/*
impl<T> Grid<T>
where
    T: PartialEq,
{
    pub fn is_same(&self, other: &Grid<T>) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }

        for (row1, row2) in self.data.iter().zip(other.data.iter()) {
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
        if self.data.len() != other.len() {
            return false;
        }

        for (row1, row2) in self.data.iter().zip(other.iter()) {
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
*/

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
/*
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
*/

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.width() + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        let idx = row * self.width() + col;
        &mut self.data[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    #[test]
    fn rotation_works() {
        let grid = Grid {
            data: vec![0, 1, 2, 3],
            rows: 2,
        };
        let r_grid = grid.rotate();

        assert_eq!(2, r_grid[(0, 0)]);
        assert_eq!(0, r_grid[(0, 1)]);
        assert_eq!(3, r_grid[(1, 0)]);
        assert_eq!(1, r_grid[(1, 1)]);
    }
    #[test]
    fn rotation_works1() {
        let grid = Grid {
            data: vec![0, 1],
            rows: 1,
        };
        let r_grid = grid.rotate();

        println!("{r_grid:?}");

        assert_eq!(0, r_grid[(0, 0)]);
        assert_eq!(1, r_grid[(1, 0)]);
    }
}
