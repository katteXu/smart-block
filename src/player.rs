use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::state::GameState;
use crate::*;

// Player
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ActionTimer(pub Stopwatch);

#[derive(Component)]
pub struct HandBlock;

#[derive(Component, Default, PartialEq, Eq)]
pub enum PlayerState {
    #[default]
    Idle,
    Moving,
    Throwing,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_player_movement,
                handle_block_movement,
                hanle_throw_block,
                hand_block_move,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut PlayerState, &mut ActionTimer), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut player_transform, mut player_state, mut action_timer) = player_query.single_mut();

    let w_key =
        keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp);

    let s_key =
        keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);

    let mut delta = Vec3::ZERO;

    // 只有上下操作
    if w_key && player_transform.translation.y < -PLAYER_INIT_POS.1 - STEP_SIZE as f32 {
        delta.y += 1.0;
    }
    if s_key && player_transform.translation.y > PLAYER_INIT_POS.1 {
        delta.y -= 1.0;
    }

    let delta = delta.normalize();

    if delta.is_finite() && (w_key || s_key) {
        player_transform.translation += vec3(delta.x, delta.y, 0.0) * (STEP_SIZE as f32);
        *player_state = PlayerState::Moving;
    } else {
        match player_state.as_ref() {
            PlayerState::Moving => {
                action_timer.0.tick(time.delta());
                if action_timer.0.elapsed_secs() >= 0.3 {
                    action_timer.0.reset();
                    *player_state = PlayerState::Idle
                }
            }
            _ => (),
        }
    }
}

fn handle_block_movement(
    player_query: Query<&Transform, With<Player>>,
    mut hand_block_query: Query<&mut Transform, (With<HandBlock>, Without<Player>)>,
) {
    if player_query.is_empty() || hand_block_query.is_empty() {
        return;
    }

    let player_transform = player_query.single();

    let mut hand_block_transform = hand_block_query.single_mut();

    hand_block_transform.translation.y = player_transform.translation.y;
}

fn hanle_throw_block(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut PlayerState, &mut ActionTimer), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut player_state, mut action_timer) = player_query.single_mut();
    let space_key = keyboard_input.just_pressed(KeyCode::Space);

    if space_key {
        *player_state = PlayerState::Throwing;
    } else {
        match player_state.as_ref() {
            PlayerState::Throwing => {
                action_timer.0.tick(time.delta());
                if action_timer.0.elapsed_secs() >= 0.4 {
                    action_timer.0.reset();
                    *player_state = PlayerState::Idle
                }
            }
            _ => (),
        }
    }
}

fn hand_block_move(
    player_query: Query<&PlayerState, With<Player>>,
    mut hand_block_query: Query<&mut Transform, (With<HandBlock>, Without<Player>)>,
) {
    if player_query.is_empty() || hand_block_query.is_empty() {
        return;
    }

    let player_state = player_query.single();
    let mut hand_block_transform = hand_block_query.single_mut();
    if player_state == &PlayerState::Throwing {
        hand_block_transform.translation.x -= 16.0;
    } else {
        let (x, y) = PLAYER_INIT_POS;
        hand_block_transform.translation.x = x - STEP_SIZE as f32;
    }
}
