use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub timestep: i32,
}

pub enum CellState {
    Alive,
    Dead,
    Tbd,
}

pub struct CellStates {
    pub states: HashMap<Cell, CellState>, // true = alive, false = dead
}

impl AsRef<Cell> for Cell {
    fn as_ref(&self) -> &Cell {
        self
    }
}

impl CellStates {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    pub fn get(&self, cell: &Cell) -> Option<&CellState> {
        self.states.get(cell)
    }

    pub fn contains(&self, cell: &Cell) -> bool {
        self.states.contains_key(cell)
    }

    pub fn insert_if_empty<T: AsRef<Cell>>(&mut self, cell: T, state: CellState) -> &CellState {
        self.states.entry(cell.as_ref().clone()).or_insert(state)
    }

    pub fn is_alive(&self, cell: &Cell) -> bool {
        matches!(self.states.get(cell), Some(CellState::Alive))
    }
}

impl Default for CellStates {
    fn default() -> Self {
        Self::new()
    }
}
