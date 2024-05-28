use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use kd_tree::{KdPoint, KdTree};

use crate::block::{Block, HandBlock};
use crate::state::{GameState, HandBlockState};
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

#[derive(Resource)]
pub struct BlockKdTree(pub KdTree<Collidable>);

#[derive(Resource)]
struct IsEliminate(bool);

impl Default for BlockKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BlockKdTree::default())
            .insert_resource(IsEliminate(false))
            .add_systems(
                Update,
                (
                    handle_block_collision.run_if(in_state(HandBlockState::Moving)),
                    update_block_kd_tree
                        .run_if(in_state(HandBlockState::Idle))
                        .run_if(on_timer(Duration::from_secs_f32(KD_TREE_REFRESH_RATE))),
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
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

// 处理碰撞
fn handle_block_collision(
    tree: ResMut<BlockKdTree>,
    mut is_eliminate: ResMut<IsEliminate>,
    mut hand_block_query: Query<(&mut Transform, &mut HandBlock), With<HandBlock>>,
    mut block_query: Query<(&mut Transform, &mut Block), (With<Block>, Without<HandBlock>)>,
    mut next_state: ResMut<NextState<HandBlockState>>,
) {
    if hand_block_query.is_empty() || block_query.is_empty() {
        return;
    }

    let (transform, mut hand_block) = hand_block_query.single_mut();
    let pos = transform.translation.truncate();
    let blocks = tree.0.within_radius(&[pos.x, pos.y], 48.0);

    for b_e in blocks {
        if let Ok((mut b_t, mut b_b)) = block_query.get_mut(b_e.entity) {
            if b_b.index == hand_block.index {
                b_t.translation.y += 12.0;
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
