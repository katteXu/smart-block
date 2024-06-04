use bevy::math::vec3;
use bevy::{prelude::*, time::common_conditions::on_timer};
use kd_tree::{KdPoint, KdTree};
use std::time::Duration;

use crate::block::{Block, Direction, HandBlock};
use crate::player::Player;
use crate::resources::GlobalAudio;
use crate::state::{GameState, HandBlockState};
use crate::wall::{Ground, Wall};
use crate::*;

#[derive(Component, Debug)]
pub struct Collidable {
    pos: Vec2,
    entity: Entity,
}

impl KdPoint for Collidable {
    type Scalar = f32;

    type Dim = typenum::U2;

    fn at(&self, k: usize) -> Self::Scalar {
        if k == 0 {
            return self.pos.x;
        }
        self.pos.y
    }
}

// 方块树
#[derive(Resource)]
pub struct BlockKdTree(pub KdTree<Collidable>);

impl Default for BlockKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}

// 墙面树
#[derive(Resource)]
pub struct WallKdTree(pub KdTree<Collidable>);

impl Default for WallKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}

// 地面树
#[derive(Resource)]
pub struct GroundKdTree(pub KdTree<Collidable>);

impl Default for GroundKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}

// 贝塞尔曲线点
#[derive(Resource)]
pub struct BezierPoints(pub Option<[[Vec3; 4]; 1]>);
impl Default for BezierPoints {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Resource)]
pub struct CollisionBackTimer(Timer);
impl Default for CollisionBackTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0 / 3.0, TimerMode::Once))
    }
}

// 是否消除
#[derive(Resource)]
struct IsEliminate(bool);

