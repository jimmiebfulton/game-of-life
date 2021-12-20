use nalgebra::DMatrix;
use std::mem::swap;

pub struct GameMatrix(DMatrix<CellState>);

pub type Cell = (usize, usize);

pub struct GameOfLife {
    previous: GameMatrix,
    current: GameMatrix,
}

#[derive(Clone, PartialEq, Debug)]
pub enum CellState {
    Alive,
    Dead,
}

impl GameOfLife {
    pub fn new(rows: usize, columns: usize) -> GameOfLife {
        GameOfLife {
            previous: GameMatrix::new(rows, columns),
            current: GameMatrix::new(rows, columns),
        }
    }

    pub fn current(&self) -> &GameMatrix {
        &self.current
    }

    pub fn current_mut(&mut self) -> &mut GameMatrix {
        &mut self.current
    }

    pub fn previous(&self) -> &GameMatrix {
        &self.previous
    }

    pub fn previous_mut(&mut self) -> &mut GameMatrix {
        &mut self.previous
    }

    pub fn tick(&mut self) {
        swap(&mut self.previous, &mut self.current);

        let (rows, columns) = self.shape();

        for row in 0..rows {
            for column in 0..columns {
                let cell = (row, column);
                let new_state = self.previous.get_next_state(cell);
                self.current.set_state(cell, new_state);
            }
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        self.current.0.shape()
    }

    pub fn kill_em_all(&mut self) {
        self.current.kill_em_all();
        self.previous.kill_em_all();
    }
}

impl GameMatrix {
    pub fn new(rows: usize, columns: usize) -> GameMatrix {
        GameMatrix(DMatrix::from_element(rows, columns, CellState::Dead))
    }

    pub fn set_state(&mut self, cell: Cell, state: CellState) {
        self.0[cell] = state
    }

    pub fn get_state(&self, cell: Cell) -> &CellState {
        &self.0[cell]
    }

    pub fn get_next_state(&self, cell: Cell) -> CellState {
        let alive_neighbors = get_neighbor_cells(cell, self.shape())
            .iter()
            .map(|cell| self.get_state(*cell))
            .filter(|state| **state == CellState::Alive)
            .count();

        match self.get_state(cell) {
            CellState::Alive => {
                match alive_neighbors {
                    2..=3 => CellState::Alive,
                    _ => CellState::Dead,
                }
            }
            CellState::Dead => {
                match alive_neighbors {
                    3 => CellState::Alive,
                    _ => CellState::Dead,
                }
            }
        }
    }

    pub fn get_internal(&self) -> &DMatrix<CellState> {
        &self.0
    }


    pub fn shape(&self) -> (usize, usize) {
        self.0.shape()
    }

    pub fn kill_em_all(&mut self) {
        for value in self.0.iter_mut() {
            *value = CellState::Dead
        }
    }
}

fn get_alive_neighbor_count(matrix: &GameMatrix, cell: Cell) -> usize {
    get_neighbor_cells(cell, matrix.shape())
        .iter()
        .map(|cell| matrix.get_state(*cell))
        .filter(|state| **state == CellState::Alive)
        .count()
}

fn get_neighbor_cells(cell: Cell, shape: (usize, usize)) -> Vec<Cell> {
    let mut offsets = vec![];
    let (row_count, column_count) = shape;
    let (row, column) = cell;
    for row_offset in -1..=1 {
        for column_offset in -1..=1 {
            if !(row_offset == 0 && column_offset == 0) {
                offsets.push((
                    get_offset(row, row_offset, row_count),
                    get_offset(column, column_offset, column_count),
                ));
            }
        }
    }

    offsets
}

fn get_offset(position: usize, offset: isize, cells: usize) -> usize {
    (((position as isize + offset) + cells as isize) % cells as isize) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_top_left_neighbor_alive() {
        let game = &mut GameOfLife::new(6, 6);
        game.current_mut().set_state((0, 0), CellState::Alive);
        assert_eq!(get_alive_neighbor_count(game.current_mut(), (1, 1)), 1);

        game.kill_em_all();
        game.current_mut().set_state((0, 1), CellState::Alive);
        assert_eq!(get_alive_neighbor_count(game.current_mut(), (1, 1)), 1);
    }

    #[test]
    fn test_neighbor_cells() {
        let offsets = get_neighbor_cells((0, 0), (10, 10));
        assert_eq!(offsets[0], (9, 9));
        assert_eq!(offsets[1], (9, 0));
        assert_eq!(offsets[2], (9, 1));
        assert_eq!(offsets[3], (0, 9));
        assert_eq!(offsets[4], (0, 1));
        assert_eq!(offsets[5], (1, 9));
        assert_eq!(offsets[6], (1, 0));
        assert_eq!(offsets[7], (1, 1));

        let offsets = get_neighbor_cells((5, 5), (10, 10));
        assert_eq!(offsets[0], (4, 4));
        assert_eq!(offsets[1], (4, 5));
        assert_eq!(offsets[2], (4, 6));
        assert_eq!(offsets[3], (5, 4));
        assert_eq!(offsets[3], (5, 4));
        assert_eq!(offsets[4], (5, 6));
        assert_eq!(offsets[5], (6, 4));
        assert_eq!(offsets[6], (6, 5));
        assert_eq!(offsets[7], (6, 6));
    }

    #[test]
    fn test_get_offset() {
        assert_eq!(get_offset(0, -1, 10), 9);
        assert_eq!(get_offset(0, 1, 10), 1);
    }
}
