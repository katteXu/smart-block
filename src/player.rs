use std::time::Duration;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::time::Stopwatch;

use crate::state::GameState;
use crate::*;

// Player
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ActionTimer(pub Stopwatch);

#[derive(Component)]
pub struct HandBlock {
    pub index: usize,
}

#[derive(Component, Default, PartialEq, Eq)]
pub enum PlayerState {
    #[default]
    Idle,
    Moving,
    Throwing,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Copy, States)]
pub enum HandBlockState {
    #[default]
    Idle,
    Moving,
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<HandBlockState>()
            .add_systems(
                Update,
                (
                    (handle_player_movement, handle_block_movement)
                        .run_if(in_state(HandBlockState::Idle)),
                    handle_throw_block,
                    hand_block_move
                        .run_if(in_state(HandBlockState::Moving))
                        .run_if(on_timer(Duration::from_secs_f32(0.02))),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(HandBlockState::Moving), handle_change_hand_block);
    }
}

// 玩家移动
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

// 方块跟随玩家移动
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
    hand_block_transform.translation.x = player_transform.translation.x - STEP_SIZE as f32;
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

// 移动方块
fn hand_block_move(mut hand_block_query: Query<&mut Transform, With<HandBlock>>) {
    if hand_block_query.is_empty() {
        return;
    }

    let mut hand_block_transform = hand_block_query.single_mut();

    hand_block_transform.translation.x -= 32.0;
}

fn handle_change_hand_block(mut query: Query<(&mut TextureAtlas, &HandBlock), With<HandBlock>>) {
    if query.is_empty() {
        return;
    }

    let (mut texture_atlas, hand_block) = query.single_mut();

    texture_atlas.index = hand_block.index as usize;
}