// 闪电块首次碰触的方块
#[derive(Resource)]
struct LightFirstRemoveBlock(Option<usize>);
impl Default for LightFirstRemoveBlock {
    fn default() -> Self {
        Self(None)
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BezierPoints>()
            .init_resource::<CollisionBackTimer>()
            .init_resource::<LightFirstRemoveBlock>()
            .insert_resource(BlockKdTree::default())
            .insert_resource(WallKdTree::default())
            .insert_resource(GroundKdTree::default())
            .insert_resource(IsEliminate(false))
            .add_systems(
                OnEnter(GameState::InGame),
                (spawn_wall_kd_tree, spawn_ground_kd_tree),
            )
            .add_systems(
                Update,
                (
                    (
                        handle_block_collision,
                        handle_lightning_block_collision,
                        handle_block_wall_collision,
                        handle_block_ground_collision,
                    )
                        .run_if(in_state(HandBlockState::Moving)),
                    handle_collision_back_animation.run_if(in_state(HandBlockState::Backing)),
                    update_block_kd_tree
                        .run_if(in_state(HandBlockState::Idle))
                        .run_if(on_timer(Duration::from_secs_f32(KD_TREE_REFRESH_RATE))),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                OnEnter(HandBlockState::Backing),
                (
                    lighting_first_remove_block_index_reset,
                    hand_block_back_sound,
                ),
            );
    }
}

// 生成墙面kd tree
fn spawn_wall_kd_tree(
    mut tree: ResMut<WallKdTree>,
    wall_query: Query<(&Transform, Entity), With<Wall>>,
) {
    let mut items = Vec::new();

    for (t, e) in wall_query.iter() {
        items.push(Collidable {
            pos: t.translation.truncate(),
            entity: e,
        });
    }

    tree.0 = KdTree::build_by_ordered_float(items);
}

// 生成地面kd tree
fn spawn_ground_kd_tree(
    mut tree: ResMut<GroundKdTree>,
    ground_query: Query<(&Transform, Entity), With<Ground>>,
) {
    let mut items = Vec::new();

    for (t, e) in ground_query.iter() {
        items.push(Collidable {
            pos: t.translation.truncate(),
            entity: e,
        });
    }

    tree.0 = KdTree::build_by_ordered_float(items);
}

// 更新方块kd tree
fn update_block_kd_tree(
    mut tree: ResMut<BlockKdTree>,
    block_query: Query<(&Transform, Entity, &Block), With<Block>>,
) {
    let mut items = Vec::new();

    for (t, e, block) in block_query.iter() {
        if block.show {
            items.push(Collidable {
                pos: t.translation.truncate(),
                entity: e,
            });
        }
    }

    tree.0 = KdTree::build_by_ordered_float(items);
}

// 处理方块碰撞
fn handle_block_collision(
    tree: ResMut<BlockKdTree>,
    mut is_eliminate: ResMut<IsEliminate>,
    mut hand_block_query: Query<(&mut Transform, &mut TextureAtlas), With<HandBlock>>,
    mut block_query: Query<
        (&mut Block, &mut Visibility, &mut TextureAtlas),
        (With<Block>, Without<HandBlock>),
    >,
    mut next_state: ResMut<NextState<HandBlockState>>,
) {
    if hand_block_query.is_empty() || block_query.is_empty() {
        return;
    }

    let (transform, mut hand_block_text_atlas) = hand_block_query.single_mut();

    // 如果是闪电块 则返回 由闪电碰撞逻辑处理
    if hand_block_text_atlas.index == LIGHT_BLOCK_INDEX {
        return;
    }
    let pos = transform.translation.truncate();
    let blocks = tree.0.within_radius(&[pos.x, pos.y], 48.0);

    for b_e in blocks {
        if let Ok((mut b_b, mut b_visible, mut block_text_atlas)) = block_query.get_mut(b_e.entity)
        {
            if !b_b.show {
                continue;
            }
            // 判断方块碰撞后是否可消除
            // 种类相同 可消除
            if block_text_atlas.index == hand_block_text_atlas.index {
                b_b.show = false;
                is_eliminate.0 = true;
                *b_visible = Visibility::Hidden;
            } else {
                // 不同种类方块 不可消除 且之前有消除过 则交换方块种类
                if is_eliminate.0 {
                    (hand_block_text_atlas.index, block_text_atlas.index) =
                        (block_text_atlas.index, hand_block_text_atlas.index);
                    is_eliminate.0 = false;
                }
                next_state.set(HandBlockState::Backing);
            }
        }
    }
}

// 处理万能方块碰撞
fn handle_lightning_block_collision(
    tree: ResMut<BlockKdTree>,
    mut lighting_first_remove_block: ResMut<LightFirstRemoveBlock>,
    mut hand_block_query: Query<(&mut Transform, &mut TextureAtlas), With<HandBlock>>,
    mut block_query: Query<
        (&mut Block, &mut Visibility, &mut TextureAtlas),
        (With<Block>, Without<HandBlock>),
    >,
    mut next_state: ResMut<NextState<HandBlockState>>,
) {
    if hand_block_query.is_empty() || block_query.is_empty() {
        return;
    }

    let (transform, hand_block_text_atlas) = hand_block_query.single_mut();
    // 如果非闪电块 则直接返回
    if hand_block_text_atlas.index != LIGHT_BLOCK_INDEX {
        return;
    }

    let pos = transform.translation.truncate();
    let blocks = tree.0.within_radius(&[pos.x, pos.y], 48.0);

    for b_e in blocks {
        if let Ok((mut b_b, mut b_visible, block_text_atlas)) = block_query.get_mut(b_e.entity) {
            if !b_b.show {
                continue;
            }

            if let Some(index) = lighting_first_remove_block.0 {
                if block_text_atlas.index == index {
                    b_b.show = false;
                    *b_visible = Visibility::Hidden;
                } else {
                    next_state.set(HandBlockState::Backing);
                }
            } else {
                b_b.show = false;
                *b_visible = Visibility::Hidden;
                lighting_first_remove_block.0 = Some(block_text_atlas.index);
            }
        }
    }
}

// 闪电方块返回重置
fn lighting_first_remove_block_index_reset(
    mut lighting_first_remove_block: ResMut<LightFirstRemoveBlock>,
    mut hand_block_query: Query<&mut TextureAtlas, With<HandBlock>>,
) {
    if hand_block_query.is_empty() {
        return;
    }

    if let Some(index) = lighting_first_remove_block.0 {
        hand_block_query.single_mut().index = index;
    }

    lighting_first_remove_block.0 = None;
}

// 处理墙体碰撞
fn handle_block_wall_collision(
    tree: ResMut<WallKdTree>,
    mut hand_block_query: Query<(&mut Transform, &mut HandBlock), With<HandBlock>>,
    wall_query: Query<&Transform, (With<Wall>, Without<HandBlock>)>,
    mut _next_state: ResMut<NextState<HandBlockState>>,
) {
    if hand_block_query.is_empty() || wall_query.is_empty() {
        return;
    }

    let (mut transform, mut hand_block) = hand_block_query.single_mut();
    let pos = transform.translation.truncate();
    let walls = tree.0.within_radius(&[pos.x, pos.y], 42.0);

    for w_e in walls {
        if let Ok(w_t) = wall_query.get(w_e.entity) {
            hand_block.direction = Direction::Down;
            transform.translation.x = w_t.translation.x + STEP_SIZE as f32;
        }
    }
}

// 处理地面碰撞
fn handle_block_ground_collision(
    tree: ResMut<GroundKdTree>,
    mut is_eliminate: ResMut<IsEliminate>,
    hand_block_query: Query<&Transform, With<HandBlock>>,
    mut next_state: ResMut<NextState<HandBlockState>>,
) {
    if hand_block_query.is_empty() {
        return;
    }

    let transform = hand_block_query.single();
    let pos = transform.translation.truncate();
    let grounds = tree.0.within_radius(&[pos.x, pos.y], 42.0);

    for _ in grounds {
        next_state.set(HandBlockState::Backing);
        is_eliminate.0 = false;
    }
}

// 方块返回动画
fn handle_collision_back_animation(
    time: Res<Time>,
    mut timer: ResMut<CollisionBackTimer>,
    mut points: ResMut<BezierPoints>,
    mut next_state: ResMut<NextState<HandBlockState>>,
    player_query: Query<&Transform, With<Player>>,
    mut hand_block_query: Query<&mut Transform, (With<HandBlock>, Without<Player>)>,
) {
    if hand_block_query.is_empty() || player_query.is_empty() {
        return;
    }

    let player_transform = player_query.single();
    let mut hand_block_transform = hand_block_query.single_mut();
    let pos = hand_block_transform.translation.truncate();
    let direction = player_transform.translation.truncate() - pos;

    timer.0.tick(time.delta());

    if let Some(b_points) = points.0 {
        let bezier = CubicBezier::new(b_points).to_curve();

        hand_block_transform.translation = bezier.position(timer.0.elapsed_secs() * 3.0);

        if direction.length() < 50.0 {
            points.0 = None;

            next_state.set(HandBlockState::Idle);

            timer.0.reset();
        }
    } else {
        let target_pos = hand_block_transform.translation;
        let top_y = (player_transform.translation.y + 220.0).min(WH / 2.0 - STEP_SIZE as f32);
        points.0 = Some([[
            target_pos,
            vec3(-50., top_y, 0.),
            vec3(-50., top_y, 0.),
            vec3(
                player_transform.translation.x - STEP_SIZE as f32,
                player_transform.translation.y,
                player_transform.translation.z,
            ),
        ]]);
    }
}

// 返回声音
fn hand_block_back_sound(audio_handles: Res<GlobalAudio>, mut commands: Commands) {
    if let Some(player_move_source) = audio_handles.hand_block_black.clone() {
        commands.spawn(AudioBundle {
            source: player_move_source,
            ..default()
        });
    }
}
