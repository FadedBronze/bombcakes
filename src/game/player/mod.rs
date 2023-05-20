mod player_jump;
mod player_move;

use crate::{
    camera::*, game::arms::ArmsTarget, game::rocket_launcher::RocketLauncherHolderSpawns, AppState,
};
use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;

use self::player_jump::{ground_player, jump_player};
use self::player_move::move_player;

use super::{GameEntity, PausedState};

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
    let Ok(mut eyes_transform) = player_eyes_query.get_single_mut() else {
        return;
    };

    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let delta = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y - 18.0,
        player_transform.translation.z,
    ) - eyes_transform.translation;

    eyes_transform.translation += delta / 2.0;
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
            GameEntity,
            ArmsTarget,
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
        GameEntity,
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

fn player_death(
    mut commands: Commands,
    player_query: Query<(), (With<PlayerMove>, Without<PlayerEyes>, Without<PlayerLegs>)>,
    player_eyes_query: Query<Entity, (With<PlayerEyes>, Without<PlayerMove>, Without<PlayerLegs>)>,
    player_legs_query: Query<Entity, (With<PlayerLegs>, Without<PlayerMove>, Without<PlayerEyes>)>,
) {
    let Ok(_) = player_query.get_single() else {
        if let Ok(player_eyes) = player_eyes_query.get_single() {
            commands.entity(player_eyes).despawn();
        }
        if let Ok(player_legs) = player_legs_query.get_single() {
            commands.entity(player_legs).despawn();
        }

        return;
    };
}

pub struct PlayerPlugin;

pub struct PlayerLandedOnEvent(pub Entity);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                follow_eyes,
                move_player,
                jump_player,
                ground_player,
                player_death,
            )
                .in_set(OnUpdate(AppState::InGame))
                .in_set(OnUpdate(PausedState::Playing)),
        )
        .add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
        .register_type::<PlayerMove>()
        .register_type::<PlayerJump>()
        .add_event::<PlayerLandedOnEvent>();
    }
}
