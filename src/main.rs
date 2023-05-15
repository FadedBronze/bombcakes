use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

mod background;
mod camera;
mod platform;
mod player;
mod rocket_launcher;

const BACKGROUND_COLOR: Color = Color::AZURE;

fn main() {
    App::new()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugin(background::BackgroundPlugin)
        .add_plugin(platform::PlatformPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(camera::GameCameraPlugin)
        .add_plugin(rocket_launcher::RocketLauncherPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        // .add_startup_system(setup_music)
        .run();
}

// fn setup_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
//     let music = asset_server.load("Bombcakes.mp3");
//     audio.play(music).looped();
// }
