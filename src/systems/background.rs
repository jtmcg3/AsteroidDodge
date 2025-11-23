use bevy::prelude::*;

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("background.png")),
        // Set z-index to -1 to ensure it's behind everything
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
}
