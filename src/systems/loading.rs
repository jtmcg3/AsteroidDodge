use bevy::prelude::*;
use crate::resources::{AudioAssets, AppState};

pub fn setup_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(
        AudioAssets {
            thruster: asset_server.load("audio/kenney_sci-fi-sounds/Audio/thrusterFire_004.ogg"),
            explosion: asset_server.load("audio/kenney_sci-fi-sounds/Audio/explosionCrunch_000.ogg"),
            bonk: asset_server.load("audio/kenney_sci-fi-sounds/Audio/forceField_001.ogg"),
            laser: asset_server.load("audio/kenney_sci-fi-sounds/Audio/laserRetro_001.ogg"),
            game_start: asset_server.load("audio/kenney_sci-fi-sounds/Audio/toggle_003.ogg"),
            game_over: asset_server.load("audio/kenney_sci-fi-sounds/Audio/explosionCrunch_002.ogg"),
            menu_boop: asset_server.load("audio/kenney_sci-fi-sounds/Audio/glitch_004.ogg"),
        }
    );
}

pub fn check_loading(
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
    audio_assets: ResMut<AudioAssets>) {

    let all_loaded = audio_assets.handles().iter().all(|handle| {asset_server.is_loaded_with_dependencies(handle)});

    if all_loaded {
        next_state.set(AppState::Menu)
    } 

    
}
