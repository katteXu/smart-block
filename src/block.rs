use bevy::math::vec3;
use bevy::prelude::*;
use std::collections::HashSet;

use crate::gui::{ClearNum, Score};
use crate::player::{Ladder, Player};
use crate::resources::GlobalAudio;
use crate::state::{BlockGroupState, GameState, HandBlockState, SettlementState};
use crate::wall::Wall;
use crate::*;

use self::alert::AlertEvent;

// Block
#[derive(Component, Debug)]
pub struct Block {
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
            fall_down_timer: Timer::from_seconds(FALL_DOWN_TIMER, TimerMode::Once),
        }
    }
}
#[derive(Event)]
struct NoRemoveEvent;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<HandBlockState>()
            .init_state::<BlockGroupState>()
            .init_resource::<RemoveBlockResource>()
            .add_event::<NoRemoveEvent>()
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
                (handle_block_remove, handle_reset_hand_block),
            )
            .add_systems(
                Update,
                handle_block_fall_down.run_if(in_state(BlockGroupState::FallDown)),
            )
            .add_systems(OnExit(BlockGroupState::FallDown), block_fall_down_sound)
            .add_systems(Update, handle_game_over.run_if(in_state(GameState::InGame)))
            .add_systems(
                OnExit(HandBlockState::Backing),
                handle_no_remove_block_by_player.run_if(in_state(GameState::InGame)),
            );
    }
}

