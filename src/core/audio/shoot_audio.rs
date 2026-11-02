use bevy::prelude::*;

pub fn background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/background_sound.mp3")),
        PlaybackSettings::LOOP,
    ));
}
