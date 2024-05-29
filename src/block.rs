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
    pub direction: Direction,
}

impl Default for HandBlock {
    fn default() -> Self {
        Self {
            index: 15,
            direction: Direction::default(),
        }
    }
}

// 方向
pub enum Direction {
    Left,
    Down,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Left
    }
}

#[derive(Event)]
struct BlockDownEvent {
    pub remove_block_pos: Vec2,
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<HandBlockState>()
            .add_event::<BlockDownEvent>()
            .add_systems(
                Update,
                (
                    handle_hand_block_move.run_if(in_state(HandBlockState::Moving)),
                    handle_block_movement.run_if(in_state(HandBlockState::Idle)),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                handle_block_down.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                OnExit(HandBlockState::Moving),
                (
                    handle_block_remove,
                    handle_block_change,
                    handle_reset_hand_block,
                ),
            );
    }
}

// 处理方块消除
fn handle_block_remove(
    mut commands: Commands,
    mut events: EventWriter<BlockDownEvent>,
    mut query: Query<(&mut Transform, &mut Block, Entity), With<Block>>,
    hand_block_query: Query<&Transform, (With<HandBlock>, Without<Block>)>,
) {
    if query.is_empty() || hand_block_query.is_empty() {
        return;
    }

    // 消除方块
    for (transform, block, entity) in query.iter_mut() {
        if !block.show {
            commands.entity(entity).despawn();

            // 触发方块下落事件
            events.send(BlockDownEvent {
                remove_block_pos: transform.translation.truncate(),
            });
        }
    }
}

// 处理方块下落
fn handle_block_down(
    mut events: EventReader<BlockDownEvent>,
    mut query: Query<(&mut Transform, &Block), With<Block>>,
) {
    if query.is_empty() {
        return;
    }

    for e in events.read() {
        let remove_block_pos = e.remove_block_pos;
        for (mut transform, block) in query.iter_mut() {
            if block.show
                && transform.translation.x == remove_block_pos.x
                && transform.translation.y >= remove_block_pos.y
            {
                transform.translation.y -= STEP_SIZE as f32;
            }
        }
    }
}

// 处理手里方块移动
fn handle_hand_block_move(
    time: Res<Time>,
    mut hand_block_query: Query<(&mut Transform, &HandBlock), With<HandBlock>>,
) {
    if hand_block_query.is_empty() {
        return;
    }

    let (mut hand_block_transform, hand_block) = hand_block_query.single_mut();
    match hand_block.direction {
        Direction::Left => {
            hand_block_transform.translation.x -= HAND_BLOCK_SPEED * time.delta_seconds();
        }
        Direction::Down => {
            hand_block_transform.translation.y -= HAND_BLOCK_SPEED * time.delta_seconds();
        }
    }
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

// 重置手里方块
fn handle_reset_hand_block(mut query: Query<&mut HandBlock, With<HandBlock>>) {
    if query.is_empty() {
        return;
    }

    let mut hand_block = query.single_mut();

    hand_block.direction = Direction::Left;
}
