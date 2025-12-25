use crate::resources::*;
use bevy::prelude::*;

#[derive(Event, Message)]
pub enum PlaySoundEvent {
    Explosion,
    Bonk,
    Laser,
    GameStart,
    GameOver,
    MenuBoop,
}

pub fn handle_audio_events(
    mut events: MessageReader<PlaySoundEvent>,
    audio_assets: Res<AudioAssets>,
    mut commands: Commands,
) {
    for event in events.read() {
        match event {
            PlaySoundEvent::Explosion => commands.spawn((
                AudioPlayer(audio_assets.explosion.clone()),
                PlaybackSettings::DESPAWN,
            )),
            PlaySoundEvent::Bonk => commands.spawn((
                AudioPlayer(audio_assets.bonk.clone()),
                PlaybackSettings::DESPAWN,
            )),
            PlaySoundEvent::Laser => commands.spawn((
                AudioPlayer(audio_assets.laser.clone()),
                PlaybackSettings::DESPAWN,
            )),
            PlaySoundEvent::GameStart => commands.spawn((
                AudioPlayer(audio_assets.game_start.clone()),
                PlaybackSettings::DESPAWN,
            )),
            PlaySoundEvent::GameOver => commands.spawn((
                AudioPlayer(audio_assets.game_over.clone()),
                PlaybackSettings::DESPAWN,
            )),
            PlaySoundEvent::MenuBoop => commands.spawn((
                AudioPlayer(audio_assets.menu_boop.clone()),
                PlaybackSettings::DESPAWN,
            )),
        };
    }
}
