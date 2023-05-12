use bevy::{prelude::*, window::*};

#[derive(Component)]
struct Background {
    iteration: i32,
}

fn create_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let mut transform = Transform {
        scale: Vec3::new(1.0, 1.0, 1.0),
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
        Name::new("Background")
    ));
}

fn animate_background(mut background_query: Query<(&mut Background, &mut Transform)>) {
    let (mut background, mut transform) = background_query.single_mut();

    background.iteration += 1;
    transform.rotation.z = ((background.iteration as f32) / 50.0).sin() / 10.0;
}

fn update_background_image_size(resize_event: Res<Events<WindowResized>>, mut background_query: Query<&mut Transform, With<Background>>) {
    let mut transform = background_query.single_mut();

    let mut reader = resize_event.get_reader();

    for e in reader.iter(&resize_event) {
        transform.scale = Vec3::new(e.width / 1920.0 * 1.5, e.height / 1080.0 * 1.5, 0.0);
    }
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
  fn build(&self, app: &mut App) {
      app.add_system(animate_background).add_system(update_background_image_size).add_startup_system(create_background);
  }
}