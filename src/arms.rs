use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionEvent, GravityScale, RigidBody, Sensor, Velocity};
use rand::*;

use crate::{camera::FollowedByCamera, rocket_launcher::RocketTarget};

#[derive(Component)]
struct Arms;

#[derive(Component)]
struct GrabHitbox;

#[derive(Resource)]
struct SpawnArmTimer(Timer);

fn spawn_arms(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut spawn_arm_timer: ResMut<SpawnArmTimer>,
    time: Res<Time>,
    target: Query<&Transform, With<ArmsTarget>>,
) {
    let mut random_gen = rand::thread_rng();

    if spawn_arm_timer.0.finished() {
        let texture_handle = asset_server.load("arms.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(500.0, 3672.0), 3, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let Ok(target_transform) = target.get_single() else {
          return;
        };
        let spawn_position = target_transform.translation
            + Vec3::new(random_gen.gen_range(-150.0..150.0), -1200.0, 0.0);

        commands
            .spawn((
                Arms,
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform {
                        translation: spawn_position,
                        scale: Vec3::new(0.15, 0.15, 0.15),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Arm"),
                RigidBody::KinematicVelocityBased,
                GravityScale(0.0),
                Velocity {
                    linvel: Vec2::new(0.0, 300.0),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpatialBundle {
                        transform: Transform {
                            translation: Vec3::new(-10.0, 225.0, 0.1),
                            ..default()
                        },
                        ..default()
                    },
                    Sensor,
                    Collider::cuboid(150.0, 150.0),
                    GrabHitbox,
                ));

                parent.spawn((
                    SpatialBundle {
                        transform: Transform {
                            translation: Vec3::new(-10.0, 225.0, 0.1),
                            ..default()
                        },
                        ..default()
                    },
                    Sensor,
                    Collider::cuboid(200.0, 300.0),
                    RocketTarget,
                ));
            });
    }

    spawn_arm_timer.0.tick(time.delta());
}

fn grab_target(
    mut collision_events: EventReader<CollisionEvent>,
    target_query: Query<Entity, (With<ArmsTarget>, Without<Arms>)>,
    mut commands: Commands,
    mut hands_query: Query<
        (Entity, &Children, &mut TextureAtlasSprite),
        (With<Arms>, Without<ArmsTarget>),
    >,
    grab_hit_box_query: Query<Entity, With<GrabHitbox>>,
) {
    let Ok(target) = target_query.get_single() else {
      return;
    };

    for collision in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _event_flag) = collision {
            for (hand, hand_children, mut sprite) in hands_query.iter_mut() {
                let mut grab_hitbox = Err("no hitbox");

                for child in hand_children.iter() {
                    if let Ok(child_grab_hitbox) = grab_hit_box_query.get(*child) {
                        grab_hitbox = Ok(child_grab_hitbox);
                    }
                }

                if h1 == &target && h2 == &grab_hitbox.unwrap() {
                    commands.entity(target).despawn_recursive();

                    commands.entity(hand).insert(FollowedByCamera);
                    sprite.index = 2;
                }

                if h2 == &target && h1 == &grab_hitbox.unwrap() {
                    commands.entity(target).despawn_recursive();

                    commands.entity(hand).insert(FollowedByCamera);
                    sprite.index = 2;
                }
            }
        }
    }
}

fn chase_target(
    target_query: Query<&Transform, (With<ArmsTarget>, Without<Arms>)>,
    mut hands_query: Query<
        (&Transform, &mut Velocity, &mut TextureAtlasSprite),
        (With<Arms>, Without<ArmsTarget>),
    >,
) {
    let Ok(target) = target_query.get_single() else {
        return;
    };

    for (hand_transform, mut hand_velocity, mut hand_sprite) in hands_query.iter_mut() {
        if (hand_transform.translation.y - target.translation.y).abs() < 550.0
            && (hand_transform.translation.x - target.translation.x).abs() < 150.0
        {
            hand_velocity.linvel = Vec2::new(0.0, 600.0);
            hand_sprite.index = 1;
        }
    }
}

pub struct ArmsPlugin;

#[derive(Component)]
pub struct ArmsTarget;

impl Plugin for ArmsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnArmTimer(Timer::new(
            Duration::from_secs(2),
            TimerMode::Repeating,
        )))
        .add_system(spawn_arms)
        .add_system(chase_target)
        .add_system(grab_target);
    }
}
