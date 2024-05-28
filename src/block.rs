use bevy::prelude::*;

use crate::state::{GameState, HandBlockState};
use crate::*;

use crate::player::Player;

// Block
#[derive(Component, Debug)]
pub struct Block {
    pub index: usize,
    pub show: bool,
}

#[derive(Component)]
pub struct HandBlock {
    pub index: usize,
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<HandBlockState>()
            .add_systems(
                Update,
                (
                    handle_hand_block_move.run_if(in_state(HandBlockState::Moving)),
                    handle_block_movement.run_if(in_state(HandBlockState::Idle)),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                OnExit(HandBlockState::Moving),
                (handle_block_down, handle_block_change),
            );
    }
}

// 处理方块消除 并 下落
fn handle_block_down(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Block, Entity), With<Block>>,
    hand_block_query: Query<&Transform, (With<HandBlock>, Without<Block>)>,
) {
    if query.is_empty() || hand_block_query.is_empty() {
        return;
    }

    let hand_block_transform = hand_block_query.single();
    let mut items_x = Vec::new();

    // 消除方块
    for (transform, block, entity) in query.iter() {
        if !block.show {
            items_x.push(transform.translation.x);
            commands.entity(entity).despawn();
        }
    }

    for (mut transform, _, _) in query.iter_mut() {
        if items_x.contains(&transform.translation.x)
            && transform.translation.y > hand_block_transform.translation.y
        {
            transform.translation.y -= STEP_SIZE as f32;
        }
    }
}

// 处理手里方块移动
fn handle_hand_block_move(
    time: Res<Time>,
    mut hand_block_query: Query<&mut Transform, With<HandBlock>>,
) {
    if hand_block_query.is_empty() {
        return;
    }

    let mut hand_block_transform = hand_block_query.single_mut();
    hand_block_transform.translation.x -= HAND_BLOCK_SPEED * time.delta_seconds();
}

// 处理方块变更
fn handle_block_change(
    mut hand_block_query: Query<(&mut TextureAtlas, &HandBlock), With<HandBlock>>,
    mut block_query: Query<(&mut TextureAtlas, &Block), (With<Block>, Without<HandBlock>)>,
) {
    if hand_block_query.is_empty() || block_query.is_empty() {
        return;
    }

    let (mut texture_atlas, hand_block) = hand_block_query.single_mut();

    texture_atlas.index = hand_block.index as usize;

    for (mut texture_atlas, block) in block_query.iter_mut() {
        texture_atlas.index = block.index as usize;
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
