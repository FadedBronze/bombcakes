use crate::{camera::*, rocket_launcher::RocketLauncherHolderSpawns};
use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

#[derive(Component, Reflect)]
struct PlayerMove {
    speed: f32,
    max_speed: f32,
    seconds_to_stop_after_key_release: f32,
}

#[derive(Component, Reflect)]
struct PlayerJump {
    jump_force: f32,
    grounded: bool,
}

#[derive(Component)]
struct PlayerEyes;

#[derive(Component)]
struct PlayerLegs;

#[derive(Component)]
struct PlayerGroundSensor;

fn follow_eyes(
    player_query: Query<&Transform, (With<PlayerMove>, Without<PlayerEyes>)>,
    mut player_eyes_query: Query<&mut Transform, (With<PlayerEyes>, Without<PlayerMove>)>,
) {
    let mut eyes_transform = player_eyes_query.single_mut();
    let player_transform = player_query.single();

    let delta = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y - 18.0,
        player_transform.translation.z,
    ) - eyes_transform.translation;

    eyes_transform.translation += delta / 2.0;
}

fn jump_player(
    mut player_query: Query<(&mut Velocity, &mut PlayerJump), Without<PlayerLegs>>,
    mut player_legs_query: Query<&mut Transform, (With<PlayerLegs>, Without<PlayerJump>)>,
    keys: Res<Input<KeyCode>>,
    mut time_since_jump: Local<f32>,
    time: Res<Time>,
) {
    let (mut velocity, mut player_jump) = player_query.single_mut();
    let mut legs_transform = player_legs_query.single_mut();

    if keys.just_pressed(KeyCode::W) && player_jump.grounded {
        velocity.linvel.y = player_jump.jump_force;
        player_jump.grounded = false;

        *time_since_jump = 0.0;
    } else {
        *time_since_jump += time.delta_seconds();
        legs_transform.scale.y = 2.0;
    }

    if *time_since_jump < 0.2 {
        legs_transform.scale.y = 1.8;
    }
}

fn move_player(
    mut player_query: Query<(&mut Velocity, &PlayerMove)>,
    keys: Res<Input<KeyCode>>,
    mut stopped_player: Local<bool>,
    mut x_velocity_when_stopped: Local<f32>,
    mut counter: Local<f32>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();

    let (mut velocity, player) = player_query.single_mut();

    if keys.pressed(KeyCode::A) {
        velocity.linvel = Vec2::new(
            (velocity.linvel.x - player.speed).max(-player.max_speed),
            velocity.linvel.y,
        );
        *stopped_player = false;
    } else if keys.pressed(KeyCode::D) {
        velocity.linvel = Vec2::new(
            (velocity.linvel.x + player.speed).min(player.max_speed),
            velocity.linvel.y,
        );
        *stopped_player = false;
    } else {
        if *stopped_player == true {
            let mut stopped_amount = 1.0 - (*counter / player.seconds_to_stop_after_key_release);

            if stopped_amount > 0.0 {
                *counter += delta_time;
            } else {
                stopped_amount = 0.0
            }

            velocity.linvel =
                Vec2::new(*x_velocity_when_stopped * stopped_amount, velocity.linvel.y);
        } else {
            *counter = 0.0;
            *stopped_player = true;
            *x_velocity_when_stopped = velocity.linvel.x;
        }
    }
}

fn spawn_player(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut ev_rocket_launcher_holder_spawns: EventWriter<RocketLauncherHolderSpawns>,
) {
    let player = commands
        .spawn((
            PlayerMove {
                speed: 50.0,
                max_speed: 380.0,
                seconds_to_stop_after_key_release: 0.3,
            },
            PlayerJump {
                grounded: true,
                jump_force: 300.0,
            },
            SpriteBundle {
                texture: asset_server.load("cupcake.png"),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    scale: Vec3::new(0.1, 0.1, 0.1),
                    ..default()
                },
                ..default()
            },
            RigidBody::Dynamic,
            Collider::capsule(Vec2::new(0.0, -195.0), Vec2::new(0.0, 0.0), 300.0),
            Velocity::default(),
            LockedAxes::ROTATION_LOCKED_Z,
            GravityScale(3.0),
            ActiveEvents::COLLISION_EVENTS,
            Name::new("Player"),
            FollowedByCamera,
        ))
        .id();

    commands.spawn((
        PlayerEyes,
        SpriteBundle {
            texture: asset_server.load("angry_eyes.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(0.1, 0.1, 0.1),
                ..default()
            },
            ..default()
        },
        Name::new("Eyes"),
    ));

    commands.entity(player).with_children(|parent| {
        parent.spawn((
            PlayerLegs,
            SpriteBundle {
                texture: asset_server.load("legs.png"),
                sprite: Sprite {
                    anchor: Anchor::TopCenter,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, -330.0, 1.0),
                    scale: Vec3::new(2.1, 2.1, 0.1),
                    ..default()
                },
                ..default()
            },
            Name::new("Legs"),
        ));

        parent.spawn((
            PlayerGroundSensor,
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, -50.0, 1.0),
                    scale: Vec3::new(2.1, 2.1, 0.1),
                    ..default()
                },
                ..default()
            },
            Name::new("Ground sensor"),
            Collider::ball(70.0),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ));
    });

    ev_rocket_launcher_holder_spawns.send(RocketLauncherHolderSpawns(player))
}

fn ground_player(
    mut collision_events: EventReader<CollisionEvent>,
    sensor_query: Query<Entity, With<PlayerGroundSensor>>,
    mut player_jump_query: Query<&mut PlayerJump, Without<PlayerGroundSensor>>,
    mut ev_landed: EventWriter<PlayerLandedOnEvent>,
) {
    let sensor = sensor_query.single();
    let mut player_jump = player_jump_query.single_mut();

    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _event_flag) = collision_event {
            if h1 == &sensor {
                player_jump.grounded = true;
                ev_landed.send(PlayerLandedOnEvent(*h2));
            }

            if h2 == &sensor {
                player_jump.grounded = true;
                ev_landed.send(PlayerLandedOnEvent(*h1));
            }
        }

        if let CollisionEvent::Stopped(h1, h2, _event_flag) = collision_event {
            if h1 == &sensor || h2 == &sensor {
                player_jump.grounded = false;
            }
        }
    }
}

pub struct PlayerPlugin;
pub struct PlayerLandedOnEvent(pub Entity);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((follow_eyes, move_player, jump_player, ground_player))
            .add_startup_system(spawn_player)
            .register_type::<PlayerMove>()
            .register_type::<PlayerJump>()
            .add_event::<PlayerLandedOnEvent>();
    }
}
