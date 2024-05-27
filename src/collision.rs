use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use kd_tree::{KdPoint, KdTree};

use crate::state::GameState;
use crate::*;

use crate::block::Block;
use crate::player::{HandBlock, HandBlockState};

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

impl Default for BlockKdTree {
    fn default() -> Self {
        Self(KdTree::build_by_ordered_float(vec![]))
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BlockKdTree::default()).add_systems(
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

fn update_block_kd_tree(
    mut tree: ResMut<BlockKdTree>,
    block_query: Query<(&Transform, Entity), With<Block>>,
) {
    let mut items = Vec::new();

    for (t, e) in block_query.iter() {
        items.push(Collidable {
            pos: t.translation.truncate(),
            entity: e,
        });
    }

    tree.0 = KdTree::build_by_ordered_float(items);
}

// 处理碰撞
fn handle_block_collision(
    tree: ResMut<BlockKdTree>,
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
                b_t.translation.y += 16.0;
                b_b.show = false;
            } else {
                hand_block.index = b_b.index;
                next_state.set(HandBlockState::Idle);
            }
        }
    }
}
