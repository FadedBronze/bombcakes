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
    .add_system(camera_follow)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    // .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(background::BackgroundPlugin)
    .add_startup_system(create_starting_platform)
    .add_system(create_platforms)
    .add_startup_system(spawn_player)
    .add_system(move_player)
    .add_system(jump_player)
    .add_system(follow_eyes)
    .register_type::<PlayerMove>()
    .register_type::<PlayerJump>()
    .register_type::<Platform>()
    .add_system(ground_player)
    .insert_resource(CurrentPlatform(None))
    .add_system(delete_platform)
    .run();
}
#[derive(Component)]
pub struct MyGameCamera;

fn create_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MyGameCamera,
        Name::new("Camera")
    ));
}

fn camera_follow(
    mut camera_query: Query<&mut Transform, (Without<PlayerMove>, With<MyGameCamera>)>,
    player_query: Query<&Transform, With<PlayerMove>>
) {
    let mut camera = camera_query.single_mut();
    let player = player_query.single();

    let delta = player.translation - camera.translation;
    camera.translation = Vec3::new(
        camera.translation.x + delta.x * 0.1,
        camera.translation.y + delta.y * 0.1, 
        camera.translation.z
    )
}

#[derive(Resource)]
struct CurrentPlatform(Option<Entity>);

#[derive(Component, Reflect)]
struct Platform {
    player_touched: bool,
} 

fn create_starting_platform(
    asset_server: Res<AssetServer>,
    mut commands: Commands
)  {
    commands.spawn(create_platform(&asset_server, Transform::from_xyz(0.0, -150.0, 1.0)));
}

fn create_platforms(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    platforms: Query<&Transform, With<Platform>>,
)  {

    let platform_count = platforms.iter().len();

    if platform_count == 1 {
        let last_platform = platforms.single();

        let random_dir = -1.0;

        commands.spawn(create_platform(
            &asset_server, 
            Transform::from_xyz(
                last_platform.translation.x + 450.0 * random_dir, 
                last_platform.translation.y + 80.0, 
                1.0
            )
        ));
    }
}

fn create_platform(asset_server: &AssetServer, transform: Transform) -> (SpriteBundle, Platform, RigidBody, Collider, Name) {
    (SpriteBundle {
        texture: asset_server.load("platform.png"),
        transform,
        ..default()
    }, Platform {
        player_touched: false
    }, RigidBody::Fixed, Collider::cuboid(154.0, 38.0), Name::new("Platform"))
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

#[derive(Component)]
struct PlayerGroundSensor;

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
        jump_force: 300.0,
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
            )
        );

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
            Collider::ball(
                70.0
            ),
            Sensor,
            ActiveEvents::COLLISION_EVENTS
        ));
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

//TODO: make the platforms form
//TODO: learn about the line renderer and code the gun

fn delete_platform(
    platforms: Query<(Entity, &Transform), With<Platform>>,
    current_platform: ResMut<CurrentPlatform>,
    mut commands: Commands
) {
    if platforms.iter().len() == 1 { return; }

    let mut lowest_platform: Option<(Entity, Transform)> = None;
    let mut lowest_platform_y = f32::INFINITY;

    for  (platform, transform) in platforms.iter() {
        if transform.translation.y < lowest_platform_y {
            lowest_platform_y = transform.translation.y;
            lowest_platform = Some((platform, *transform));
        }
    }

    if let Some((lowest_platform, _)) = lowest_platform {
        if let Some(standing_platform) = current_platform.0 {    

            if lowest_platform != standing_platform {
                commands.entity(lowest_platform).despawn();
            }
        }
    } else {
        panic!("lowest platform not found")
    }
}

fn ground_player(
    mut collision_events: EventReader<CollisionEvent>,
    sensor_query: Query<Entity, With<PlayerGroundSensor>>,
    mut player_jump_query: Query<&mut PlayerJump, Without<PlayerGroundSensor>>,
    mut platforms: Query<(Entity, &mut Platform), With<Platform>>,
    mut current_platform: ResMut<CurrentPlatform>
) {
    let sensor = sensor_query.single();
    let mut player_jump = player_jump_query.single_mut();

    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(h1, h2, _event_flag) = collision_event {
            if h1 == &sensor || h2 == &sensor {
                player_jump.grounded = true;

                for (entity, mut platform) in platforms.iter_mut() {
                    if h1 == &entity || h2 == &entity {
                        platform.player_touched = true;
                        current_platform.as_mut().0 = Some(entity);
                        break;
                    }
                }
            }
        }

        if let CollisionEvent::Stopped(h1, h2, _event_flag) = collision_event {
            if h1 == &sensor || h2 == &sensor {
                player_jump.grounded = false;
            }
        }
    }
}
