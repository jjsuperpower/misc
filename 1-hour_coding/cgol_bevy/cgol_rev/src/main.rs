use cgol_rev::cell::{Cell, CellStates};
use cgol_rev::reverse_gol;
use cgol_rev::sat::SatInputs;
use varisat::Solver;

/// Reverse Conway's Game of Life
///
/// This library provides algorithms to compute possible previous states
/// that could have led to a given Conway's Game of Life pattern.
///
/// Conway's Game of Life is deterministic going forward, but when going
/// backward, there can be multiple valid previous states that could have
/// produced the current pattern.
///

pub fn main() {
    let mut cells: Vec<Cell> = vec![];
    let mut cell_states = CellStates::new();

    for x in -1..=1 {
        for y in -1..=1 {
            let neighbor_cell = Cell { x, y, timestep: 0 };
            cells.push(neighbor_cell.clone());
            if x == 0 && y == 0 {
                cell_states.add_living_cell(neighbor_cell);
            } else {
                cell_states.add_dead_cell(neighbor_cell);
            }
        }
    }

    let mut sat_inputs = SatInputs::new();
    reverse_gol(&mut sat_inputs, &cells, 0);

    let mut solver = Solver::new();
    // solver.write_proof(std::io::stdout(), varisat::ProofFormat::Varisat);

    for (cell, is_alive) in cell_states.states.iter() {
        sat_inputs.set_cell_value(cell, *is_alive);
    }

    solver.add_formula(&sat_inputs.get_cnf_formulas());

    let solution = solver.solve().unwrap();
    assert!(solution);
    let model = solver.model().unwrap();

    for lit in model {
        if lit.is_positive() {
            if let Some(cell) = sat_inputs.get_cell(&lit) {
                println!(
                    "TimeStep: {} Cell({}, {}) is alive",
                    cell.timestep, cell.x, cell.y
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
