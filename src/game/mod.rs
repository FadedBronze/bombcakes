use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;

use crate::AppState;

mod arms;
mod platform;
mod player;
mod rocket_launcher;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum PausedState {
    Paused,
    #[default]
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(platform::PlatformPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(rocket_launcher::RocketLauncherPlugin)
            .add_plugin(arms::ArmsPlugin)
            .add_system(despawn_game.in_schedule(OnExit(AppState::InGame)))
            .add_system(toggle_pause.run_if(in_state(AppState::InGame)))
            .add_state::<PausedState>();
    }
}

#[derive(Component)]
struct GameEntity;

fn despawn_game(all: Query<Entity, With<GameEntity>>, mut commands: Commands) {
    for entity in all.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn toggle_pause(
    mut commands: Commands,
    paused: Res<State<PausedState>>,
    keys: Res<Input<KeyCode>>,
    mut simulation_state: ResMut<RapierConfiguration>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if paused.0 == PausedState::Paused {
            commands.insert_resource(NextState(Some(PausedState::Playing)));
            simulation_state.physics_pipeline_active = true;
        } else {
            commands.insert_resource(NextState(Some(PausedState::Paused)));
            simulation_state.physics_pipeline_active = false;
        }
    }
}
