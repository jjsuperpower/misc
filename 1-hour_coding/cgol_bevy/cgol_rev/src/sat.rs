use std::collections::{BTreeSet, HashMap, HashSet};

use varisat::{CnfFormula, ExtendFormula, Lit};

use crate::cell::Cell;

#[derive(Debug, Clone)]
pub enum Formula<'a> {
    Lit(Lit),
    Cell(&'a Cell),
    Not(Box<Formula<'a>>),
    And(Box<Formula<'a>>, Box<Formula<'a>>),
    Or(Box<Formula<'a>>, Box<Formula<'a>>),
    Xor(Box<Formula<'a>>, Box<Formula<'a>>),
    Implies(Box<Formula<'a>>, Box<Formula<'a>>),
    Eq(Box<Formula<'a>>, Box<Formula<'a>>),
    False,
    True,
}

impl Formula<'_> {
    pub fn is_literal(&self) -> bool {
        matches!(self, Formula::Cell(_))
    }
}

impl<'a> AsRef<Formula<'a>> for Formula<'a> {
    fn as_ref(&self) -> &Formula<'a> {
        self
    }
}

// --------------------- From Implementations ---------------------

impl From<Lit> for Formula<'_> {
    fn from(lit: Lit) -> Self {
        Formula::Lit(lit)
    }
}

impl<'a> From<&'a Cell> for Formula<'a> {
    fn from(cell: &'a Cell) -> Self {
        Formula::Cell(cell)
    }
}

impl From<bool> for Formula<'_> {
    fn from(value: bool) -> Self {
        if value {
            Formula::True
        } else {
            Formula::False
        }
    }
}

// --------------------- Right Hand Side Implementations ---------------------

impl<'a> core::ops::Not for Formula<'a> {
    type Output = Formula<'a>;

    fn not(self) -> Self::Output {
        Formula::Not(Box::new(self))
    }
}

impl<'a, T: Into<Formula<'a>>> core::ops::BitAnd<T> for Formula<'a> {
    type Output = Formula<'a>;

    fn bitand(self, rhs: T) -> Self::Output {
        Formula::And(Box::new(self), Box::new(rhs.into()))
    }
}

impl<'a, T: Into<Formula<'a>>> core::ops::BitOr<T> for Formula<'a> {
    type Output = Formula<'a>;

    fn bitor(self, rhs: T) -> Self::Output {
        Formula::Or(Box::new(self), Box::new(rhs.into()))
    }
}

impl<'a, T: Into<Formula<'a>>> core::ops::BitXor<T> for Formula<'a> {
    type Output = Formula<'a>;

    fn bitxor(self, rhs: T) -> Self::Output {
        Formula::Xor(Box::new(self), Box::new(rhs.into()))
    }
}

impl<'a, T: Into<Formula<'a>>> core::ops::Shr<T> for Formula<'a> {
    type Output = Formula<'a>;
    fn shr(self, rhs: T) -> Self::Output {
        Formula::Implies(Box::new(self), Box::new(rhs.into()))
    }
}

impl<'a, T: Into<Formula<'a>>> core::ops::Rem<T> for Formula<'a> {
    type Output = Formula<'a>;

    fn rem(self, rhs: T) -> Self::Output {
        Formula::Eq(Box::new(self), Box::new(rhs.into()))
    }
}

// ----------------- Left Hand Side Implementations for bool -------------------
// For all of these except Eq, there is no point in using bool, however it may be useful as a placeholder.

impl<'a> core::ops::BitAnd<Formula<'a>> for bool {
    type Output = Formula<'a>;

    fn bitand(self, rhs: Formula<'a>) -> Self::Output {
        Formula::And(Box::new(self.into()), Box::new(rhs))
    }
}

impl<'a> core::ops::BitOr<Formula<'a>> for bool {
    type Output = Formula<'a>;

    fn bitor(self, rhs: Formula<'a>) -> Self::Output {
        Formula::Or(Box::new(self.into()), Box::new(rhs))
    }
}

impl<'a> core::ops::BitXor<Formula<'a>> for bool {
    type Output = Formula<'a>;

    fn bitxor(self, rhs: Formula<'a>) -> Self::Output {
        Formula::Xor(Box::new(self.into()), Box::new(rhs))
    }
}

impl<'a> core::ops::Shr<Formula<'a>> for bool {
    type Output = Formula<'a>;
    fn shr(self, rhs: Formula<'a>) -> Self::Output {
        Formula::Implies(Box::new(self.into()), Box::new(rhs))
    }
}

impl<'a> core::ops::Rem<Formula<'a>> for bool {
    type Output = Formula<'a>;

    fn rem(self, rhs: Formula<'a>) -> Self::Output {
        Formula::Eq(Box::new(self.into()), Box::new(rhs))
    }
}

