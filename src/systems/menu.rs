use crate::events::*;
use crate::resources::AppState;
use bevy::prelude::*;

// This spawns the Menu UI
pub fn setup_menu(mut commands: Commands) {
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
            DespawnOnExit(AppState::Menu),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Begin Your Game!\nPress <Enter>"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    // Add text children for title and instructions
    // Make sure to include StateScoped(AppState:Menu) in root
}

// menu input, runs every frame while in menu state
pub fn handle_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut message: MessageWriter<PlaySoundEvent>,
) {
    // Check for Enter Key, transition to playing
    if keyboard.just_pressed(KeyCode::Enter) {
        message.write(PlaySoundEvent::MenuBoop); // this will move when i have multiple ships to select
        message.write(PlaySoundEvent::GameStart); // this belongs here
        next_state.set(AppState::Playing);
    }
}
