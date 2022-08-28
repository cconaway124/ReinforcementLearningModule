use std::collections::HashSet;

use rand::seq::SliceRandom;

use crate::grid::{Grid, GridIndex};

#[derive(Debug, Default, Clone, Copy)]
pub enum Tile {
    #[default]
    Open,
    Trap,
    Reward,
}

#[derive(Debug, Default)]
pub struct Maze<const ROWS: usize, const COLUMNS: usize> {
    grid: Grid<ROWS, COLUMNS, Tile>,
    start: GridIndex<ROWS, COLUMNS>,
    reward: GridIndex<ROWS, COLUMNS>,
}

impl<const ROWS: usize, const COLUMNS: usize> Maze<ROWS, COLUMNS> {
    pub const fn new(
        grid: Grid<ROWS, COLUMNS, Tile>,
        start: GridIndex<ROWS, COLUMNS>,
        reward: GridIndex<ROWS, COLUMNS>,
    ) -> Self {
        Self {
            grid,
            start,
            reward,
        }
    }
    pub const fn from_tiles(
        data: [[Tile; COLUMNS]; ROWS],
        start: GridIndex<ROWS, COLUMNS>,
        reward: GridIndex<ROWS, COLUMNS>,
    ) -> Self {
        Self::new(Grid::new(data), start, reward)
    }
    pub fn from_fn<F: FnMut(GridIndex<ROWS, COLUMNS>) -> Tile>(
        mut f: F,
        start: GridIndex<ROWS, COLUMNS>,
        reward: GridIndex<ROWS, COLUMNS>,
    ) -> Self {
        let mut y = 0;
        let tiles = [[(); COLUMNS]; ROWS].map(|row| {
            let mut x = 0;
            let new_row = row.map(|_| {
                let new_val = f(GridIndex::new(x, y));
                x += 1;
                new_val
            });
            y += 1;
            new_row
        });
        Self::from_tiles(tiles, start, reward)
    }
    pub fn generate() -> Self {
        let mut tiles = [[Tile::Trap; COLUMNS]; ROWS];
        let mut pos = GridIndex::<ROWS, COLUMNS>::default();
        let mut visited: Vec<GridIndex<ROWS, COLUMNS>> = Default::default();
        for _ in 0..(((ROWS * COLUMNS) as f64)*0.75) as i32 {
            visited.push(pos);
            tiles[pos.y][pos.x] = Tile::Open;
            let mut next_positions = vec![];
            if pos.x > 0 {
                next_positions.push(GridIndex::new(pos.x - 1, pos.y));
            }
            if pos.y > 0 {
                next_positions.push(GridIndex::new(pos.x, pos.y - 1));
            }
            if pos.x < COLUMNS-1 {
                next_positions.push(GridIndex::new(pos.x + 1, pos.y));
            }
            if pos.y < ROWS-1 {
                next_positions.push(GridIndex::new(pos.x, pos.y + 1));
            }
            if next_positions.iter().any(|p| !visited.contains(p)){
                next_positions = next_positions
                    .into_iter()
                    .filter(|p| !visited.contains(p))
                    .collect();
            }
            pos = *next_positions
                .choose(&mut rand::thread_rng())
                .expect("No possible steps");
        }
        let reward = *visited.last().expect("No visited spots");
        tiles[reward.y][reward.x] = Tile::Reward;
        Self::from_tiles(
            tiles,
            Default::default(),
            reward,
        )
    }
    pub fn rows(&self) -> impl Iterator<Item = &[Tile; COLUMNS]> {
        self.grid.rows()
    }
}
