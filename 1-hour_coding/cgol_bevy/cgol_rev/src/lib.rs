use std::collections::{HashMap, HashSet};

pub mod cell;
use cell::{Cell, CellStates};
pub mod sat;
use sat::SatInputs;

use crate::sat::Formula;

fn constr_prev_gol<'a>(
    sat_inputs: &mut SatInputs,
    curr_cell: &'a Cell,
    past_cell: &'a Cell,
    past_neighbors: [&'a Cell; 8],
) {
    // enforce rules for each cell in the previous timestep
    //
    // if the cell is alive in the current timestep,
    // - It was alive with 2 or 3 alive neighbors in the previous timestep, or
    // - It was dead with exactly 3 alive neighbors in the previous timestep
    //
    // if the cell is dead in the current timestep,
    // - It was alive with fewer than 2 or more than 3 alive neighbors in the previous timestep, or
    // - It was dead with any number of alive neighbors other than exactly 3 in the previous timestep
    let (two_alive, three_alive) = sat_inputs.cell_constr_neighbors(past_neighbors);

    let past_cell = Formula::from(past_cell);
    let curr_cell = Formula::from(curr_cell);

    let current_alive = Formula::from(curr_cell.clone())
        >> ((Formula::from(past_cell.clone()) & (three_alive.clone() | two_alive.clone()))
            | (!Formula::from(past_cell.clone()) & three_alive.clone()));

    let current_dead = !Formula::from(curr_cell.clone())
        >> ((!Formula::from(past_cell.clone()) & (!three_alive.clone()))
            | (Formula::from(past_cell.clone()) & (!three_alive & !two_alive)));

    sat_inputs.add_formula(current_alive);
    sat_inputs.add_formula(current_dead);
}

pub fn reverse_gol(
    sat_inputs: &mut SatInputs,
    alive_cells: &[Cell],
    cell_states: &mut CellStates,
    current_timestep: i32,
) -> () {
    let mut curr_timestep_cells = HashMap::new();

    for cell in alive_cells {
        for dx in -1..=1 {
            for dy in -1..=1 {
                let cell = Cell {
                    x: cell.x + dx,
                    y: cell.y + dy,
                    timestep: current_timestep,
                };
                cell_states.add_default(cell.clone());
                curr_timestep_cells.insert((cell.x, cell.y), cell);
            }
        }
    }

    let prev_timestep_cells: HashSet<Cell> = cell_states
        .states
        .keys()
        .filter(|c| c.timestep == current_timestep - 1)
        .cloned()
        .collect();

    for ((_x, _y), cell) in curr_timestep_cells.iter() {
        let mut neighbor_prev_cells = vec![];
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let neighbor_cell = Cell {
                    x: cell.x + dx,
                    y: cell.y + dy,
                    timestep: current_timestep - 1,
                };
                if prev_timestep_cells.contains(&neighbor_cell) {
                    neighbor_prev_cells.push(neighbor_cell);
                } else {
                    // cell_states.add_dead_cell(neighbor_cell.clone());
                    neighbor_prev_cells.push(neighbor_cell);
                }
            }
        }

        let past_cell = Cell {
            x: cell.x,
            y: cell.y,
            timestep: current_timestep - 1,
        };
        constr_prev_gol(
            sat_inputs,
            cell,
            &past_cell,
            [
                &neighbor_prev_cells[0],
                &neighbor_prev_cells[1],
                &neighbor_prev_cells[2],
                &neighbor_prev_cells[3],
                &neighbor_prev_cells[4],
                &neighbor_prev_cells[5],
                &neighbor_prev_cells[6],
                &neighbor_prev_cells[7],
            ],
        );
    }
}
