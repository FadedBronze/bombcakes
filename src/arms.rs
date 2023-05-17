use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionEvent, GravityScale, RigidBody, Sensor, Velocity};
use rand::*;

#[derive(Component)]
struct Arms;

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
                            translation: Vec3::new(0.0, 225.0, 0.1),
                            ..default()
                        },
                        ..default()
                    },
                    Sensor,
                    Collider::cuboid(150.0, 150.0),
                ));
            });
    }

    spawn_arm_timer.0.tick(time.delta());
}

//check for collision
//update sprite to grabby one
//set velocity to 0
//trigger a delete player
fn grab_target(
    mut collision_events: EventReader<CollisionEvent>,
    target_query: Query<Entity, (With<ArmsTarget>, Without<Arms>)>,
    mut commands: Commands,
    mut hands_query: Query<(&Children, &mut Velocity), (With<Arms>, Without<ArmsTarget>)>,
) {
    let Ok(target) = target_query.get_single() else {
      return;
    };

    for collision in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _event_flag) = collision {
            for (hand_children, mut hand_velocity) in hands_query.iter_mut() {
                let hand = hand_children.iter().next().unwrap();

                if h1 == &target && h2 == hand {
                    commands.entity(target).despawn_recursive();

                    hand_velocity.linvel = Vec2::ZERO;
                }

                if h2 == &target && h1 == hand {
                    commands.entity(target).despawn_recursive();

                    hand_velocity.linvel = Vec2::ZERO;
                }
            }
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
        .add_system(grab_target);
    }
}
