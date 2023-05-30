use std::f32::consts::PI;

use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::{camera::GameCamera, AppState};

use super::{GameEntity, PausedState};
#[derive(Component)]
struct RocketLauncher {
    power: f32,
}

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
                RocketLauncher { power: 800.0 },
                SpriteBundle {
                    texture: asset_server.load("weapons/rocket_launcher/rocket_launcher.png"),
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

    let normalized_delta = delta.normalize();

    let angle = normalized_delta.y.atan2(normalized_delta.x);

    rocket_launcher.rotation = Quat::from_rotation_z(angle + PI / 2.0);
}

#[derive(Component)]
struct Rocket;

fn rocket_launcher_shoots(
    buttons: Res<Input<MouseButton>>,
    rocket_launcher_query: Query<
        (&Transform, &RocketLauncher),
        (With<RocketLauncher>, Without<RocketLauncherHolder>),
    >,
    rocket_launcher_holder_query: Query<
        &Transform,
        (With<RocketLauncherHolder>, Without<RocketLauncher>),
    >,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let Ok((rocket_launcher_transform, rocket_launcher)) = rocket_launcher_query.get_single() else {
        return;
    };

    let Ok(rocket_launcher_holder_transform) = rocket_launcher_holder_query.get_single() else {
        return;
    };

    let direction_angle = Quat::to_euler(rocket_launcher_transform.rotation, EulerRot::XYZ).2;

    let direction = Vec2::new(f32::sin(direction_angle), -f32::cos(direction_angle));

    if buttons.just_pressed(MouseButton::Left) {
        commands.spawn((
            Rocket,
            SpriteBundle {
                texture: asset_server.load("weapons/rocket_launcher/rocket.png"),
                transform: Transform {
                    translation: Vec3::new(
                        rocket_launcher_holder_transform.translation.x + direction.x * 100.0,
                        rocket_launcher_holder_transform.translation.y + direction.y * 100.0,
                        1.0,
                    ),
                    scale: Vec3::new(0.2, 0.2, 0.1),
                    ..default()
                },
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(50.0),
            Restitution {
                coefficient: 1.0,
                ..default()
            },
            GravityScale(3.0),
            Velocity {
                linvel: Vec2::new(
                    direction.x * rocket_launcher.power,
                    direction.y * rocket_launcher.power,
                ),
                ..default()
            },
            Name::new("Rocket"),
            ActiveEvents::COLLISION_EVENTS,
            GameEntity,
        ));
    }
}

fn handle_rocket_hit(
    mut ev_collision: EventReader<CollisionEvent>,
    rocket_targets: Query<(Entity, Option<&Parent>), (With<RocketTarget>, Without<Rocket>)>,
    rockets: Query<Entity, (With<Rocket>, Without<RocketTarget>)>,
    mut commands: Commands,
) {
    for collision_event in ev_collision.iter() {
        if let CollisionEvent::Started(h1, h2, _event_flag) = collision_event {
            for (target, parent_option) in rocket_targets.iter() {
                for rocket in rockets.iter() {
                    if h1 == &target && h2 == &rocket || h2 == &target && h1 == &rocket {
                        if let Some(parent) = parent_option {
                            commands.entity(**parent).despawn_recursive();
                        } else {
                            commands.entity(target).despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}

pub struct RocketLauncherHolderSpawns(pub Entity);
pub struct RocketLauncherPlugin;

#[derive(Component)]
pub struct RocketTarget;

impl Plugin for RocketLauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                rocket_launcher_follows_mouse,
                rocket_launcher_shoots,
                handle_rocket_hit,
            )
                .in_set(OnUpdate(AppState::InGame))
                .in_set(OnUpdate(PausedState::Playing)),
        )
        .add_system(spawn_rocket_launcher.run_if(in_state(AppState::InGame)))
        .add_event::<RocketLauncherHolderSpawns>();
    }
}
