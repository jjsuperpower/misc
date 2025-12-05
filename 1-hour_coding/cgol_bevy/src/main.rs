use bevy::platform::cell;
use dashmap::{DashMap, DashSet};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

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

#[derive(Component)]
// #[derive(on_add = )]
struct Cell {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct CellAlive;

#[derive(Component)]
struct EdgeCell; // Cells that are dead but adjacent to alive cells

#[derive(Resource)]
struct PruneTimer(Timer);

impl Default for PruneTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

#[derive(Resource, Default)]
struct CellLocs {
    loc_to_entity: DashMap<(i32, i32), Entity>,
}

impl CellLocs {
    fn add(&self, cell: &(i32, i32), entity: Entity) {
        // panic if cell already exists
        if self.loc_to_entity.contains_key(cell) {
            warn!("Cell at {:?} already exists!", cell);
        }

        // trace!("Adding cell at {:?} with entity {:?}", cell, entity);
        self.loc_to_entity.insert(*cell, entity);
    }

    fn remove(&self, pos: &(i32, i32)) {
        self.loc_to_entity.remove(pos);
    }

    fn get_neighbors(&self, pos: &(i32, i32)) -> (Vec<Entity>, Vec<(i32, i32)>) {
        // trace!("Getting neighbors for {:?}", pos);
        let mut neighbors = Vec::with_capacity(8);
        let mut vacancies = Vec::with_capacity(8);
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if let Some(entity) = self.loc_to_entity.get(&(pos.0 + dx, pos.1 + dy)) {
                    neighbors.push(*entity);
                } else {
                    vacancies.push((pos.0 + dx, pos.1 + dy));
                }
            }
        }
        (neighbors, vacancies)
    }
}

fn add_cell_to_locs(add: On<Add, Cell>, query: Query<&Cell>, cell_locs: Res<CellLocs>) {
    debug!("add_cell_to_locs");
    let cell = query.get(add.entity).unwrap();
    cell_locs.add(&(cell.x, cell.y), add.entity);
}

fn remove_cell_from_locs(remove: On<Remove, Cell>, query: Query<&Cell>, cell_locs: Res<CellLocs>) {
    if let Ok(cell) = query.get(remove.entity) {
        cell_locs.remove(&(cell.x, cell.y));
    }
}

fn spawn_cell(commands: &mut Commands, pos: (i32, i32), is_edge_cell: bool) -> Entity {
    debug!("Spawning cell at {:?}", pos);
    let mut entity = commands.spawn((
        Cell { x: pos.0, y: pos.1 },
        Transform::from_translation(Vec3::new(
            pos.0 as f32 * CELL_SIZE,
            pos.1 as f32 * CELL_SIZE,
            0.0,
        )),
    ));
    if is_edge_cell {
        entity.insert(EdgeCell);
    } else {
        entity.insert(CellAlive);
    }
    entity.id()
}

fn update_cell_sprites(
    mut commands: Commands,
    // Only update when CellAlive component is added or removed
    born_query: Query<Entity, (With<Cell>, Added<CellAlive>)>,
    mut died: RemovedComponents<CellAlive>,
    entities: Query<(), With<Cell>>, // Check if entity still exists
) {
    debug!("update_cell_sprites");
    for entity in born_query.iter() {
        commands.entity(entity).insert(CELL_ALIVE_SPRITE.clone());
    }

    for entity in died.read() {
        // Only update sprite if the entity still exists
        if entities.contains(entity) {
            commands.entity(entity).insert(CELL_DEAD_SPRITE.clone());
        }
    }
}

/// Update all dead cells that are adjacent to alive cells
fn update_edge_cells(
    mut commands: Commands,
    cell_locs: Res<CellLocs>,
    alive_query: Query<&Cell, With<CellAlive>>,
    edge_query: Query<Entity, (With<Cell>, With<EdgeCell>)>,
) {
    debug!("update_edge_cells");

    // Remove EdgeCell component from all edge cells
    edge_query.iter().for_each(|entity| {
        commands.entity(entity).remove::<EdgeCell>();
    });

    let cells_to_spawn = DashSet::new();
    let new_edge_cells = DashSet::new();

    // add EdgeCell component to all dead cells adjacent to alive cells
    alive_query.iter().for_each(|cell| {
        let (neighbors, vacancies) = cell_locs.get_neighbors(&(cell.x, cell.y));
        // if no cell exists, create it
        for vacancy in vacancies {
            // check if we've already spawned a cell at this location
            // the query will not update until the next frame, so we need to track spawned cells ourselves
            cells_to_spawn.insert(vacancy);
        }
        // mark existing dead neighbors as edge cells
        for neighbor in neighbors {
            if !alive_query.get(neighbor).is_ok() {
                new_edge_cells.insert(neighbor);
            }
        }
    });

    for vacancy in cells_to_spawn.iter() {
        spawn_cell(&mut commands, *vacancy, true);
    }

    for entity in new_edge_cells.iter() {
        commands.entity(*entity).insert(EdgeCell);
    }
}

