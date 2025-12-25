use crate::events::*;
use crate::resources::AppState;
use bevy::prelude::*;

// This spawns the GameOver UI
pub fn setup_game_over(mut commands: Commands) {
    // spawn a root node that covers the screen, flexbox container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            DespawnOnExit(AppState::GameOver),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Have you tried...\nbeing better?\nPress <Enter> ya filthy animal"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    // Add text children for title and instructions
    // Make sure to include StateScoped(AppState:GameOver) in root
}

// game_over
// input, runs every frame while in game_over
// state
pub fn handle_game_over_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut message: MessageWriter<PlaySoundEvent>,
) {
    // Check for Enter Key, Transition to Menu
    if keyboard.just_pressed(KeyCode::Enter) {
        message.write(PlaySoundEvent::MenuBoop); // this will also play when entering name
        next_state.set(AppState::Menu);
    }
}
