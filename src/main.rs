use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

mod background;
mod camera;
mod game;
mod main_menu;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        //Default plugins
        .add_plugins(DefaultPlugins)
        //Rapier
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        //World inspector
        .add_plugin(WorldInspectorPlugin::new())
        //Audio
        .add_plugin(AudioPlugin)
        .add_startup_system(setup_music)
        //App state
        .add_state::<AppState>()
        //Game stuff
        .add_plugin(game::GamePlugin)
        .add_plugin(camera::GameCameraPlugin)
        .add_plugin(main_menu::MenuPlugin)
        .add_plugin(background::BackgroundPlugin)
        //run
        .run();
}

fn setup_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("Bombcakes.mp3");
    audio.play(music).loop_from(11.0);
}
