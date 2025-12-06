use bevy::platform::cell;
use dashmap::{DashMap, DashSet};
use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};

const CELL_SIZE: f32 = 2.0;
const CELL_INFILL_SIZE: f32 = CELL_SIZE - 1.0;

const CELL_DEAD_SPRITE: LazyLock<Sprite> = LazyLock::new(|| Sprite {
    color: Color::linear_rgba(0.05, 0.05, 0.05, 1.0),
    custom_size: Some(Vec2::splat(CELL_INFILL_SIZE)),
    ..Default::default()
});

const CELL_ALIVE_SPRITE: LazyLock<Sprite> = LazyLock::new(|| Sprite {
    color: Color::linear_rgba(0.9, 0.9, 0.9, 1.0),
    custom_size: Some(Vec2::splat(CELL_INFILL_SIZE)),
    ..Default::default()
});

#[derive(Component, Clone, Copy, Debug)]
// #[derive(on_add = )]
struct Cell {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct CellAlive;

#[derive(Resource)]
struct PruneTimer(Timer);

impl Default for PruneTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

#[derive(Resource)]
struct EntityCountTimer(Timer);

impl Default for EntityCountTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

#[derive(Resource)]
struct GameState {
    paused: bool,
    step_requested: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            paused: true, // Start the game in paused state
            step_requested: false,
        }
    }
}

#[derive(Resource, Default, Debug)]
struct CellAliveNeighbors(pub DashMap<(i32, i32), Mutex<u8>>);

#[derive(Message, Clone, Copy, PartialEq, Eq, Hash)]
struct CellBirth {
    entity: Entity,
}

#[derive(Message, Clone, Copy, PartialEq, Eq, Hash)]
struct CellDeath {
    entity: Entity,
}

impl CellAliveNeighbors {
    #[inline]
    fn increment_neighbor_count(&self, pos: &(i32, i32), inc: i8) {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let neighbor_pos = (pos.0 + dx, pos.1 + dy);
                if let Some(count) = self.0.get(&neighbor_pos) {
                    let mut guard = count.lock().unwrap();
                    let val = (*guard as i8) + inc;
                    let new_val = if val < 0 {
                        warn!(
                            "Neighbor count for cell at {:?} went below zero!",
                            neighbor_pos
                        );
                        0
                    } else {
                        val as u8
                    };
                    trace!(
                        "Updating neighbor count for cell at {:?} from {} to {}",
                        neighbor_pos,
                        *guard,
                        new_val
                    );
                    *guard = new_val;
                } else {
                    warn!(
                        "Neighbor position {:?} not found in CellAliveNeighbors",
                        neighbor_pos
                    );

                    // trace!(
                    //     "Inserting neighbor count for new cell at {:?} with initial value {}",
                    //     neighbor_pos,
                    //     if inc < 0 { 0 } else { inc as u8 }
                    // );
                    // self.0.insert(
                    //     neighbor_pos,
                    //     Mutex::new(if inc < 0 { 0 } else { inc as u8 }),
                    // );
                }
            }
        }
    }

    #[inline]
    fn get_count(&self, pos: &(i32, i32)) -> u8 {
        self.0
            .get(pos)
            .map(|count_mutex| {
                let guard = count_mutex.lock().unwrap();
                *guard
            })
            .unwrap_or_else(|| {
                warn!("No neighbor count found for cell at {:?}", pos);
                0
            })
    }
}

#[derive(Resource, Default, Debug)]
struct CellLocs(
    // Map from cell position to entity
    pub DashMap<(i32, i32), Entity>,
);

impl CellLocs {
    #[inline]
    fn add(&self, cell: &(i32, i32), entity: Entity) {
        // panic if cell already exists
        if self.0.contains_key(cell) {
            warn!("Cell at {:?} already exists!", cell);
        }

        // trace!("Adding cell at {:?} with entity {:?}", cell, entity);
        self.0.insert(*cell, entity);
    }

