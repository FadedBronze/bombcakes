use bevy::prelude::*;

use crate::{AppState, SettingsState};

mod input_types;

use self::input_types::{slider::create_slider, InputPlugin};

#[derive(Component)]
struct SettingsMenu;

#[derive(Component)]
struct BackButton;

fn create_settings_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(Color::Rgba {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.5,
                }),
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            SettingsMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        size: Size::width(Val::Percent(100.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(30.0), Val::Px(56.2)),
                                ..default()
                            },
                            image: asset_server.load("menus/back_arrow.png").into(),
                            ..default()
                        },
                        BackButton,
                    ));

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Settings",
                                    TextStyle {
                                        font: asset_server.load("century-gothic/gothic.ttf"),
                                        font_size: 60.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::new(
                                        Val::Px(10.0),
                                        Val::Px(10.0),
                                        Val::Px(10.0),
                                        Val::Px(10.0),
                                    ),
                                    ..default()
                                }),
                            );
                        });

                    parent.spawn(NodeBundle::default());
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Start,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            create_slider(&asset_server, parent, "Master volume", ());
                            create_slider(&asset_server, parent, "Music volume", ());
                            create_slider(&asset_server, parent, "SFX volume", ());
                        });
                });
        });
}

fn interact_back_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackButton>),
    >,
    mut commands: Commands,
) {
    let Ok((interaction, mut background_color)) = button_query.get_single_mut() else {
        return;
    };

    match *interaction {
        Interaction::Clicked => {
            commands.insert_resource(NextState(Some(SettingsState::Closed)));
        }
        Interaction::Hovered => {
            *background_color = BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 1.0));
        }
        Interaction::None => {
            *background_color = BackgroundColor(Color::rgba(1.0, 1.0, 1.0, 0.8));
        }
    }
}

fn despawn_settings_menu(
    mut commmands: Commands,
    settings_menu: Query<Entity, With<SettingsMenu>>,
) {
    if let Ok(settings_entity) = settings_menu.get_single() {
        commmands.entity(settings_entity).despawn_recursive();
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_settings_menu.in_schedule(OnEnter(SettingsState::Open)))
            .add_system(despawn_settings_menu.in_schedule(OnEnter(SettingsState::Closed)))
            .add_system(interact_back_button)
            .add_plugin(InputPlugin);
        //     .add_systems(
        //         (
        //             interact_exit_button,
        //             interact_play_button,
        //             interact_settings_button,
        //         )
        //             .in_set(OnUpdate(AppState::MainMenu)),
        //     );
    }
}
