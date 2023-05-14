use bevy::prelude::*;
mod background;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod player;
use player::*;

mod platform;
use platform::*;

mod camera;
use camera::*;

mod rocket_launcher;
use rocket_launcher::*;

const BACKGROUND_COLOR: Color = Color::AZURE;

fn main() {
    App::new()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugin(background::BackgroundPlugin)
        .add_plugin(PlatformPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(GameCameraPlugin)
        .add_plugin(RocketLauncherPlugin)
        .run();
}
