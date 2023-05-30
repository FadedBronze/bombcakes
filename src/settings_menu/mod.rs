use bevy::prelude::*;

use crate::{
    game_audio::GameAudioSettings,
    utils::interact_button::{self, HoverButton},
    SettingsState,
};

mod input_types;

use self::input_types::{
    slider::{create_slider, SliderDataController, SliderHandle},
    InputPlugin,
};

#[derive(Component)]
struct SettingsMenu;

#[derive(Component)]
struct BackButton;

impl HoverButton for BackButton {
    fn on_click(commands: &mut Commands) {
        commands.insert_resource(NextState(Some(SettingsState::Closed)));
    }
}

#[derive(Component)]
struct MasterVolumeSlider;

#[derive(Component)]
struct SFXVolumeSlider;

#[derive(Component)]
struct MusicVolumeSlider;

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
                            image: asset_server.load("menus/buttons/back_arrow.png").into(),
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
                            create_slider(
                                &asset_server,
                                parent,
                                "Master volume",
                                MasterVolumeSlider,
                            );
                            create_slider(&asset_server, parent, "Music volume", MusicVolumeSlider);
                            create_slider(&asset_server, parent, "SFX volume", SFXVolumeSlider);
                        });
                });
        });
}

fn update_slider_data<Data: Resource, Slider: Component + SliderDataController<Data>>(
    mut slider: Query<&mut SliderHandle, (Changed<SliderHandle>, With<Slider>)>,
    mut slider_controlling_data: ResMut<Data>,
) {
    if let Ok(mut data) = slider.get_single_mut() {
        if data.just_created {
            data.position = Slider::load_data(&slider_controlling_data);
            data.just_created = false;
        } else {
            Slider::save_data(&mut slider_controlling_data, data.position);
        }
    }
}

impl SliderDataController<GameAudioSettings> for MasterVolumeSlider {
    fn load_data(data: &GameAudioSettings) -> f32 {
        data.master as f32
    }
    fn save_data(data: &mut GameAudioSettings, position: f32) {
        data.master = position.into();
    }
}

impl SliderDataController<GameAudioSettings> for SFXVolumeSlider {
    fn load_data(data: &GameAudioSettings) -> f32 {
        data.sfx as f32
    }
    fn save_data(data: &mut GameAudioSettings, position: f32) {
        data.sfx = position.into();
    }
}

impl SliderDataController<GameAudioSettings> for MusicVolumeSlider {
    fn load_data(data: &GameAudioSettings) -> f32 {
        data.music as f32
    }
    fn save_data(data: &mut GameAudioSettings, position: f32) {
        data.music = position.into();
    }
}

fn despawn_settings_menu(mut commands: Commands, settings_menu: Query<Entity, With<SettingsMenu>>) {
    if let Ok(settings_entity) = settings_menu.get_single() {
        commands.entity(settings_entity).despawn_recursive();
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_settings_menu.in_schedule(OnEnter(SettingsState::Open)))
            .add_system(despawn_settings_menu.in_schedule(OnEnter(SettingsState::Closed)))
            .add_systems((
                interact_button::interact_system::<BackButton>,
                update_slider_data::<GameAudioSettings, MasterVolumeSlider>,
                update_slider_data::<GameAudioSettings, SFXVolumeSlider>,
                update_slider_data::<GameAudioSettings, MusicVolumeSlider>,
            ))
            .add_plugin(InputPlugin);
    }
}
