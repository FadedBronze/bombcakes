use bevy::prelude::*;
use bevy_kira_audio::*;

#[derive(Resource)]
struct MusicChannel;

#[derive(Resource)]
struct SFXChannel;

fn setup_music(asset_server: Res<AssetServer>, music_channel: Res<AudioChannel<MusicChannel>>) {
    let music_handle = asset_server.load("audio/Bombcakes.mp3");
    music_channel.play(music_handle).loop_from(11.0);
}

fn update_volume(
    game_audio_settings: Res<GameAudioSettings>,
    sfx: Res<AudioChannel<SFXChannel>>,
    music: Res<AudioChannel<MusicChannel>>,
) {
    if game_audio_settings.is_changed() || game_audio_settings.is_added() {
        sfx.set_volume(game_audio_settings.sfx * game_audio_settings.master);
        music.set_volume(game_audio_settings.music * game_audio_settings.master);
    }
}

#[derive(Resource, Debug)]
pub struct GameAudioSettings {
    pub master: f64,
    pub music: f64,
    pub sfx: f64,
}

pub(super) struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_music)
            .insert_resource(GameAudioSettings {
                master: 1.0,
                music: 1.0,
                sfx: 1.0,
            })
            .add_system(update_volume)
            .add_audio_channel::<MusicChannel>()
            .add_audio_channel::<SFXChannel>();
    }
}
