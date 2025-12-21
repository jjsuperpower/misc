use std::collections::{HashMap, HashSet};

use slab::Slab;
use varisat::{CnfFormula, ExtendFormula, Lit, Solver, Var};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

struct Cell {
    pub x: i32,
    pub y: i32,
}

struct CellStates {
    pub states: HashMap<Cell, bool>, // true = alive, false = dead
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

    pub fn is_alive(&self, cell: &Cell) -> bool {
        *self.states.get(cell).unwrap_or(&false)
    }
}

struct SatInputs {
    cell2lit: HashMap<Cell, Lit>,
    lit2cell: HashMap<Lit, Cell>,
    idx_count: usize,
    formulas: Vec<CnfFormula>,
}

impl SatInputs {
    fn new() -> Self {
        SatInputs {
            cell2lit: HashMap::new(),
            lit2cell: HashMap::new(),
            idx_count: 0,
            formulas: Vec::new(),
        }
    }

    #[inline]
    fn new_lit(&mut self) -> Lit {
        let lit = Lit::from_index(self.idx_count, true);
        self.idx_count += 1;
        lit
    }

    fn add_cell(&mut self, cell: Cell) -> Lit {
        let lit = self.new_lit();
        self.cell2lit.insert(cell.clone(), lit);
        self.lit2cell.insert(lit, cell.clone());
        lit
    }

    fn get_lit(&self, cell: &Cell) -> Lit {
        *self.cell2lit.get(cell).unwrap()
    }

    fn get_cell(&self, lit: &Lit) -> &Cell {
        self.lit2cell.get(lit).unwrap()
    }

    fn sat_not(&mut self, a: Lit) -> Lit {
        let y = self.new_lit();
        let mut formula = CnfFormula::new();
        formula.add_clause(&[!a, !y]);
        formula.add_clause(&[a, y]);
        self.formulas.push(formula);
        y
    }

    /// Constrain literal a to be true
    fn sat_true(&mut self, a: Lit) {
        let mut formula = CnfFormula::new();
        formula.add_clause(&[a]);
        self.formulas.push(formula);
    }

    fn sat_false(&mut self, a: Lit) {
        let mut formula = CnfFormula::new();
        formula.add_clause(&[!a]);
        self.formulas.push(formula);
    }

    fn sat_and(&mut self, a: Lit, b: Lit) -> Lit {
        let y = self.new_lit();
        let mut formula = CnfFormula::new();
        formula.add_clause(&[!a, !b, y]);
        formula.add_clause(&[a, !y]);
        formula.add_clause(&[b, !y]);
        self.formulas.push(formula);
        y
    }

    fn sat_or(&mut self, a: Lit, b: Lit) -> Lit {
        let y = self.new_lit();
        let mut formula = CnfFormula::new();
        formula.add_clause(&[a, b, !y]);
        formula.add_clause(&[!a, y]);
        formula.add_clause(&[!b, y]);
        self.formulas.push(formula);
        y
    }

    /// A implies B
    /// Truth table:
    /// A B | Valid
    /// 0 0 | 1
    /// 0 1 | 1
    /// 1 0 | 0
    /// 1 1 | 1
    fn sat_implies(&mut self, a: Lit, b: Lit) {
        let mut formula = CnfFormula::new();
        formula.add_clause(&[!a, b]);
        self.formulas.push(formula);
    }

    /// A equivalent to B
    fn sat_equiv(&mut self, a: Lit, b: Lit) {
        let mut formula = CnfFormula::new();
        formula.add_clause(&[!a, b]);
        formula.add_clause(&[a, !b]);
        self.formulas.push(formula);
    }

    /// Sort two literals
    /// Returns (lesser, greater)
    /// Truth table:
    /// A B | L G
    /// 0 0 | 0 0
    /// 0 1 | 0 1
    /// 1 0 | 0 1
    /// 1 1 | 1 1
    fn sat_sort_2(&mut self, a: Lit, b: Lit) -> (Lit, Lit) {
        let lesser = self.sat_and(a, b);
        let greater = self.sat_or(a, b);
        (lesser, greater)
    }