// 处理方块消除
fn handle_block_remove(
    mut commands: Commands,
    audio_handles: Res<GlobalAudio>,
    mut score: ResMut<Score>,
    mut remove_block_resource: ResMut<RemoveBlockResource>,
    mut query: Query<(&mut Transform, &mut Block, Entity), With<Block>>,
    hand_block_query: Query<&Transform, (With<HandBlock>, Without<Block>)>,
    mut next_state: ResMut<NextState<BlockGroupState>>,
) {
    if query.is_empty() || hand_block_query.is_empty() {
        return;
    }

    // 消除块数
    let mut remove_blocks = vec![];

    // 消除方块
    for (transform, block, entity) in query.iter_mut() {
        if !block.show {
            commands.entity(entity).despawn();

            remove_blocks.push(RemoveBlock {
                pos: transform.translation.truncate(),
            });

            // 生成消除声效
            if let Some(hand_block_hit_block_sound) = audio_handles.hand_block_hit_block.clone() {
                commands.spawn(AudioBundle {
                    source: hand_block_hit_block_sound,
                    ..default()
                });
            }

            next_state.set(BlockGroupState::FallDown);
        }
    }
    // 分数计算
    score.once_remove_block = remove_blocks.len() as u32;

    // 移除方块resource
    if remove_blocks.len() > 0 {
        remove_block_resource.blocks = Some(remove_blocks);
    } else {
        remove_block_resource.blocks = None;
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

// 重置手里方块移动方向
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

                    transform.translation.y -=
                        STEP_SIZE as f32 * time.delta_seconds() / FALL_DOWN_TIMER;

                    if finished {
                        // 下移完成，重置方块的y值
                        transform.translation.y = block.pos.y;
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

// 处理游戏结束
fn handle_game_over(
    mut no_remove_event: EventReader<NoRemoveEvent>,
    mut alert_event: EventWriter<AlertEvent>,
    mut hand_block_query: Query<(&mut Transform, &mut TextureAtlas), With<HandBlock>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<HandBlock>)>,
    block_query: Query<(), With<Block>>,
    clear_query: Query<&Text, With<ClearNum>>,
    mut next_state: ResMut<NextState<SettlementState>>,
) {
    if block_query.is_empty()
        || clear_query.is_empty()
        || hand_block_query.is_empty()
        || player_query.is_empty()
    {
        return;
    }

    let mut player_transform = player_query.single_mut();
    let (mut hand_block_transform, mut hand_block_atlas) = hand_block_query.single_mut();

    let block_number = block_query.iter().count();
    let clear_number = clear_query.single().sections[0]
        .value
        .parse::<usize>()
        .unwrap();

    for _e in no_remove_event.read() {
        // 如果方块数量小于等于消除数量，则获胜
        if block_number <= clear_number {
            println!("游戏胜利");

            // 开始结算
            next_state.set(SettlementState::Start);
        } else {
            alert_event.send(AlertEvent(Some(String::from(
                "Can't Remove\nGive You A LightningBlock.",
            ))));
            let (player_x, player_y) = PLAYER_INIT_POS;
            player_transform.translation = vec3(player_x, player_y, 1.0);
            hand_block_transform.translation = vec3(player_x - STEP_SIZE as f32, player_y, 1.0);
            hand_block_atlas.index = LIGHT_BLOCK_INDEX;
        }
    }
}

// 判断是否有方块可以消除
fn handle_no_remove_block_by_player(
    mut no_remove_event: EventWriter<NoRemoveEvent>,
    ladder_query: Query<&Transform, With<Ladder>>,
    block_query: Query<(&Transform, &Block, &TextureAtlas), With<Block>>,
    wall_query: Query<&Transform, (With<Wall>, Without<Block>)>,
    hand_block_query: Query<&TextureAtlas, With<HandBlock>>,
) {
    if block_query.is_empty() || wall_query.is_empty() || ladder_query.is_empty() {
        return;
    }

    let hand_block_atlas = hand_block_query.single();

    let res_index = get_target_block_by_player_position(ladder_query, block_query, wall_query);

    if res_index.contains(&hand_block_atlas.index) {
        // 有可消除的方块
    } else {
        no_remove_event.send(NoRemoveEvent);
    }
}

// 根据玩家位置判断目标方块
fn get_target_block_by_player_position(
    ladder_query: Query<&Transform, With<Ladder>>,
    block_query: Query<(&Transform, &Block, &TextureAtlas), With<Block>>,
    wall_query: Query<&Transform, (With<Wall>, Without<Block>)>,
) -> HashSet<usize> {
    let mut res_hashset = HashSet::new();

    for ladder_transform in ladder_query.iter() {
        let ladder_position = ladder_transform.translation.truncate();
        let mut res_index = None;

        // 基于y
        let base_y = ladder_position.y;
        // 基于 block
        let mut base_block = false;

        // 方块坐标
        let mut block_x = None;
        let mut block_y = None;

        // 方块位置
        for (block_transform, _block, _) in block_query.iter() {
            if base_y == block_transform.translation.y {
                block_y = Some(block_transform.translation.y);

                if let Some(x) = block_x {
                    block_x = Some(block_transform.translation.x.max(x));
                } else {
                    block_x = Some(block_transform.translation.x);
                }

                base_block = true;
            }
        }

        if !base_block {
            let mut wall_x = None;
            let mut _wall_y = None;
            // 获取墙体位置
            for wall_transform in wall_query.iter() {
                // wall_transform.translation.x < 0.0 表示墙体在左侧
                if base_y == wall_transform.translation.y && wall_transform.translation.x < 0.0 {
                    _wall_y = Some(wall_transform.translation.y);

                    if let Some(x) = wall_x {
                        wall_x = Some(wall_transform.translation.x.max(x));
                    } else {
                        wall_x = Some(wall_transform.translation.x);
                    }
                }
            }

            // 基于x
            let base_x = wall_x.map(|x| x + STEP_SIZE as f32).unwrap_or(0.0);

            // 遍历所有方块，获取最外层方块的坐标
            for (block_transform, _, _) in block_query.iter() {
                if base_x == block_transform.translation.x {
                    block_x = Some(block_transform.translation.x);

                    if let Some(y) = block_y {
                        block_y = Some(block_transform.translation.y.max(y));
                    } else {
                        block_y = Some(block_transform.translation.y);
                    }
                }
            }
        }
        // 获取最外层方块
        for (block_transform, _, texture_atlas) in block_query.iter() {
            match (block_x, block_y) {
                (Some(x), Some(y)) => {
                    if block_transform.translation.x == x && block_transform.translation.y == y {
                        res_index = Some(texture_atlas.index)
                    }
                }
                _ => {}
            }
        }

        if let Some(index) = res_index {
            res_hashset.insert(index);
        }
    }

    return res_hashset;
}

// 播放下落方块音效
fn block_fall_down_sound(audio_handles: Res<GlobalAudio>, mut commands: Commands) {
    if let Some(block_fall_down_sound) = audio_handles.block_fall_down.clone() {
        commands.spawn(AudioBundle {
            source: block_fall_down_sound,
            ..default()
        });
    }
}
