use bevy::math::vec3;
use bevy::prelude::*;

use crate::state::{GameState, HandBlockState, PlayerState};
use crate::*;

use self::arrow::ArrowPlugin;

// Player
#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_player_movement.run_if(in_state(HandBlockState::Idle)),
                handle_throw_block,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_plugins(ArrowPlugin);
    }
}

// 玩家移动
fn handle_player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut PlayerState), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut player_transform, mut player_state) = player_query.single_mut();

    let w_key =
        keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp);

    let s_key = keyboard_input.just_pressed(KeyCode::KeyS)
        || keyboard_input.just_pressed(KeyCode::ArrowDown);

    let mut delta = Vec3::ZERO;

    // 只有上下操作
    if w_key && player_transform.translation.y <= -PLAYER_INIT_POS.1 - STEP_SIZE as f32 {
        delta.y += 1.0;
    }
    if s_key && player_transform.translation.y > PLAYER_INIT_POS.1 {
        delta.y -= 1.0;
    }

    let delta = delta.normalize();

    if delta.is_finite() && (w_key || s_key) {
        player_transform.translation += vec3(delta.x, delta.y, 0.0) * (STEP_SIZE as f32);
        *player_state = PlayerState::Moving;
    }
}

// 扔方块
fn handle_throw_block(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<HandBlockState>>,
) {
    let space_key = keyboard_input.just_pressed(KeyCode::Space);

    if space_key {
        next_state.set(HandBlockState::Moving);
    }
}
