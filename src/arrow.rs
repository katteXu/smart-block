use std::f32::consts::PI;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;

use crate::player::Player;
use crate::*;
use crate::{resources::GlobalTextAtlas, state::GameState};

use crate::block::Block;
use crate::state::HandBlockState;
use crate::wall::Wall;
use crate::world::GameEntity;

pub struct ArrowPlugin;

#[derive(Component)]
struct Arrow;

impl Plugin for ArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInit), spawn_arrow)
            .add_systems(Update, update_arrow.run_if(in_state(HandBlockState::Idle)))
            .add_systems(
                OnEnter(HandBlockState::Idle),
                handle_arrow_show.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                OnEnter(HandBlockState::Moving),
                handle_arrow_hide.run_if(in_state(GameState::InGame)),
            );
    }
}

fn spawn_arrow(mut commands: Commands, handle: ResMut<GlobalTextAtlas>) {
    // 生成玩家
    let (x, y) = PLAYER_INIT_POS;
    commands.spawn((
        SpriteSheetBundle {
            texture: handle.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: ARROW_TEXTATLAS_INDEX,
            },
            transform: Transform::from_translation(vec3(x, y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Arrow,
        GameEntity,
    ));
}

// 更新箭头
fn update_arrow(
    player_query: Query<&Transform, With<Player>>,
    block_query: Query<&Transform, (With<Block>, Without<Player>)>,
    wall_query: Query<&Transform, (With<Wall>, Without<Player>, Without<Block>)>,
    mut arrow_query: Query<
        &mut Transform,
        (With<Arrow>, Without<Player>, Without<Block>, Without<Wall>),
    >,
) {
    if player_query.is_empty()
        || arrow_query.is_empty()
        || block_query.is_empty()
        || wall_query.is_empty()
    {
        return;
    }

    let player_transform = player_query.single();
    // 基于y
    let base_y = player_transform.translation.y;
    // 基于block
    let mut base_block = false;

    let mut arrow_transform = arrow_query.single_mut();
    let mut target_block = Vec2::NEG_INFINITY;

    // 方块位置
    for block_transform in block_query.iter() {
        let block_y = block_transform.translation.y;
        if base_y == block_y {
            target_block.y = block_y;
            target_block.x = block_transform.translation.x.max(target_block.x);
            base_block = true;
            // 角度
            let angle = 0.5 * PI;
            arrow_transform.rotation = Quat::from_rotation_z(angle);
        }
    }

    if !base_block {
        // 墙体位置
        for wall_transform in wall_query.iter() {
            let wall_y = wall_transform.translation.y;
            let wall_x = wall_transform.translation.x;
            // wall_x < 0.0 表示墙体在左侧
            if base_y == wall_y && wall_x < 0.0 {
                target_block.y = wall_y;
                target_block.x = wall_transform.translation.x.max(target_block.x);

                // 角度
                let angle = PI;
                arrow_transform.rotation = Quat::from_rotation_z(angle);

                let base_x = wall_x + STEP_SIZE as f32;
                target_block.y = Vec2::NEG_INFINITY.y;
                for block_transform in block_query.iter() {
                    let block_x = block_transform.translation.x;

                    if block_x == base_x {
                        target_block.x = block_x;
                        target_block.y = block_transform.translation.y.max(target_block.y);
                    }
                }
            }
        }
    }

    if base_block {
        arrow_transform.translation = vec3(target_block.x + STEP_SIZE as f32, target_block.y, 1.0);
    } else {
        arrow_transform.translation = vec3(target_block.x, target_block.y + STEP_SIZE as f32, 1.0);
    }
}

// 隐藏箭头
fn handle_arrow_hide(mut arrow_query: Query<&mut Visibility, (With<Arrow>, Without<Player>)>) {
    if arrow_query.is_empty() {
        return;
    }

    let mut visibility = arrow_query.single_mut();

    *visibility = Visibility::Hidden;
}

// 显示箭头
fn handle_arrow_show(mut arrow_query: Query<&mut Visibility, (With<Arrow>, Without<Player>)>) {
    if arrow_query.is_empty() {
        return;
    }

    let mut visibility = arrow_query.single_mut();

    *visibility = Visibility::Visible;
}
