use bevy::prelude::*;

#[derive(Component)]
struct Arms;

fn spawn_arms(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("arms.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(500.0, 3672.0), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Arms,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(2),
            transform: Transform {
                translation: Vec3::new(0.0, -200.0, 0.3),
                scale: Vec3::new(0.15, 0.15, 0.15),
                ..default()
            },
            ..default()
        },
        Name::new("Arm"),
    ));
}

pub struct ArmsPlugin;

impl Plugin for ArmsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_arms);
    }
}