    #[inline]
    fn remove(&self, pos: &(i32, i32)) {
        self.0.remove(pos);
    }

    #[inline]
    fn get_neighbor_cells(&self, pos: &(i32, i32)) -> (Vec<Entity>, Vec<(i32, i32)>) {
        // trace!("Getting neighbors for {:?}", pos);
        let mut neighbors = Vec::with_capacity(8);
        let mut vacancies = Vec::with_capacity(8);
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if let Some(entity) = self.0.get(&(pos.0 + dx, pos.1 + dy)) {
                    neighbors.push(*entity);
                } else {
                    vacancies.push((pos.0 + dx, pos.1 + dy));
                }
            }
        }
        (neighbors, vacancies)
    }
}

fn spawn_cell(
    commands: &mut Commands,
    cell_locs: &CellLocs,
    alive_neighbors: &CellAliveNeighbors,
    pos: (i32, i32),
    is_dead_cell: bool,
) -> Entity {
    trace!(
        "Spawning {} cell at {:?}",
        if is_dead_cell { "edge" } else { "alive" },
        pos
    );

    let cell = Cell { x: pos.0, y: pos.1 };
    let mut entity = {
        commands.spawn((
            cell.clone(),
            Transform::from_translation(Vec3::new(
                pos.0 as f32 * CELL_SIZE,
                pos.1 as f32 * CELL_SIZE,
                0.0,
            )),
        ))
    };

    let entity_id = entity.id();
    cell_locs.add(&pos, entity_id);
    alive_neighbors.0.insert(pos, Mutex::new(0));

    if is_dead_cell {
        entity.insert(CELL_DEAD_SPRITE.clone());
    } else {
        entity.insert(CellAlive);
        entity.insert(CELL_ALIVE_SPRITE.clone());
    }

    entity_id
}

fn init_edge_cells(
    mut commands: Commands,
    cell_locs: &CellLocs,
    alive_neighbors: &CellAliveNeighbors,
    positions: impl Iterator<Item = (i32, i32)>,
) {
    debug!("Initializing edge cells...");
    // spawn edge cells around alive cells
    for pos in positions {
        let (_neighbors, vacancies) = cell_locs.get_neighbor_cells(&pos);

        for vacancy in vacancies {
            trace!("Padding with edge cell at {:?}", vacancy);
            spawn_cell(&mut commands, &cell_locs, &alive_neighbors, vacancy, true);
        }
    }
}

fn init_neighbor_counts(
    alive_neighbors: &CellAliveNeighbors,
    positions: impl Iterator<Item = (i32, i32)>,
) {
    debug!("Initializing neighbor counts...");
    for pos in positions {
        trace!(
            "Incrementing neighbor counts for cell at ({}, {})",
            pos.0,
            pos.1
        );
        alive_neighbors.increment_neighbor_count(&pos, 1);
    }
}

fn update_on_cell_death(
    mut death_message: MessageReader<CellDeath>,
    par_commands: ParallelCommands,
    query: Query<&Cell>,
    alive_neighbors: Res<CellAliveNeighbors>,
) {
    death_message.read().for_each(|death_event| {
        debug!("Updating on cell death for entity {:?}", death_event.entity);
        let entity = death_event.entity;
        let cell = query.get(entity).unwrap();
        let cell_pos = (cell.x, cell.y);

        // decrement neighbor counts
        alive_neighbors.increment_neighbor_count(&cell_pos, -1);

        par_commands.command_scope(|mut commands| {
            commands.entity(death_event.entity).remove::<CellAlive>();

            // update the cell sprite to dead
            commands.entity(entity).insert(CELL_DEAD_SPRITE.clone());
        });
    });
}

