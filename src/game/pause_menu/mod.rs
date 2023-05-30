mod button;

use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;

use crate::{
    utils::interact_button::{self, *},
    AppState, SettingsState,
};

use super::PausedState;

use button::*;

pub struct PauseMenuPlugin;

#[derive(Component)]
struct PauseMenu;

#[derive(Component)]
struct ContinueButton;

impl HoverButton for ContinueButton {
    fn on_click(commands: &mut Commands) {
        commands.insert_resource(NextState(Some(PausedState::Playing)));
    }
}

#[derive(Component)]
struct SettingsButton;

impl HoverButton for SettingsButton {
    fn on_click(commands: &mut Commands) {
        commands.insert_resource(NextState(Some(SettingsState::Open)));
    }
}

#[derive(Component)]
struct MainMenuButton;

impl HoverButton for MainMenuButton {
    fn on_click(commands: &mut Commands) {
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
    }
}

fn create_pause_menu(
    mut commands: Commands,
    pause_menu: Query<Entity, With<PauseMenu>>,
    asset_server: Res<AssetServer>,
) {
    if pause_menu.iter().len() > 0 {
        return;
    };

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
            PauseMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.5)),
                    style: Style {
                        size: Size::new(Val::Auto, Val::Percent(100.0)),
                        padding: UiRect::horizontal(Val::Px(15.0)),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Paused",
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

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                size: Size::all(Val::Percent(100.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::horizontal(Val::Px(10.0)),
                                gap: Size::height(Val::Px(10.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            create_pause_menu_button(
                                parent,
                                &asset_server,
                                "continue",
                                ContinueButton,
                            );

                            create_pause_menu_button(
                                parent,
                                &asset_server,
                                "settings",
                                SettingsButton,
                            );

                            create_pause_menu_button(
                                parent,
                                &asset_server,
                                "main menu",
                                MainMenuButton,
                            );
                        });
                });
        });
}

fn despawn_pause_menu(mut commands: Commands, pause_menu: Query<Entity, With<PauseMenu>>) {
    if let Ok(pause_menu_entity) = pause_menu.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

fn toggle_pause(
    mut commands: Commands,
    paused: Res<State<PausedState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        if paused.0 == PausedState::Paused {
            commands.insert_resource(NextState(Some(PausedState::Playing)));
            commands.insert_resource(NextState(Some(SettingsState::Closed)));
        } else {
            commands.insert_resource(NextState(Some(PausedState::Paused)));
        }
    }
}

fn pause_sim(paused: Res<State<PausedState>>, mut simulation_state: ResMut<RapierConfiguration>) {
    if paused.0 == PausedState::Paused {
        simulation_state.physics_pipeline_active = false;
    } else {
        simulation_state.physics_pipeline_active = true;
    }
}

fn hide_on_settings_open(mut pause_menu: Query<&mut Visibility, With<PauseMenu>>) {
    *pause_menu.single_mut() = Visibility::Hidden;
}

fn reveal_on_settings_close(mut pause_menu: Query<&mut Visibility, With<PauseMenu>>) {
    *pause_menu.single_mut() = Visibility::Visible;
}

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_pause_menu.in_schedule(OnEnter(PausedState::Paused)))
            .add_system(
                create_pause_menu
                    .in_schedule(OnEnter(AppState::InGame))
                    .run_if(in_state(PausedState::Paused)),
            )
            .add_system(despawn_pause_menu.in_schedule(OnExit(PausedState::Paused)))
            .add_system(despawn_pause_menu.in_schedule(OnExit(AppState::InGame)))
            .add_systems(
                (
                    interact_button::interact_system::<ContinueButton>,
                    interact_button::interact_system::<SettingsButton>,
                    interact_button::interact_system::<MainMenuButton>,
                )
                    .in_set(OnUpdate(PausedState::Paused)),
            )
            .add_systems((toggle_pause, pause_sim).in_set(OnUpdate(AppState::InGame)))
            .add_system(
                hide_on_settings_open
                    .in_schedule(OnEnter(SettingsState::Open))
                    .run_if(in_state(AppState::InGame)),
            )
            .add_system(
                reveal_on_settings_close
                    .in_schedule(OnExit(SettingsState::Open))
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
