use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub timestep: i32,
}

pub struct CellStates {
    pub states: HashMap<Cell, bool>, // true = alive, false = dead
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

    pub fn add_living_cell(&mut self, cell: Cell) {
        self.states.insert(cell, true);
    }

    pub fn add_dead_cell(&mut self, cell: Cell) {
        self.states.insert(cell, false);
    }

    /// Adds a dead cell if it doesn't exist, returns whether the cell is alive
    pub fn add_default(&mut self, cell: Cell) -> bool {
        *self.states.entry(cell.clone()).or_insert(false)
    }

    pub fn is_alive(&self, cell: &Cell) -> bool {
        *self.states.get(cell).unwrap_or(&false)
    }
}

impl Default for CellStates {
    fn default() -> Self {
        Self::new()
    }
}