    /// Sort eight literals using a sorting network
    /// Returns sorted array of literals
    ///
    /// Reference: [https://bertdobbelaere.github.io/sorting_networks.html#N8L19D6](https://bertdobbelaere.github.io/sorting_networks.html#N8L19D6)
    ///
    /// Arguments:
    /// - lits: array of 8 literals to sort
    ///
    /// Returns:
    /// - sorted array of 8 literals, from least to greatest
    fn sat_sort_8(&mut self, lits: &[Lit; 8]) -> [Lit; 8] {
        // Batcher's odd-even mergesort network for 8 inputs
        // [(0,2),(1,3),(4,6),(5,7)]
        // [(0,4),(1,5),(2,6),(3,7)]
        // [(0,1),(2,3),(4,5),(6,7)]
        // [(2,4),(3,5)]
        // [(1,4),(3,6)]
        // [(1,2),(3,4),(5,6)]

        // Stage 1
        let (x1, x3) = self.sat_sort_2(lits[1], lits[3]);
        let (x4, x6) = self.sat_sort_2(lits[4], lits[6]);
        let (x0, x2) = self.sat_sort_2(lits[0], lits[2]);
        let (x5, x7) = self.sat_sort_2(lits[5], lits[7]);

        // Stage 2
        let (x3, x7) = self.sat_sort_2(x3, x7);
        let (x2, x6) = self.sat_sort_2(x2, x6);
        let (x1, x5) = self.sat_sort_2(x1, x5);
        let (x0, x4) = self.sat_sort_2(x0, x4);

        // Stage 3
        let (x0, x1) = self.sat_sort_2(x0, x1);
        let (x2, x3) = self.sat_sort_2(x2, x3);
        let (x4, x5) = self.sat_sort_2(x4, x5);
        let (x6, x7) = self.sat_sort_2(x6, x7);

        // Stage 4
        let (x3, x5) = self.sat_sort_2(x3, x5);
        let (x2, x4) = self.sat_sort_2(x2, x4);

        // Stage 5
        let (x3, x6) = self.sat_sort_2(x3, x6);
        let (x1, x4) = self.sat_sort_2(x1, x4);

        // Stage 6
        let (x1, x2) = self.sat_sort_2(x1, x2);
        let (x3, x4) = self.sat_sort_2(x3, x4);
        let (x5, x6) = self.sat_sort_2(x5, x6);

        [x0, x1, x2, x3, x4, x5, x6, x7]
    }

    /// Determine if exactly two literals are true
    ///
    /// This uses a sorting network to sort the literals and then checks if the two last literals are true.
    fn sat_k_eq_2(&mut self, lits: &[Lit; 8]) -> Lit {
        // We only need to check that the literals at index 6 and 7 are true and 5 is false
        // this is because the sorting network will sort the literals such that the true literals are at the end
        let sorted = self.sat_sort_8(lits);
        let two_true = self.sat_and(sorted[6], sorted[7]);
        let five_false = self.sat_not(sorted[5]);
        let result = self.sat_and(two_true, five_false);
        result
    }

    /// Determine if exactly three literals are true
    /// 
    /// This uses a sorting network to sort the literals and then checks if the three last literals are true.
    fn sat_k_eq_3(&mut self, lits: &[Lit; 8]) -> Lit {
        // We only need to check that the literals at index 5, 6 and 7 are true and 4 is false
        // this is because the sorting network will sort the literals such that the true literals are at the end
        let sorted = self.sat_sort_8(lits);
        let three_true = self.sat_and(self.sat_and(sorted   [5], sorted[6]), sorted[7]);
        let four_false = self.sat_not(sorted[4]);
        let result = self.sat_and(three_true, four_false);
        result
}

fn reverse_gol(live_cells: &[Cell], cell_states: HashMap<Cell, bool>) -> () {
    let cell2lit: HashMap<Cell, Lit> = HashMap::new();
    let lit2cell: Slab<Cell> = Slab::new(); // Slab guarantees unique indices

    let formulas = Vec::new();

    // for each living cell restrict its past neighbors to the rules of GOL
    for cell in live_cells {
        let mut clause = Vec::new();

        // Example: If a cell is alive, at least one of its neighbors must have been alive
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue; // Skip the cell itself
                }
                // two or t
            }
        }
    }
}
