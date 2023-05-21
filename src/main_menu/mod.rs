use bevy::{app::AppExit, prelude::*};

use crate::AppState;

pub struct MenuPlugin;

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct PlayButton;

fn create_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    gap: Size::new(Val::Px(10.0), Val::Px(10.0)),
                    position_type: PositionType::Relative,
                    ..default()
                },
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(80.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Percent(0.0),
                            right: Val::Auto,
                            top: Val::Percent(0.0),
                            bottom: Val::Auto,
                        },
                        padding: UiRect::new(
                            Val::Px(10.0),
                            Val::Px(10.0),
                            Val::Px(10.0),
                            Val::Px(10.0),
                        ),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(ButtonBundle {
                        image: asset_server.load("setting_icon.png").into(),
                        style: Style {
                            size: Size::new(Val::Auto, Val::Percent(100.0)),
                            aspect_ratio: Some(1.0 / 1.0),
                            ..default()
                        },
                        ..default()
                    });

                    parent.spawn(ButtonBundle {
                        image: asset_server.load("music_icon.png").into(),
                        style: Style {
                            size: Size::new(Val::Auto, Val::Percent(100.0)),
                            aspect_ratio: Some(1.0 / 1.0),
                            ..default()
                        },
                        ..default()
                    });
                });

            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(600.0), Val::Px(80.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(80.0)),
                    ..default()
                },
                image: asset_server.load("Bombcakes.png").into(),
                ..default()
            });

            parent
                .spawn((
                    PlayButton,
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(220.0), Val::Px(80.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        image: UiImage {
                            texture: asset_server.load("blue_button.png"),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                TextStyle {
                                    font: asset_server.load("century-gothic/gothic_bold.ttf"),
                                    font_size: 50.0,
                                    color: Color::ALICE_BLUE,
                                },
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });

            parent
                .spawn((
                    ExitButton,
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(100.0), Val::Px(50.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        image: UiImage {
                            texture: asset_server.load("red_button.png"),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0)),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                TextStyle {
                                    font: asset_server.load("century-gothic/gothic_bold.ttf"),
                                    font_size: 30.0,
                                    color: Color::ALICE_BLUE,
                                },
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

fn despawn_main_menu(mut commands: Commands, main_menu: Query<Entity, With<MainMenu>>) {
    commands.entity(main_menu.single()).despawn_recursive();
}

fn switch_into_menu(
    keys: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if app_state.0 == AppState::Menu {
            commands.insert_resource(NextState(Some(AppState::InGame)));
        } else {
            commands.insert_resource(NextState(Some(AppState::Menu)));
        }
    }
}

fn interact_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut commands: Commands,
) {
    let Ok((interaction, mut background_color)) = button_query.get_single_mut() else {
    return;
  };

    match *interaction {
        Interaction::Clicked => {
            commands.insert_resource(NextState(Some(AppState::InGame)));
        }
        Interaction::Hovered => {
            *background_color = BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 1.0));
        }
        Interaction::None => {
            *background_color = BackgroundColor(Color::rgba(0.95, 0.95, 0.95, 1.0));
        }
    }
}

fn interact_exit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ExitButton>),
    >,
    mut ev_exit: EventWriter<AppExit>,
) {
    let Ok((interaction, mut background_color)) = button_query.get_single_mut() else {
  return;
};

    match *interaction {
        Interaction::Clicked => {
            ev_exit.send(AppExit);
        }
        Interaction::Hovered => {
            *background_color = BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 1.0));
        }
        Interaction::None => {
            *background_color = BackgroundColor(Color::rgba(0.95, 0.95, 0.95, 1.0));
        }
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(switch_into_menu)
            .add_system(create_main_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::Menu)))
            .add_systems(
                (interact_exit_button, interact_play_button).in_set(OnUpdate(AppState::Menu)),
            );
    }
}
