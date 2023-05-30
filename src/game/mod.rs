use bevy::prelude::*;

use crate::AppState;

mod arms;
mod pause_menu;
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
        app.add_state::<PausedState>()
            .add_plugin(platform::PlatformPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(rocket_launcher::RocketLauncherPlugin)
            .add_plugin(arms::ArmsPlugin)
            .add_plugin(pause_menu::PauseMenuPlugin)
            .add_system(despawn_game.in_schedule(OnExit(AppState::InGame)));
    }
}

#[derive(Component)]
struct GameEntity;

fn despawn_game(all: Query<Entity, With<GameEntity>>, mut commands: Commands) {
    for entity in all.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