// ------------------- Left Hand Side Implementations for Cell -------------------

impl<'a> core::ops::BitAnd<Formula<'a>> for &'a Cell {
    type Output = Formula<'a>;

    fn bitand(self, rhs: Formula<'a>) -> Self::Output {
        Formula::And(Box::new(self.into()), Box::new(rhs))
    }
}

impl<'a> core::ops::BitOr<Formula<'a>> for &'a Cell {
    type Output = Formula<'a>;

    fn bitor(self, rhs: Formula<'a>) -> Self::Output {
        Formula::Or(Box::new(self.into()), Box::new(rhs))
    }
}

impl<'a> core::ops::BitXor<Formula<'a>> for &'a Cell {
    type Output = Formula<'a>;

    fn bitxor(self, rhs: Formula<'a>) -> Self::Output {
        Formula::Xor(Box::new(self.into()), Box::new(rhs))
    }
}

impl<'a> core::ops::Shr<Formula<'a>> for &'a Cell {
    type Output = Formula<'a>;
    fn shr(self, rhs: Formula<'a>) -> Self::Output {
        Formula::Implies(Box::new(self.into()), Box::new(rhs))
    }
}

impl<'a> core::ops::Rem<Formula<'a>> for &'a Cell {
    type Output = Formula<'a>;

    fn rem(self, rhs: Formula<'a>) -> Self::Output {
        Formula::Eq(Box::new(self.into()), Box::new(rhs))
    }
}

// ------------------- Left Hand Side Implementations for Lit -------------------

impl<'a> core::ops::BitAnd<Formula<'a>> for Lit {
    type Output = Formula<'a>;

    fn bitand(self, rhs: Formula<'a>) -> Self::Output {
        Formula::And(Box::new(self.into()), Box::new(rhs))
    }
}

impl<'a> core::ops::BitOr<Formula<'a>> for Lit {
    type Output = Formula<'a>;

    fn bitor(self, rhs: Formula<'a>) -> Self::Output {
        Formula::Or(Box::new(self.into()), Box::new(rhs))
    }
}

impl<'a> core::ops::BitXor<Formula<'a>> for Lit {
    type Output = Formula<'a>;

    fn bitxor(self, rhs: Formula<'a>) -> Self::Output {
        Formula::Xor(Box::new(self.into()), Box::new(rhs))
    }
}

impl<'a> core::ops::Shr<Formula<'a>> for Lit {
    type Output = Formula<'a>;
    fn shr(self, rhs: Formula<'a>) -> Self::Output {
        Formula::Implies(Box::new(self.into()), Box::new(rhs))
    }
}

impl<'a> core::ops::Rem<Formula<'a>> for Lit {
    type Output = Formula<'a>;

    fn rem(self, rhs: Formula<'a>) -> Self::Output {
        Formula::Eq(Box::new(self.into()), Box::new(rhs))
    }
}

// -------------------- End of Operator Implementations --------------------

pub struct SatInputs {
    cell2lit: HashMap<Cell, Lit>,
    lit2cell: HashMap<Lit, Cell>,
    idx_count: usize,
    formulas: HashSet<BTreeSet<Lit>>,
}

