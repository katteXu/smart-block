use bevy::math::vec3;
use bevy::prelude::*;

use crate::arrow::ArrowPlugin;
use crate::resources::GlobalAudio;
use crate::state::{GameState, HandBlockState, PlayerState};
use crate::*;

// Player
#[derive(Component)]
pub struct Player;

// 梯子
#[derive(Component)]
pub struct Ladder;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    handle_player_movement,
                    player_move_sound.run_if(has_user_input_up_or_down),
                    player_throw_sound.run_if(has_user_input_space),
                )
                    .run_if(in_state(HandBlockState::Idle)),
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
    mut player_query: Query<&mut PlayerState, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<HandBlockState>>,
) {
    if player_query.is_empty() || !keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    let mut player_state = player_query.single_mut();

    let space_key = keyboard_input.just_pressed(KeyCode::Space);

    if space_key {
        next_state.set(HandBlockState::Moving);
        *player_state = PlayerState::Throwing;
    }
}

// 播放玩家移动音效
fn player_move_sound(audio_handles: Res<GlobalAudio>, mut commands: Commands) {
    if let Some(player_move_source) = audio_handles.player_move.clone() {
        commands.spawn(AudioBundle {
            source: player_move_source,
            ..default()
        });
    }
}

//  播放玩家抛出音效
fn player_throw_sound(audio_handles: Res<GlobalAudio>, mut commands: Commands) {
    if let Some(player_throw_source) = audio_handles.player_throw.clone() {
        commands.spawn(AudioBundle {
            source: player_throw_source,
            ..default()
        });
    }
}

// 玩家是否按下了上下方向键
pub fn has_user_input_up_or_down(keyboard_input: Res<ButtonInput<KeyCode>>) -> bool {
    let arrow_up =
        keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW);

    let arrow_down = keyboard_input.just_pressed(KeyCode::ArrowDown)
        || keyboard_input.just_pressed(KeyCode::KeyS);

    return arrow_up || arrow_down;
}

// 玩家是否按下了空格键
pub fn has_user_input_space(keyboard_input: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard_input.just_pressed(KeyCode::Space)
}