fn game_rules(
    mut commands: Commands,
    cell_locs: Res<CellLocs>,
    alive_query: Query<(Entity, &Cell), With<CellAlive>>,
    edge_query: Query<(Entity, &Cell), (Without<CellAlive>, With<EdgeCell>)>,
) {
    debug!("game_rules");
    // check how many neighbors are alive
    let get_num_alive_neighbors = |cell: &Cell| {
        cell_locs
            .get_neighbors(&(cell.x, cell.y))
            .0
            .iter()
            .filter(|e| alive_query.get(**e).is_ok())
            .count()
    };

    // Get a list of cells to spawn, but don't spawn them yet
    // cells with exactly 3 alive neighbors become alive
    let cells_to_spawn: DashSet<Entity> = DashSet::new();
    edge_query.iter().for_each(|(entity, cell)| {
        let alive_neighbors = get_num_alive_neighbors(cell);
        if alive_neighbors == 3 {
            trace!("Found cell {} @ {:?} to birth", entity, (cell.x, cell.y));
            cells_to_spawn.insert(entity);
        }
    });

    // kill cells with fewer than 2 or more than 3 alive neighbors
    let cells_to_kill: DashSet<Entity> = DashSet::new();
    alive_query.iter().for_each(|(entity, cell)| {
        let alive_neighbors = get_num_alive_neighbors(cell);
        if (alive_neighbors < 2) || (alive_neighbors > 3) {
            trace!("Found cell {} @ {:?} to kill", entity, (cell.x, cell.y));
            cells_to_kill.insert(entity);
        }
    });

    debug!("Birthing {} cells", cells_to_spawn.len());
    debug!("Killing {} cells", cells_to_kill.len());

    // Spawn new alive cells
    for entity in cells_to_spawn {
        commands
            .entity(entity)
            .insert(CellAlive)
            .remove::<EdgeCell>();
    }
    // Kill cells
    for entity in cells_to_kill {
        commands.entity(entity).remove::<CellAlive>();
    }
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_cells(mut commands: Commands) {
    // // create spinner for testing
    // spawn_cell(&mut commands, (0, 0), false);
    // spawn_cell(&mut commands, (1, 0), false);
    // spawn_cell(&mut commands, (-1, 0), false);

    // acorn pattern
    spawn_cell(&mut commands, (0, 0), false);
    spawn_cell(&mut commands, (1, 0), false);
    spawn_cell(&mut commands, (1, 2), false);
    spawn_cell(&mut commands, (3, 1), false);
    spawn_cell(&mut commands, (4, 0), false);
    spawn_cell(&mut commands, (5, 0), false);
    spawn_cell(&mut commands, (6, 0), false);
}

fn prune_dead_cells(
    time: Res<Time>,
    mut timer: ResMut<PruneTimer>,
    mut commands: Commands,
    dead_cells: Query<(Entity, &Cell), (Without<CellAlive>, Without<EdgeCell>)>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let mut pruned_count = 0;

        for (entity, _cell) in dead_cells.iter() {
            // Despawn the entity - the observer will handle CellLocs cleanup
            commands.entity(entity).despawn();
            pruned_count += 1;
        }

        if pruned_count > 0 {
            info!("Pruned {} dead cells", pruned_count);
        }
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
        .init_resource::<PruneTimer>()
        .add_systems(Startup, setup_system)
        .add_systems(Startup, spawn_cells)
        .add_systems(
            Update,
            ((update_edge_cells, game_rules).chain(), update_cell_sprites).before(prune_dead_cells),
        )
        .add_systems(Update, prune_dead_cells)
        .add_observer(add_cell_to_locs)
        .add_observer(remove_cell_from_locs)
        .run();
}