impl SatInputs {
    pub fn new() -> Self {
        let mut sat_inputs = SatInputs {
            cell2lit: HashMap::new(),
            lit2cell: HashMap::new(),
            idx_count: 2,
            formulas: HashSet::new(),
        };

        sat_inputs
            .formulas
            .insert(BTreeSet::from([Lit::from_index(0, false)])); // false literal
        sat_inputs
            .formulas
            .insert(BTreeSet::from([Lit::from_index(1, true)])); // true literal
        sat_inputs
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

    pub fn get_or_add_lit(&mut self, cell: &Cell) -> Lit {
        self.cell2lit
            .get(cell)
            .copied()
            .unwrap_or_else(|| self.add_cell(cell.clone()))
    }

    pub fn get_cell(&self, lit: &Lit) -> Option<&Cell> {
        self.lit2cell.get(lit)
    }

    pub fn tseytin<'a, A>(&mut self, formula: A) -> Lit
    where
        A: AsRef<Formula<'a>>,
    {
        match formula.as_ref() {
            Formula::Lit(lit) => *lit,
            Formula::Cell(cell) => self.get_or_add_lit(cell),
            Formula::Not(f) => {
                let a = self.tseytin(f);
                let y = self.new_lit();
                self.formulas.insert(BTreeSet::from([!a, !y]));
                self.formulas.insert(BTreeSet::from([a, y]));
                y
            }
            Formula::And(f1, f2) => {
                let a = self.tseytin(f1);
                let b = self.tseytin(f2);
                let y = self.new_lit();
                self.formulas.insert(BTreeSet::from([!a, !b, y]));
                self.formulas.insert(BTreeSet::from([a, !y]));
                self.formulas.insert(BTreeSet::from([b, !y]));
                y
            }
            Formula::Or(f1, f2) => {
                let a = self.tseytin(f1);
                let b = self.tseytin(f2);
                let y = self.new_lit();
                self.formulas.insert(BTreeSet::from([a, b, !y]));
                self.formulas.insert(BTreeSet::from([!a, y]));
                self.formulas.insert(BTreeSet::from([!b, y]));
                y
            }
            Formula::Xor(f1, f2) => {
                let a = self.tseytin(f1);
                let b = self.tseytin(f2);
                let y = self.new_lit();
                self.formulas.insert(BTreeSet::from([!a, !b, !y]));
                self.formulas.insert(BTreeSet::from([!a, b, y]));
                self.formulas.insert(BTreeSet::from([a, !b, y]));
                self.formulas.insert(BTreeSet::from([a, b, !y]));
                y
            }
            Formula::Implies(f1, f2) => {
                let a = self.tseytin(f1);
                let b = self.tseytin(f2);
                let y = self.new_lit();
                self.formulas.insert(BTreeSet::from([!a, b, !y]));
                y
            }
            Formula::Eq(f1, f2) => {
                let a = self.tseytin(f1);
                let b = self.tseytin(f2);
                let y = self.new_lit();
                self.formulas.insert(BTreeSet::from([!a, b, !y]));
                self.formulas.insert(BTreeSet::from([a, !b, !y]));
                y
            }
            Formula::True => Lit::from_index(1, true),
            Formula::False => Lit::from_index(0, false),
        }
    }

    pub fn add_formula<'a, A>(&mut self, formula: A)
    where
        A: AsRef<Formula<'a>>,
    {
        let lit = self.tseytin(formula.as_ref());
        self.formulas.insert(BTreeSet::from([lit]));
    }

    pub fn get_cnf_formulas(&self) -> CnfFormula {
        let mut cnf = CnfFormula::new();
        for clause in &self.formulas {
            cnf.add_clause(&clause.iter().cloned().collect::<Vec<Lit>>());
        }
        cnf
    }

    /// Sort two literals
    /// Returns (lesser, greater)
    /// Truth table:
    /// A B | L G
    /// 0 0 | 0 0
    /// 0 1 | 0 1
    /// 1 0 | 0 1
    /// 1 1 | 1 1
    fn sat_sort_2<'a: 'b, 'b: 'a>(
        &self,
        a: Formula<'a>,
        b: Formula<'b>,
    ) -> (Formula<'a>, Formula<'b>) {
        let lesser = a.clone() & b.clone();
        let greater = a.clone() | b.clone();
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
    fn sat_sort_8<'a>(&self, cells: [&'a Cell; 8]) -> [Formula<'a>; 8] {
        // Batcher's odd-even mergesort network for 8 inputs
        // [(0,2),(1,3),(4,6),(5,7)]
        // [(0,4),(1,5),(2,6),(3,7)]
        // [(0,1),(2,3),(4,5),(6,7)]
        // [(2,4),(3,5)]
        // [(1,4),(3,6)]
        // [(1,2),(3,4),(5,6)]

        // Stage 1
        let (x1, x3) = self.sat_sort_2(cells[1].into(), cells[3].into());
        let (x4, x6) = self.sat_sort_2(cells[4].into(), cells[6].into());
        let (x0, x2) = self.sat_sort_2(cells[0].into(), cells[2].into());
        let (x5, x7) = self.sat_sort_2(cells[5].into(), cells[7].into());

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
    ///     
    /// Arguments:
    /// - cells: array of 8 cell references to get literals from
    ///
    /// Returns:
    /// - (Formula for exactly two alive neighbors, Formula for exactly three alive neighbors)
    pub fn cell_constr_neighbors<'a>(&self, cells: [&'a Cell; 8]) -> (Formula<'a>, Formula<'a>) {
        // We only need to check that the literals at index 6 and 7 are true and 5 is false
        // this is because the sorting network will sort the literals such that the true literals are at the end

        let sorted = self.sat_sort_8(cells);
        let exactly_two_true = !sorted[5].clone() & sorted[6].clone() & sorted[7].clone();
        let exactly_three_true =
            !sorted[4].clone() & sorted[5].clone() & sorted[6].clone() & sorted[7].clone();
        (exactly_two_true, exactly_three_true)
    }

    pub fn set_cell_value(&mut self, cell: &Cell, value: bool) {
        let lit = self.get_or_add_lit(cell);
        if value {
            self.formulas.insert(BTreeSet::from([lit]));
        } else {
            self.formulas.insert(BTreeSet::from([!lit]));
        }
    }
}
