use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::PlayerLandedOnEvent;

#[derive(Component, Reflect)]
pub struct Platform;

fn create_starting_platform(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(create_platform(
        &asset_server,
        Transform::from_xyz(0.0, -150.0, 1.0),
    ));
}

fn create_platforms(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    platforms: Query<&Transform, With<Platform>>,
) {
    let platform_count = platforms.iter().len();

    if platform_count == 1 {
        let last_platform = platforms.single();

        let random_dir = -1.0;

        commands.spawn(create_platform(
            &asset_server,
            Transform::from_xyz(
                last_platform.translation.x + 450.0 * random_dir,
                last_platform.translation.y + 80.0,
                1.0,
            ),
        ));
    }
}

fn create_platform(
    asset_server: &AssetServer,
    transform: Transform,
) -> (SpriteBundle, Platform, RigidBody, Collider, Name) {
    (
        SpriteBundle {
            texture: asset_server.load("platform.png"),
            transform,
            ..default()
        },
        Platform,
        RigidBody::Fixed,
        Collider::cuboid(154.0, 38.0),
        Name::new("Platform"),
    )
}

//TODO: learn about the line renderer and code the gun

fn delete_platform(
    platforms: Query<(Entity, &Transform), With<Platform>>,
    mut ev_player_land: EventReader<PlayerLandedOnEvent>,
    mut commands: Commands,
) {
    if platforms.iter().len() == 1 {
        return;
    }

    for land_event in ev_player_land.iter() {
        let mut lowest_platform: Option<(Entity, Transform)> = None;
        let mut lowest_platform_y = f32::INFINITY;
        for (platform, transform) in platforms.iter() {
            if transform.translation.y < lowest_platform_y {
                lowest_platform_y = transform.translation.y;
                lowest_platform = Some((platform, *transform));
            }
        }

        if let Some((lowest_platform, _)) = lowest_platform {
            if lowest_platform != land_event.0 {
                commands.entity(lowest_platform).despawn();
            }
        } else {
            panic!("lowest platform not found")
        }
    }
}

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Platform>()
            .add_startup_system(create_starting_platform)
            .add_systems((create_platforms, delete_platform));
    }
}
