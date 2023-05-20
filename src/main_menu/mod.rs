use bevy::prelude::*;

use crate::AppState;

pub struct MenuPlugin;

fn create_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn despawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn switch_into_menu(
    keys: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if app_state.0 == AppState::Menu {
            commands.insert_resource(NextState(Some(AppState::InGame)));
        } else {
            commands.insert_resource(NextState(Some(AppState::Menu)));
        }
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(switch_into_menu)
            .add_system(create_main_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::Menu)));
    }
}
