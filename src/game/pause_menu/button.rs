use bevy::prelude::*;

pub(super) fn create_pause_menu_button(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    label: &str,
    bundle: impl Bundle,
) {
    parent
        .spawn((
            ButtonBundle {
                background_color: BackgroundColor(Color::BLACK),
                style: Style {
                    
                    padding: UiRect::all(Val::Px(15.0)),
                    size: Size::width(Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            },
            bundle,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: asset_server.load("century-gothic/gothic.ttf"),
                    color: Color::WHITE,
                    font_size: 25.0,
                },
            ));
        });
}
