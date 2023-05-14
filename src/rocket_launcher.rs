use std::f32::consts::PI;

use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};

use crate::camera::GameCamera;
#[derive(Component)]
struct RocketLauncher;

#[derive(Component)]
struct RocketLauncherHolder;

fn spawn_rocket_launcher(
    asset_server: Res<AssetServer>,
    mut ev_rocket_holder_spawns: EventReader<RocketLauncherHolderSpawns>,
    mut commands: Commands,
) {
    for launcher_holder in ev_rocket_holder_spawns.iter() {
        let child = commands
            .spawn((
                RocketLauncher,
                SpriteBundle {
                    texture: asset_server.load("rocket_launcher.png"),
                    sprite: Sprite {
                        anchor: Anchor::Custom(Vec2::new(0.0, 1.0)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0.0, -100.0, 1.0),
                        scale: Vec3::new(1.0, 1.0, 0.1),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Rocket launcher"),
            ))
            .id();

        commands
            .entity(launcher_holder.0)
            .push_children(&[child])
            .insert(RocketLauncherHolder);
    }
}

fn rocket_launcher_follows_mouse(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut rocket_launcher_query: Query<
        &mut Transform,
        (With<RocketLauncher>, Without<RocketLauncherHolder>),
    >,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    rocket_launcher_holder_query: Query<
        &Transform,
        (With<RocketLauncherHolder>, Without<RocketLauncher>),
    >,
) {
    let Ok(primary) = primary_query.get_single() else {
        return;
    };

    let Ok(mut rocket_launcher) = rocket_launcher_query.get_single_mut() else {
        return;
    };

    let (camera, camera_transform) = camera_query.single();

    let Some(world_position) = primary.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    else {
        return;
    };

    let rocket_launcher_holder = rocket_launcher_holder_query.single();

    let delta = Vec2::new(
        world_position.x - rocket_launcher_holder.translation.x,
        world_position.y - rocket_launcher_holder.translation.y,
    );

    let unit_vector = delta.normalize();

    let angle = unit_vector.y.atan2(unit_vector.x);

    rocket_launcher.rotation = Quat::from_rotation_z(angle + PI / 2.0);
}

pub struct RocketLauncherHolderSpawns(pub Entity);
pub struct RocketLauncherPlugin;

impl Plugin for RocketLauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_rocket_launcher)
            .add_system(rocket_launcher_follows_mouse)
            .add_event::<RocketLauncherHolderSpawns>();
    }
}
