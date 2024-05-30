use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use kd_tree::{KdPoint, KdTree};

use crate::block::{Block, HandBlock};
use crate::state::{GameState, HandBlockState};
use crate::*;

use crate::block::Direction;
use crate::wall::{Ground, Wall};

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

#[derive(Resource)]
struct IsEliminate(bool);

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BlockKdTree::default())
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
                        handle_block_wall_collision,
                        handle_block_ground_collision,
                    )
                        .run_if(in_state(HandBlockState::Moving)),
                    update_block_kd_tree
                        .run_if(in_state(HandBlockState::Idle))
                        .run_if(on_timer(Duration::from_secs_f32(KD_TREE_REFRESH_RATE))),
                )
                    .run_if(in_state(GameState::InGame)),
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
    mut hand_block_query: Query<(&mut Transform, &mut HandBlock), With<HandBlock>>,
    mut block_query: Query<&mut Block, (With<Block>, Without<HandBlock>)>,
    mut next_state: ResMut<NextState<HandBlockState>>,
) {
    if hand_block_query.is_empty() || block_query.is_empty() {
        return;
    }

    let (transform, mut hand_block) = hand_block_query.single_mut();
    let pos = transform.translation.truncate();
    let blocks = tree.0.within_radius(&[pos.x, pos.y], 48.0);

    for b_e in blocks {
        if let Ok(mut b_b) = block_query.get_mut(b_e.entity) {
            if b_b.index == hand_block.index {
                b_b.show = false;
                is_eliminate.0 = true;
            } else if b_b.show {
                if is_eliminate.0 {
                    (hand_block.index, b_b.index) = (b_b.index, hand_block.index);
                    is_eliminate.0 = false;
                }
                next_state.set(HandBlockState::Idle);
            }
        }
    }
}

// 处理墙体碰撞
fn handle_block_wall_collision(
    tree: ResMut<WallKdTree>,
    mut is_eliminate: ResMut<IsEliminate>,
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
            // is_eliminate.0 = false;
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
        next_state.set(HandBlockState::Idle);
        is_eliminate.0 = false;
    }
}
