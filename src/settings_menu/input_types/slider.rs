use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component, Reflect)]
pub(super) struct SliderHandle {
    pub position: f32,
    drag_start: Option<f32>,
}

#[derive(Component)]
pub(super) struct SliderCover;

#[derive(Component)]
pub(super) struct SliderBack;

pub fn create_slider(
    asset_server: &AssetServer,
    parent: &mut ChildBuilder,
    label: &str,
    bundle: impl Bundle,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: asset_server.load("century-gothic/gothic.ttf"),
                    font_size: 23.0,
                    color: Color::WHITE,
                },
            ));

            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(200.0), Val::Px(13.0)),
                            margin: UiRect::all(Val::Px(10.0)),
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        ..default()
                    },
                    SliderBack,
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        image: asset_server.load("inputs/slider/slider_back.png").into(),
                        style: Style {
                            size: Size::new(Val::Px(200.0), Val::Px(13.0)),
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn(ImageBundle {
                        image: asset_server.load("inputs/slider/slider_bar.png").into(),
                        style: Style {
                            size: Size::new(Val::Px(190.0), Val::Px(4.7)),
                            position: UiRect {
                                left: Val::Px(5.0),
                                right: Val::Px(5.0),
                                top: Val::Px(4.15),
                                bottom: Val::Px(4.15),
                            },
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn((
                        ButtonBundle {
                            image: asset_server.load("inputs/slider/slider_handle.png").into(),
                            style: Style {
                                size: Size::new(Val::Px(9.0), Val::Px(9.0)),
                                position: UiRect {
                                    left: Val::Px(5.0),
                                    right: Val::Auto,
                                    top: Val::Px(2.0),
                                    bottom: Val::Px(2.0),
                                },
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            z_index: ZIndex::Local(10),
                            ..default()
                        },
                        SliderHandle {
                            position: 0.0,
                            drag_start: None,
                        },
                        bundle,
                    ));

                    parent.spawn((
                        NodeBundle {
                            background_color: BackgroundColor(Color::BLACK),
                            style: Style {
                                size: Size::new(Val::Px(10.0), Val::Px(9.0)),
                                position: UiRect {
                                    left: Val::Auto,
                                    right: Val::Px(5.0),
                                    top: Val::Px(2.0),
                                    bottom: Val::Px(2.0),
                                },
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            ..default()
                        },
                        SliderCover,
                    ));
                });
        });
}

pub(super) fn start_drag_slider(
    mut slider_query: Query<
        (&Interaction, &mut BackgroundColor, &mut SliderHandle),
        Changed<Interaction>,
    >,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(cursor_position) = primary_query.single().cursor_position() else {
    return;
  };

    for (interaction, mut background_color, mut slider_handle) in slider_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                slider_handle.drag_start = Some(cursor_position.x);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 1.0));
            }
            Interaction::None => {
                *background_color = BackgroundColor(Color::rgba(0.9, 0.9, 0.9, 1.0));
            }
        }
    }
}

pub(super) fn drag_slider(
    slider_back_query: Query<
        Entity,
        (
            With<SliderBack>,
            Without<SliderCover>,
            Without<SliderHandle>,
        ),
    >,
    mut slider_cover_query: Query<
        (&Parent, &mut Style),
        (
            Without<SliderBack>,
            With<SliderCover>,
            Without<SliderHandle>,
        ),
    >,
    mut slider_handle_query: Query<
        (&Parent, &mut Style, &mut SliderHandle),
        (
            Without<SliderBack>,
            Without<SliderCover>,
            With<SliderHandle>,
        ),
    >,
    mouse: Res<Input<MouseButton>>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(cursor_position) = primary_query.single().cursor_position() else {
        return;
    };

    for parent in slider_back_query.iter() {
        let mut slider_handle = Err("No child slider handle found");
        let mut slider_cover = Err("No child slider cover found");

        for child_slider_handle in slider_handle_query.iter_mut() {
            if child_slider_handle.0.get() == parent {
                slider_handle = Ok(child_slider_handle);
            }
        }

        for child_slider_cover in slider_cover_query.iter_mut() {
            if child_slider_cover.0.get() == parent {
                slider_cover = Ok(child_slider_cover);
            }
        }

        let Ok((_, mut handle_style, mut handle)) = slider_handle else { break; };
        let Ok((_, mut cover_style)) = slider_cover else { break; };

        let Some(drag_start) = handle.drag_start else {
          //if not dragging we want to just set the bar to its position

          handle_style.position = UiRect::new(
            Val::Px((handle.position) * 190.0),
            Val::Auto,
            Val::Px(2.0),
            Val::Px(2.0),
          );

            cover_style.size.width = Val::Px((1.0 - handle.position) * 190.0);

          continue;
        };

        let distance = (cursor_position.x - drag_start) / 190.0;

        let handle_display_pos = (handle.position + distance).min(1.0).max(0.0) * 190.0;

        handle_style.position = UiRect::new(
            Val::Px(handle_display_pos),
            Val::Auto,
            Val::Px(2.0),
            Val::Px(2.0),
        );

        cover_style.size.width = Val::Px(190.0 - handle_display_pos);

        if mouse.just_released(MouseButton::Left) {
            handle.position += distance;
            handle.position = handle.position.min(1.0).max(0.0);
            handle.drag_start = None;
        }
    }
}
