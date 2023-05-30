use bevy::{app::AppExit, prelude::*};

use crate::{utils::interact_button, AppState, SettingsState};

pub struct MenuPlugin;

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct SettingsButton;

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
                        justify_content: JustifyContent::End,
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
                    parent.spawn((
                        ButtonBundle {
                            image: asset_server.load("menus/buttons/setting_icon.png").into(),
                            style: Style {
                                size: Size::new(Val::Auto, Val::Percent(100.0)),
                                aspect_ratio: Some(1.0 / 1.0),
                                ..default()
                            },
                            ..default()
                        },
                        SettingsButton,
                    ));
                });

            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(600.0), Val::Px(80.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(0.0), Val::Px(80.0)),
                    ..default()
                },
                image: asset_server.load("menus/Bombcakes.png").into(),
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
                            texture: asset_server.load("menus/buttons/blue_button.png"),
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
                            texture: asset_server.load("menus/buttons/red_button.png"),
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

impl interact_button::HoverButton for SettingsButton {
    fn on_click(commands: &mut Commands) {
        commands.insert_resource(NextState(Some(SettingsState::Open)));
    }
}

impl interact_button::HoverButton for PlayButton {
    fn on_click(commands: &mut Commands) {
        commands.insert_resource(NextState(Some(AppState::InGame)));
    }
    fn get_interaction_colors() -> interact_button::InteractionColors {
        interact_button::InteractionColors {
            hover_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            normal_color: Color::rgba(0.95, 0.95, 0.95, 1.0),
        }
    }
}

fn hide_on_settings_open(mut main_menu: Query<&mut Visibility, With<MainMenu>>) {
    *main_menu.single_mut() = Visibility::Hidden;
}

fn reveal_on_settings_close(mut main_menu: Query<&mut Visibility, With<MainMenu>>) {
    *main_menu.single_mut() = Visibility::Visible;
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(
                hide_on_settings_open
                    .in_schedule(OnEnter(SettingsState::Open))
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_system(
                reveal_on_settings_close
                    .in_schedule(OnExit(SettingsState::Open))
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(
                (
                    interact_exit_button,
                    interact_button::interact_system::<PlayButton>,
                    interact_button::interact_system::<SettingsButton>,
                )
                    .in_set(OnUpdate(AppState::MainMenu)),
            );
    }
}
