use crate::components::{Health, Player};
use crate::events::*;
use crate::resources::AppState;
use crate::systems::collision::spawn_explosion;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub fn handle_health_message(
    mut events: MessageReader<DamageEvent>,
    // mut health_message: MessageWriter<HealthChanged>,
    mut death_message: MessageWriter<DeathEvent>,
    mut audio_message: MessageWriter<PlaySoundEvent>,
    mut player_query: Query<&mut Health, With<Player>>,
) {
    for event in events.read() {
        let mut health = player_query.single_mut().unwrap();
        health.damage(event.source_type.damage());

        if health.is_dead() {
            audio_message.write(PlaySoundEvent::GameOver);
            death_message.write(DeathEvent {
                player: event.player,
                position: event.position,
            });
        } else {
            audio_message.write(PlaySoundEvent::Bonk);
            // health_message.write(HealthChanged {
            //     player: event.player,
            //     new_health: health.current(),
            //     position: event.position,
            // });
        }
    }
}

pub fn handle_death_message(
    mut events: MessageReader<DeathEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    for event in events.read() {
        commands.entity(event.player).despawn();
        spawn_explosion(&mut commands, &mut effects, event.position);
        next_state.set(AppState::GameOver);
        //despawn
        // transition to GameOver
    }
}
