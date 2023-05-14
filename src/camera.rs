use bevy::prelude::*;

#[derive(Component)]
pub struct GameCamera;

fn create_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera, Name::new("Camera")));
}

fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<FollowedByCamera>)>,
    mut followed_by_camera_query: Query<
        &mut Transform,
        (With<FollowedByCamera>, Without<GameCamera>),
    >,
) {
    let mut camera = camera_query.single_mut();
    let followed = followed_by_camera_query.single_mut();

    let delta = followed.translation - camera.translation;
    camera.translation = Vec3::new(
        camera.translation.x + delta.x * 0.1,
        camera.translation.y + delta.y * 0.1,
        camera.translation.z,
    )
}

pub struct GameCameraPlugin;

#[derive(Component)]
pub struct FollowedByCamera;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_camera)
            .add_system(camera_follow);
    }
}
