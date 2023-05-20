use bevy::{prelude::*, window::*};

use crate::{camera::GameCamera, AppState};

#[derive(Component)]
struct Background {
    iteration: i32,
}

fn create_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut transform = Transform {
        scale: Vec3::new(1.0, 1.0, -1.0),
        ..default()
    };
    transform.rotate_z(5.0);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("background.png"),
            transform,
            ..default()
        },
        Background { iteration: 0 },
        Name::new("Background"),
    ));
}

fn animate_background(
    mut background_query: Query<(&mut Background, &mut Transform)>,
    camera_query: Query<&Transform, (With<GameCamera>, Without<Background>)>,
) {
    let Ok((mut background, mut transform)) = background_query.get_single_mut() else {
        return;
    };
    let camera = camera_query.single();

    background.iteration += 1;
    transform.rotation.z = ((background.iteration as f32) / 50.0).sin() / 10.0;
    transform.translation = Vec3::new(
        camera.translation.x,
        camera.translation.y,
        transform.translation.z,
    )
}

fn update_background_image_size(
    resize_event: Res<Events<WindowResized>>,
    mut background_query: Query<&mut Transform, With<Background>>,
) {
    let Ok(mut transform) = background_query.get_single_mut() else {
        return;
    };

    let mut reader = resize_event.get_reader();

    for e in reader.iter(&resize_event) {
        transform.scale = Vec3::new(e.width / 1920.0 * 1.5, e.height / 1080.0 * 1.5, 0.0);
    }
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_background.run_if(in_state(AppState::InGame)))
            .add_systems(
                (update_background_image_size, animate_background)
                    .in_set(OnUpdate(AppState::InGame)),
            );
    }
}