fn update_on_cell_birth(
    mut birth_message: MessageReader<CellBirth>,
    par_commands: ParallelCommands,
    query: Query<&Cell>,
    alive_neighbors: Res<CellAliveNeighbors>,
    cell_locs: Res<CellLocs>,
) {
    birth_message.read().for_each(|birth_event| {
        debug!("Updating on cell birth for entity {:?}", birth_event.entity);
        let entity = birth_event.entity;
        let cell = query.get(entity).unwrap();
        let cell_pos = (cell.x, cell.y);

        let (_, vacancies) = cell_locs.get_neighbor_cells(&cell_pos);

        // bring cell to life
        par_commands.command_scope(|mut commands| {
            commands.entity(entity).insert(CellAlive);

            // update the cell sprite to alive
            commands.entity(entity).insert(CELL_ALIVE_SPRITE.clone());

            // check if there are any cell vacancies around this cell, and spawn dead cells there
            for vacancy in vacancies {
                debug!("Padding with dead cell at {:?}", vacancy);
                spawn_cell(&mut commands, &cell_locs, &alive_neighbors, vacancy, true);
            }
        });

        // increment neighbor counts
        alive_neighbors.increment_neighbor_count(&cell_pos, 1);
    });
}

#[derive(Component)]
struct GameStateUI;

