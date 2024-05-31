use std::collections::HashSet;

use bevy::math::vec2;
use bevy::prelude::*;

use crate::state::{BlockGroupState, GameState, HandBlockState};
use crate::*;

use crate::player::Player;

use crate::gui::TextScore;

// Block
#[derive(Component, Debug)]
pub struct Block {
    pub index: usize,
    pub show: bool,
    pub pos: Vec2,
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

// 手里方块移动方向
// 默认向左 碰触墙壁则向下
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

#[derive(Debug)]
pub struct RemoveBlock {
    pub pos: Vec2,
}

#[derive(Resource)]
pub struct RemoveBlockResource {
    blocks: Option<Vec<RemoveBlock>>,
    fall_down_timer: Timer,
}
impl Default for RemoveBlockResource {
    fn default() -> Self {
        Self {
            blocks: None,
            fall_down_timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<HandBlockState>()
            .init_state::<BlockGroupState>()
            .init_resource::<RemoveBlockResource>()
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
                (handle_block_down).run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                OnExit(HandBlockState::Moving),
                (
                    handle_block_remove,
                    // handle_block_change,
                    handle_reset_hand_block,
                ),
            )
            .add_systems(
                Update,
                handle_block_fall_down.run_if(in_state(BlockGroupState::FallDown)),
            );
    }
}

// 处理方块消除
fn handle_block_remove(
    mut commands: Commands,
    mut remove_block_resource: ResMut<RemoveBlockResource>,
    mut score_query: Query<&mut TextScore, With<TextScore>>,
    mut events: EventWriter<BlockDownEvent>,
    mut query: Query<(&mut Transform, &mut Block, Entity), With<Block>>,
    hand_block_query: Query<&Transform, (With<HandBlock>, Without<Block>)>,
    mut next_state: ResMut<NextState<BlockGroupState>>,
) {
    if query.is_empty() || hand_block_query.is_empty() || score_query.is_empty() {
        return;
    }
    let mut text_score = score_query.single_mut();
    // 消除块数
    let mut remove_blocks = vec![];

    // 消除方块
    for (transform, block, entity) in query.iter_mut() {
        if !block.show {
            commands.entity(entity).despawn();

            remove_blocks.push(RemoveBlock {
                pos: transform.translation.truncate(),
            });
            // 触发方块下落事件
            // events.send(BlockDownEvent {
            //     remove_block_pos: transform.translation.truncate(),
            // });

            next_state.set(BlockGroupState::FallDown);
        }
    }
    // 分数计算
    text_score.once_remove_block = remove_blocks.len() as u32;

    // 移除方块resource
    if remove_blocks.len() > 0 {
        remove_block_resource.blocks = Some(remove_blocks);
    } else {
        remove_block_resource.blocks = None;
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
        // for (mut transform, block) in query.iter_mut() {
        //     if block.show
        //         && transform.translation.x == remove_block_pos.x
        //         && transform.translation.y >= remove_block_pos.y
        //     {
        //         transform.translation.y -= STEP_SIZE as f32;
        //     }
        // }
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

// 重置手里方块方向
fn handle_reset_hand_block(mut query: Query<&mut HandBlock, With<HandBlock>>) {
    if query.is_empty() {
        return;
    }

    let mut hand_block = query.single_mut();

    hand_block.direction = Direction::Left;
}

// 下落动画
fn handle_block_fall_down(
    time: Res<Time>,
    mut remove_block_resource: ResMut<RemoveBlockResource>,
    mut next_state: ResMut<NextState<BlockGroupState>>,
    mut query: Query<(&mut Transform, &mut Block), With<Block>>,
) {
    if remove_block_resource.blocks.is_none() {
        return;
    }
    remove_block_resource.fall_down_timer.tick(time.delta());
    let finished: bool = remove_block_resource.fall_down_timer.finished();

    if let Some(remove_blocks) = remove_block_resource.blocks.as_mut() {
        // 方块下落
        // 1. 获取到所有下落的方块
        // 2. 遍历所有方块，将所有下落的方块的上方方块下移
        for remove_block in remove_blocks {
            let pos = remove_block.pos;

            for (mut transform, mut block) in query.iter_mut() {
                if block.show
                    && transform.translation.x == pos.x
                    && transform.translation.y >= pos.y
                {
                    // 根据方块的y值判断是否需要下移
                    if transform.translation.y == block.pos.y {
                        // 将下移位置保存
                        block.pos.y -= STEP_SIZE as f32;
                    }

                    transform.translation.y -= STEP_SIZE as f32 * time.delta_seconds() * 10.0;

                    if finished {
                        // 下移完成，重置方块的y值
                        transform.translation.y = block.pos.y;
                        // transform.translation.y -= STEP_SIZE as f32
                    }
                }
            }
        }
    }

    // 下落完成
    if finished {
        remove_block_resource.fall_down_timer.reset();
        remove_block_resource.blocks = None;
        next_state.set(BlockGroupState::Static);
    }
}

// TODO: 处理没有可消除方块逻辑
fn handle_none_block(
    block_query: Query<(&Transform, &Block), With<Block>>,
    mut hand_block_query: Query<(&Transform, &HandBlock), (With<HandBlock>, Without<Block>)>,
) {
    if block_query.is_empty() || hand_block_query.is_empty() {
        return;
    }
    let (hb_t, hb_b) = hand_block_query.single_mut();

    let mut target_block_index = HashSet::new();
    let mut target_block_translation = Vec2::NEG_INFINITY;
    // 遍历所有方块，获取最外层方块的索引并添加到target_block_index中
    for (b_t, _) in block_query.iter() {
        let block_translation = b_t.translation;
        target_block_translation.x = block_translation.x.max(target_block_translation.x);
        target_block_translation.y = block_translation.y.max(target_block_translation.y);
    }
    for (b_t, b_b) in block_query.iter() {
        let block_translation = b_t.translation;
        if target_block_translation.x == block_translation.x
            || target_block_translation.y == block_translation.y
        {
            target_block_index.insert(b_b.index);
        }
    }

    if !target_block_index.contains(&hb_b.index) {
        // 方块在目标方块内部，重置手里方块
        println!(
            "target_block_index: {:?} 手里的是：{:?}",
            target_block_index, hb_b.index
        );
        println!("没有可消除的方块了");
    }
}
