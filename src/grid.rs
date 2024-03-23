use std::ops::IndexMut;

use std::ops::Index;

use std;

#[derive(Debug)]
pub(crate) struct Grid<T>(Vec<Vec<T>>);

impl<T: Default + Clone> Grid<T> {
    pub fn default(width: usize, height: usize) -> Self {
        Grid(vec![vec![T::default(); width]; height])
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
    pub fn from_string(text: &str, width: usize) -> Self {
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

impl<T> Grid<T> {
    #[inline]
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.0.len()
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.0[col][row]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.0[col][row]
    }
}
