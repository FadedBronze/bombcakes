use crate::player::PlayerLandedOnEvent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::*;

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
    let mut rand_gen = rand::thread_rng();

    if platform_count <= 3 {
        let mut highest_platform_y = f32::NEG_INFINITY;
        let mut highest_platform: Option<Transform> = None;

        for platform in platforms.iter() {
            if platform.translation.y > highest_platform_y {
                highest_platform_y = platform.translation.y;
                highest_platform = Some(*platform);
            }
        }

        let Some(last_platform) = highest_platform else {
            return;
        };

        let platform_spawn_range_x = rand_gen.gen_range(450.0..600.0);
        let platform_spawn_range_y = rand_gen.gen_range(125.0..150.0);

        let random_dir_left = rand_gen.gen_bool(0.5);

        commands.spawn(create_platform(
            &asset_server,
            Transform::from_xyz(
                last_platform.translation.x
                    + if random_dir_left {
                        -platform_spawn_range_x
                    } else {
                        platform_spawn_range_x
                    },
                last_platform.translation.y + platform_spawn_range_y,
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
