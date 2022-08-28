use std::{cmp::Ordering, task::Context};

pub trait ReinforcementState<T> {
    type Action;
    type Actions: Iterator<Item = Self::Action>;
    type Context;

    fn evaluate(&self, action: &Self::Action, context: &Self::Context) -> T;
    fn actions(&self, context: &Self::Context) -> Self::Actions;

    fn take_action(self, action: Self::Action, context: &Self::Context) -> Self;

    fn best_action(&self, context: &Self::Context) -> Option<Self::Action>
    where
        T: PartialOrd,
    {
        let actions = self.actions(context);
        actions.max_by_key(|action| CmpOrEq(self.evaluate(&action, context)))
    }
    fn take_best_action(self, context: &Self::Context) -> Option<Self>
    where
        Self: Sized,
        T: PartialOrd,
    {
        let best_action = self.best_action(context)?;
        Some(self.take_action(best_action, context))
    }
}

#[repr(transparent)]
#[derive(PartialOrd)]
struct CmpOrEq<T>(T);
impl<T: PartialOrd> PartialEq for CmpOrEq<T> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).map(|c| c.is_eq()).unwrap_or(false)
    }
}
impl<T: PartialOrd> Eq for CmpOrEq<T> {}
impl<T: PartialOrd> Ord for CmpOrEq<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Less)
    }
}

#[cfg(test)]
mod tests {
    use super::ReinforcementState;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct GridPos {
        x: usize,
        y: usize,
    }

    struct Grid<T> {
        data: [[T; 10]; 10],
    }
    impl<T> Grid<T> {
        fn from_fn<F: FnMut(GridPos) -> T>(mut f: F) -> Self {
            let mut row_index = 0;
            let data = [[(); 10]; 10].map(|row| {
                let mut col_index = 0;
                let new_row = row.map(|_| {
                    let new_val = f(GridPos {
                        x: col_index,
                        y: row_index,
                    });
                    col_index += 1;
                    new_val
                });
                row_index += 1;
                new_row
            });
            Grid { data }
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Direction {
        Up,
        Right,
        Down,
        Left,
    }
    // impl Direction {
    //     fn clockwise(self) -> Self {
    //         match self {
    //             Direction::Up => Self::Right,
    //             Direction::Right => Self::Down,
    //             Direction::Down => Self::Left,
    //             Direction::Left => Self::Up,
    //         }
    //     }
    //     fn counter_clockwise(self) -> Self {
    //         match self {
    //             Direction::Right => Self::Up,
    //             Direction::Down => Self::Right,
    //             Direction::Left => Self::Down,
    //             Direction::Up => Self::Left,
    //         }
    //     }
    // }

    impl<T: Clone> ReinforcementState<T> for GridPos {
        type Action = Direction;

        type Actions = std::vec::IntoIter<Direction>;

        type Context = Grid<T>;

        fn evaluate(&self, action: &Self::Action, context: &Self::Context) -> T {
            let new_pos = self.clone().take_action(*action, context);
            context.data[new_pos.y][new_pos.x].clone()
        }

        fn actions(&self, _context: &Self::Context) -> Self::Actions {
            [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ]
            .into_iter()
            .filter(|d| match d {
                Direction::Up => self.y > 0,
                Direction::Right => self.x < 10,
                Direction::Down => self.y < 10,
                Direction::Left => self.x > 0,
            })
            .collect::<Vec<_>>()
            .into_iter()
        }

        fn take_action(self, action: Self::Action, _context: &Self::Context) -> Self {
            let mut new_pos = self.clone();
            match action {
                Direction::Up => new_pos.y -= 1,
                Direction::Right => new_pos.x += 1,
                Direction::Down => new_pos.y += 1,
                Direction::Left => new_pos.x -= 1,
            }
            new_pos
        }
    }

    #[test]
    fn test() {
        let grid = Grid::from_fn(|GridPos { x, y }| if x == y && x == 4 { 2.0 } else { 1.0 });
        let pos = GridPos { x: 3, y: 4 };
        assert_eq!(Some(Direction::Right), pos.best_action(&grid));
        assert_eq!(Some(GridPos { x: 4, y: 4 }), pos.take_best_action(&grid));
    }
}
