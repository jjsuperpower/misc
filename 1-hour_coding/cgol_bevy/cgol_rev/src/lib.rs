use std::collections::{HashMap, HashSet};

pub mod cell;
use cell::{Cell, CellStates};
pub mod sat;
use sat::SatInputs;

use crate::{cell::CellState, sat::Formula};

fn constr_prev_gol<'a>(
    sat_inputs: &mut SatInputs,
    curr_cell: &'a Cell,
    past_cell: &'a Cell,
    past_neighbors: &'a [Cell; 8],
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
) -> HashSet<Cell> {
    let loc_prev_cells: HashSet<(i32, i32)> = alive_cells
        .iter()
        .inspect(|c| {
            cell_states.insert_if_empty(c, CellState::Alive);
        })
        .map(|cell| (cell.x, cell.y))
        .collect();
    let mut prev_timestep_cells: HashSet<Cell> = HashSet::new();

    for cell in alive_cells.iter() {
        let neighbor_prev_cells: [Cell; 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|(dx, dy)| {
            let neighbor_cell = Cell {
                x: cell.x + dx,
                y: cell.y + dy,
                timestep: current_timestep - 1,
            };

            if !loc_prev_cells.contains(&(neighbor_cell.x, neighbor_cell.y)) {
                cell_states.insert_if_empty(neighbor_cell.clone(), CellState::Dead);
            }
            neighbor_cell
        })
        .collect::<Vec<Cell>>()
        .try_into()
        .unwrap();

        let past_cell = Cell {
            x: cell.x,
            y: cell.y,
            timestep: current_timestep - 1,
        };

        constr_prev_gol(sat_inputs, cell, &past_cell, &neighbor_prev_cells);

        prev_timestep_cells.insert(past_cell);
        for neighbor in neighbor_prev_cells.iter() {
            prev_timestep_cells.insert(neighbor.clone());
            cell_states.insert_if_empty(neighbor.clone(), CellState::Tbd);
        }
    }

    prev_timestep_cells
}
