use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Grid<const ROWS: usize, const COLUMNS: usize, T>([[T; COLUMNS]; ROWS]);
impl<const ROWS: usize, const COLUMNS: usize, T> Grid<ROWS, COLUMNS, T> {
    pub const fn new(data: [[T; COLUMNS]; ROWS]) -> Self {
        Self(data)
    }
    pub fn from_fn<F: FnMut(GridIndex<ROWS, COLUMNS>) -> T>(mut f: F) -> Self {
        let mut y = 0;
        let data = [[(); COLUMNS]; ROWS].map(|row| {
            let mut x = 0;
            let new_row = row.map(|_| {
                let new_val = f(GridIndex::new(x, y));
                x += 1;
                new_val
            });
            y += 1;
            new_row
        });
        Grid(data)
    }
    pub fn rows(&self) -> impl Iterator<Item = &'_ [T; COLUMNS]> {
        self.0.iter()
    }
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &'_ mut [T; COLUMNS]> {
        self.0.iter_mut()
    }
    pub fn into_rows(self) -> impl Iterator<Item = [T; COLUMNS]> {
        self.0.into_iter()
    }
}
impl<const ROWS: usize, const COLUMNS: usize, T: Default> Default for Grid<ROWS, COLUMNS, T> {
    fn default() -> Self {
        Self::from_fn(|_| T::default())
    }
}
impl<const ROWS: usize, const COLUMNS: usize, T> Index<GridIndex<ROWS, COLUMNS>>
    for Grid<ROWS, COLUMNS, T>
{
    type Output = T;

    fn index(&self, index: GridIndex<ROWS, COLUMNS>) -> &Self::Output {
        self.0.index(index.y).index(index.x)
    }
}
impl<const ROWS: usize, const COLUMNS: usize, T> IndexMut<GridIndex<ROWS, COLUMNS>>
    for Grid<ROWS, COLUMNS, T>
{
    fn index_mut(&mut self, index: GridIndex<ROWS, COLUMNS>) -> &mut Self::Output {
        self.0.index_mut(index.y).index_mut(index.x)
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct GridIndex<const ROWS: usize, const COLUMNS: usize> {
    pub x: usize,
    pub y: usize,
}
impl<const ROWS: usize, const COLUMNS: usize> GridIndex<ROWS, COLUMNS> {
    const MIN: Self = Self {
        x: usize::MIN,
        y: usize::MIN,
    };
    const MAX: Self = Self {
        x: usize::MAX,
        y: usize::MAX,
    };
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
