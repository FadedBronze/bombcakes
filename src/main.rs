use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use game_audio::GameAudioPlugin;

mod background;
mod camera;
mod game;
mod game_audio;
mod main_menu;
mod settings_menu;
mod utils;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SettingsState {
    Open,
    #[default]
    Closed,
}

fn main() {
    App::new()
        //Default plugins
        .add_plugins(DefaultPlugins)
        //Rapier
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(setup_rapier)
        //World inspector
        .add_plugin(WorldInspectorPlugin::new())
        //Audio
        .add_plugin(AudioPlugin)
        .add_plugin(GameAudioPlugin)
        //App state
        .add_state::<AppState>()
        .add_state::<SettingsState>()
        //Game stuff
        .add_plugin(game::GamePlugin)
        .add_plugin(camera::GameCameraPlugin)
        .add_plugin(main_menu::MenuPlugin)
        .add_plugin(background::BackgroundPlugin)
        .add_plugin(settings_menu::SettingsPlugin)
        //run
        .run();
}

fn setup_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.timestep_mode = TimestepMode::Variable {
        max_dt: 1.0 / 30.0,
        time_scale: 1.0,
        substeps: 1,
    }
}