/// Apply the Game of Life rules to determine which cells live, die, or are born
/// - Any live cell with two or three live neighbours survives.
/// - Any dead cell with three live neighbours becomes a live cell.
/// - All other live cells die in the next generation. Similarly, all other dead cells stay dead.
fn game_rules(
    mut birth_message: MessageWriter<CellBirth>,
    mut death_message: MessageWriter<CellDeath>,
    mut game_state: ResMut<GameState>,
    alive_neighbors: Res<CellAliveNeighbors>,
    alive_query: Query<(Entity, &Cell), With<CellAlive>>,
    edge_query: Query<(Entity, &Cell), Without<CellAlive>>,
) {
    // Check if game is paused and no step was requested
    if game_state.paused && !game_state.step_requested {
        return;
    }

    // Reset step request if it was used
    if game_state.step_requested {
        game_state.step_requested = false;
        info!("Stepping one frame...");
    }

    debug!("Applying game rules...");

    let cells_to_birth = DashSet::new();
    let cells_to_kill = DashSet::new();

    alive_query.par_iter().for_each(|(entity, &cell)| {
        let loc = (cell.x, cell.y);
        let neighbor_count = alive_neighbors.get_count(&loc);

        if neighbor_count < 2 || neighbor_count > 3 {
            // Cell dies
            cells_to_kill.insert(CellDeath { entity });
        }
    });

    edge_query.par_iter().for_each(|(entity, &cell)| {
        let loc = (cell.x, cell.y);
        let neighbor_count = alive_neighbors.get_count(&loc);

        if neighbor_count == 3 {
            // Cell becomes alive
            cells_to_birth.insert(CellBirth { entity });
        }
    });

    birth_message.write_batch(cells_to_birth.into_iter());
    death_message.write_batch(cells_to_kill.into_iter());
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Spawn UI text
    commands.spawn((
        Text::new("SPACE: Pause/Resume | ENTER: Step (when paused)"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        GameStateUI,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

// Update UI to show current game state
fn update_ui(game_state: Res<GameState>, mut ui_query: Query<&mut Text, With<GameStateUI>>) {
    if game_state.is_changed() {
        for mut text in &mut ui_query {
            let status = if game_state.paused {
                "PAUSED"
            } else {
                "RUNNING"
            };
            text.0 = format!(
                "SPACE: Pause/Resume | ENTER: Step (when paused) | Status: {}",
                status
            );
        }
    }
}

fn spawn_cells(
    mut commands: Commands,
    mut cell_locs: ResMut<CellLocs>,
    mut alive_neighbors: ResMut<CellAliveNeighbors>,
) {
    // single cell
    // let pattern = [(0, 0)];

    // create spinner/blinker for testing
    // let pattern = [(-1, 0), (0, 0), (1, 0)];

    // block pattern
    // let pattern = [(0, 0), (1, 0), (0, 1), (1, 1)];

    // honeycomb pattern
    // let pattern = [(0, 0), (-1, -1), (1, -1), (-1, -2), (1, -2), (0, -3)];

    // glider pattern
    // let pattern = [(0, 0), (1, 0), (2, 0), (0, 1), (1, 2)];

    // acorn pattern
    let pattern = [(0, 0), (1, 0), (1, 2), (3, 1), (4, 0), (5, 0), (6, 0)];

    // weird edge case
    // let pattern = [
    //     (0, 0),
    //     (0, 1),
    //     (1, 0),
    //     (2, 0),
    //     (1, 2),
    //     (3, 1),
    //     (3, 3),
    //     (2, 3),
    //     (4, 2),
    // ];

    let pattern = HashSet::from(pattern);
    for pos in pattern.iter() {
        spawn_cell(
            &mut commands,
            &mut cell_locs,
            &mut alive_neighbors,
            *pos,
            false,
        );
    }

    init_edge_cells(
        commands,
        &cell_locs,
        &alive_neighbors,
        pattern.iter().copied(),
    );
    init_neighbor_counts(&alive_neighbors, pattern.iter().copied());
}

fn prune_dead_cells(
    time: Res<Time>,
    mut timer: ResMut<PruneTimer>,
    par_commands: ParallelCommands,
    alive_neighbors: Res<CellAliveNeighbors>,
    cell_locs: Res<CellLocs>,
    dead_cells: Query<(Entity, &Cell), Without<CellAlive>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        debug!("Pruning dead cells");
        dead_cells.par_iter().for_each(|(entity, cell)| {
            if alive_neighbors.get_count(&(cell.x, cell.y)) == 0 {
                trace!("Pruning dead cell at {:?}", (cell.x, cell.y));

                cell_locs.remove(&(cell.x, cell.y));
                alive_neighbors.0.remove(&(cell.x, cell.y));

                // Despawn the entity - the observer will handle CellLocs cleanup
                par_commands.command_scope(|mut commands| {
                    commands.entity(entity).despawn();
                });
            }
        });
    }
}

fn handle_input(mut game_state: ResMut<GameState>, input: Res<ButtonInput<KeyCode>>) {
    // Toggle pause with spacebar
    if input.just_pressed(KeyCode::Space) {
        game_state.paused = !game_state.paused;
        info!(
            "Game {}",
            if game_state.paused {
                "paused"
            } else {
                "resumed"
            }
        );
    }

    // Step one frame when paused (Enter key)
    if input.just_pressed(KeyCode::Enter) && game_state.paused {
        game_state.step_requested = true;
        info!("Step requested");
    }
}

fn print_entity_count(
    time: Res<Time>,
    mut timer: ResMut<EntityCountTimer>,
    all_entities: Query<Entity>,
    alive_cells: Query<Entity, With<CellAlive>>,
    dead_cells: Query<Entity, (With<Cell>, Without<CellAlive>)>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let total_entities = all_entities.iter().count();
        let alive_count = alive_cells.iter().count();
        let dead_count = dead_cells.iter().count();

        info!(
            "Entity counts - Total: {}, Alive: {}, Dead: {}",
            total_entities, alive_count, dead_count
        );
    }
}

fn main() {
    info!("Starting Game of Life with Bevy");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::Immediate, // Disables VSync
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .init_resource::<CellLocs>()
        .init_resource::<CellAliveNeighbors>()
        .init_resource::<PruneTimer>()
        .init_resource::<EntityCountTimer>()
        .init_resource::<GameState>()
        .add_systems(Startup, (setup_system, spawn_cells).chain())
        .add_systems(Update, (handle_input, update_ui, print_entity_count))
        .add_systems(
            Update,
            (
                game_rules,
                update_on_cell_death,
                update_on_cell_birth,
                prune_dead_cells,
            )
                .chain(),
        )
        .add_message::<CellBirth>()
        .add_message::<CellDeath>()
        // .add_systems(PostUpdate, fix_edge_cells) // Temporarily disabled for testing
        .run();
}
