use bevy::math::vec3;
use bevy::prelude::*;

use crate::state::GameState;
use crate::*;

// Player
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct HandBlock;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_player_movement, handle_block_movement).run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut player_transform = player_query.single_mut();

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

    let delta = delta.normalize_or_zero();

    player_transform.translation += vec3(delta.x, delta.y, 0.0) * (STEP_SIZE as f32);
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
