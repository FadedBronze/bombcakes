use bevy::{prelude::*, sprite::*};
mod background;
use bevy_rapier2d::{prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const BACKGROUND_COLOR: Color = Color::AZURE;

fn main() {
    App::new().insert_resource(ClearColor(BACKGROUND_COLOR))
    .add_plugins(DefaultPlugins)
    .add_plugin(WorldInspectorPlugin::new())
    .add_startup_system(create_camera)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(background::BackgroundPlugin)
    .add_startup_system(create_starting_platform)
    .add_startup_system(spawn_player)
    .add_system(move_player)
    .add_system(jump_player)
    .add_system(follow_eyes)
    .register_type::<PlayerMove>()
    .register_type::<PlayerJump>()
    .add_system(ground_player)
    .run();
}
#[derive(Component)]
struct MyGameCamera;

fn create_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MyGameCamera,
        Name::new("Camera")
    ));
}


#[derive(Component)]
struct Platform;

fn create_starting_platform(
    asset_server: Res<AssetServer>,
    mut commands: Commands
)  {
    commands.spawn(create_platform(&asset_server, Transform::from_xyz(0.0, -150.0, 1.0)));
}

fn create_platform(asset_server: &AssetServer, transform: Transform) -> (SpriteBundle, Platform, RigidBody, Collider, Name) {
    (SpriteBundle {
        texture: asset_server.load("platform.png"),
        transform,
        ..default()
    }, Platform, RigidBody::Fixed, Collider::cuboid(154.0, 38.0), Name::new("Platform"))
}

#[derive(Component, Reflect)]
struct PlayerMove {
    speed: f32,
    max_speed: f32,
    seconds_to_stop_after_key_release: f32
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

fn spawn_player(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
)  {

    let player = commands.spawn((PlayerMove {
        speed: 50.0,
        max_speed: 180.0,
        seconds_to_stop_after_key_release: 0.3,
    }, PlayerJump {
        grounded: true,
        jump_force: 240.0,
    }, SpriteBundle {
        texture: asset_server.load("cupcake.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::new(0.1, 0.1, 0.1),
            ..default()
        },
        ..default()
    }, 
        RigidBody::Dynamic, 
        Collider::capsule(
            Vec2::new(0.0, -195.0), 
            Vec2::new(0.0, 0.0), 
            300.0
        ), 
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED_Z, 
        GravityScale(3.0),
        ActiveEvents::COLLISION_EVENTS,
        Name::new("Player")
    )).id();


    commands.spawn((PlayerEyes, SpriteBundle {
        texture: asset_server.load("angry_eyes.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            scale: Vec3::new(0.1, 0.1, 0.1),
            ..default()
        },
        ..default()
    }, 
        Name::new("Eyes"))
    );

    commands.entity(player).with_children(|parent| {
        parent.spawn((PlayerLegs, SpriteBundle {
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
            Name::new("Legs"))
        );
    });
}

fn move_player(
    mut player_query: Query<(&mut Velocity, &PlayerMove)>,
    keys: Res<Input<KeyCode>>,
    mut stopped_player: Local<bool>,
    mut x_velocity_when_stopped: Local<f32>,
    mut counter: Local<f32>,
    time: Res<Time>
) {
    let delta_time = time.delta_seconds();

    let (mut velocity, player) = player_query.single_mut();

    if keys.pressed(KeyCode::A) {
        velocity.linvel = Vec2::new((velocity.linvel.x - player.speed).max(-player.max_speed), velocity.linvel.y);
        *stopped_player = false;
    } else if keys.pressed(KeyCode::D) {
        velocity.linvel = Vec2::new((velocity.linvel.x + player.speed).min(player.max_speed), velocity.linvel.y);
        *stopped_player = false;
    } else {
        if *stopped_player == true {
            
            let mut stopped_amount = 1.0 - (*counter / player.seconds_to_stop_after_key_release);
            
            if stopped_amount > 0.0 {
                *counter += delta_time;
            } else {
                stopped_amount = 0.0
            }

            velocity.linvel = Vec2::new(*x_velocity_when_stopped * stopped_amount, velocity.linvel.y);
        } else {
            *counter = 0.0;
            *stopped_player = true;
            *x_velocity_when_stopped = velocity.linvel.x;
        } 
    }
}

fn jump_player(
    mut player_query: Query<(&mut Velocity, &mut PlayerJump), Without<PlayerLegs>>,
    mut player_legs_query: Query<&mut Transform, (With<PlayerLegs>, Without<PlayerJump>)>,
    keys: Res<Input<KeyCode>>,
    mut time_since_jump: Local<f32>,
    time: Res<Time>
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

fn follow_eyes(
    player_query: Query<&Transform, (With<PlayerMove>, Without<PlayerEyes>)>,
    mut player_eyes_query: Query<&mut Transform, (With<PlayerEyes>, Without<PlayerMove>)>
) {
    let mut eyes_transform = player_eyes_query.single_mut();
    let player_transform = player_query.single();

    let delta = Vec3::new(player_transform.translation.x, player_transform.translation.y - 18.0, player_transform.translation.z) - eyes_transform.translation;

    eyes_transform.translation += delta / 2.0;
}

//TODO: create a detection collider for detecting the ground only
fn ground_player(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut PlayerJump), With<PlayerJump>>,
) {
    let (player, mut player_jump) = player_query.single_mut();

    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _event_flag) = collision_event {
            if h1 == &player || h2 == &player {
                player_jump.grounded = true;
            }
        }

        if let CollisionEvent::Stopped(h1, h2, _event_flag) = collision_event {
            if h1 == &player || h2 == &player {
                player_jump.grounded = false;
            }
        }
    }
}
