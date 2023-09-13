use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;
use super::SimulationState;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 32.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            // .add_startup_system(spawn_stars)
            // Exit
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            // .add_system(tick_star_spawn_timer)
            // .add_system(spawn_star_over_time);
            .add_systems(
                (tick_star_spawn_timer, spawn_star_over_time)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_star.in_schedule(OnExit(AppState::Game)));
    }
}