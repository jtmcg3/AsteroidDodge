# Audio System Refactor Plan

## Overview

Expand the current thruster-only audio system to support multiple sound types with a centralized, event-driven architecture.

## Current State

- Single `ThrusterAudio` marker component
- Audio loaded inline during player spawning (`src/systems/player.rs`)
- Audio entity spawned as child of player
- `update_thruster_audio` system controls playback via `AudioSink`
- Looping sound with pause/play based on keyboard input

## Target Sound Types

| Sound | Trigger | Playback |
|-------|---------|----------|
| Thruster | Movement keys held | Loop (pause/play) |
| Explosion | Projectile hits asteroid | One-shot (DESPAWN) |
| Bonk | Player-asteroid collision (survives) | One-shot (DESPAWN) |
| Laser | Projectile fired | One-shot (DESPAWN) |
| GameStart | Transition to Playing state | One-shot (DESPAWN) |
| GameOver | Player death | One-shot (DESPAWN) |
| MenuBoop | Menu interaction | One-shot (DESPAWN) |
| Background Music | State-based | Loop (separate handler) |

## Architecture Decisions

### Event-Driven Audio (Pattern B)

Game systems emit events; a dedicated audio system handles playback.

```
collision system emits PlaySoundEvent::Explosion
    → audio system reads events
    → spawns AudioPlayer entity with appropriate handle
```

**Rationale**: Clean separation of concerns, all audio spawning centralized.

### Centralized AudioAssets Resource

A single `Resource` holds all `Handle<AudioSource>` fields, populated during a Loading state.

**Why Resource, not Component**:
- Global singleton data (one collection for the whole game)
- Not attached to any specific entity
- Multiple systems need access

### Loading State

Add `AppState::Loading` as the initial state:
```
Loading → MainMenu → Playing → GameOver → MainMenu
```

Two-part loading pattern:
1. **OnEnter(Loading)**: Kick off asset loading, store handles in AudioAssets
2. **Update (in Loading)**: Check if assets ready, transition to MainMenu

## Implementation Order

1. Add `Loading` variant to `AppState` enum
2. Create `src/events.rs` with `PlaySoundEvent` enum
3. Create `AudioAssets` resource in `resources.rs`
4. Create loading systems (populate AudioAssets, transition when ready)
5. Create audio event handler system (reads events, spawns AudioPlayer)
6. Wire `PlaySoundEvent` emissions into collision and other systems
7. (Optional) Refactor thruster audio to use AudioAssets for consistency

## File Organization

| Item | Location |
|------|----------|
| `PlaySoundEvent` enum | `src/events.rs` (new) |
| `AudioAssets` resource | `src/resources.rs` |
| Loading systems | `src/main.rs` or `src/systems/loading.rs` |
| Audio event handler | `src/systems/audio.rs` (new) |

## Sound Placement in Collision Handler

```rust
// In handle_collision (player-asteroid)
if health.is_dead() {
    // Emit PlaySoundEvent::Explosion (ship explodes)
    // Emit PlaySoundEvent::GameOver
    // Despawn player, transition to GameOver state
} else {
    // Emit PlaySoundEvent::Bonk (player survived hit)
}
```

## Background Music (Separate Concern)

Background music requires different handling:
- Track what's currently playing
- Separate volume control from SFX
- Track selection/switching logic
- Consider event-based (`PlayMusicEvent::MainTheme`) or resource with methods

## Open Questions

- Should `GameOver` sound be emitted by collision system or by the state transition handler?
- Spatial audio for positional sounds (future consideration)?
- Volume balancing between SFX and music?
