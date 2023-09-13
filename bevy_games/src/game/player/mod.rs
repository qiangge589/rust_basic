use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;
use super::SimulationState;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
            //  Startup Systems
            // On Enter State
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_system(
                player_movement
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
            )
            .add_system(
                confine_player_movement
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
            )            
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit State
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}