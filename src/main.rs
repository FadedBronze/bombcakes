use bevy::{prelude::*};
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
    // .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(background::BackgroundPlugin)
    .add_startup_system(create_starting_platform)
    .add_startup_system(spawn_player)
    .add_system(move_player)
    .add_system(jump_player)
    .add_system(follow_eyes)
    .register_type::<Player>()
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
struct Player {
    speed: f32,
    max_x_velocity: f32,
    velocity_decrease: f32,
    jump_force: f32,
    grounded: bool,
}

#[derive(Component)]
struct PlayerEyes;

fn spawn_player(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
)  {

    commands.spawn((Player {
        jump_force: 240.0,
        speed: 50.0,
        max_x_velocity: 180.0,
        velocity_decrease: 0.95,
        grounded: false,
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
        Collider::capsule(Vec2::new(0.0, -75.0), 
        Vec2::new(0.0, 0.0), 300.0), 
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED_Z, 
        GravityScale(3.0),
        Name::new("Player"))
    );


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
}

fn move_player(
    mut player_query: Query<(&mut Velocity, &Player)>,
    keys: Res<Input<KeyCode>>,
    mut counter: Local<f32>,
    time: Res<Time>
) {
    let delta_time = time.delta_seconds();

    let (mut velocity, player) = player_query.single_mut();

    if keys.pressed(KeyCode::A) {
        velocity.linvel = Vec2::new((velocity.linvel.x - player.speed).max(-player.max_x_velocity), velocity.linvel.y);
    } else if keys.pressed(KeyCode::D) {
        velocity.linvel = Vec2::new((velocity.linvel.x + player.speed).min(player.max_x_velocity), velocity.linvel.y);
    } else {
        //incrementing counter
        *counter += delta_time;

        //code runs 50 times a second
        if (*counter * 50.0).floor() > ((*counter - delta_time) * 50.0).floor() {
            velocity.linvel = Vec2::new(velocity.linvel.x * player.velocity_decrease, velocity.linvel.y);
        }
    }
}

fn jump_player(
    mut player_query: Query<(&mut Velocity, &mut Player)>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut velocity, mut player) = player_query.single_mut();

    if keys.just_pressed(KeyCode::W) {
        velocity.linvel.y = player.jump_force;
        player.grounded = false;
    }
}

fn follow_eyes(
    player_query: Query<&Transform, (With<Player>, Without<PlayerEyes>)>,
    mut player_eyes_query: Query<&mut Transform, (With<PlayerEyes>, Without<Player>)>
) {
    let mut eyes_transform = player_eyes_query.single_mut();
    let player_transform = player_query.single();

    let delta = Vec3::new(player_transform.translation.x, player_transform.translation.y - 18.0, player_transform.translation.z) - eyes_transform.translation;

    eyes_transform.translation += delta / 2.0;
}