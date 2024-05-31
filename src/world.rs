use bevy::math::vec3;
use bevy::prelude::*;
#[allow(unused)]
use rand::Rng;

use crate::state::{GameState, PlayerState};
use crate::*;

use crate::animation::AnimationTimer;
use crate::block::{Block, HandBlock};
use crate::player::Player;
use crate::resources::GlobalTextAtlas;
use crate::wall::Ground;
use crate::wall::Wall;

#[derive(Component)]
pub struct GameEntity;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInit), init_world)
            .add_systems(OnExit(GameState::InGame), despawn_all_game_entities);
    }
}

// 初始化游戏世界
fn init_world(
    mut commands: Commands,
    handle: ResMut<GlobalTextAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut rng = rand::thread_rng();

    // 生成玩家
    let (x, y) = PLAYER_INIT_POS;
    commands.spawn((
        SpriteSheetBundle {
            texture: handle.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 0,
            },
            transform: Transform::from_translation(vec3(x, y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Player,
        PlayerState::default(),
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Once)),
        GameEntity,
    ));

    // 生成梯子
    for i in 0..100 {
        commands.spawn((
            SpriteSheetBundle {
                texture: handle.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 4,
                },
                transform: Transform::from_translation(vec3(x, y + (i * STEP_SIZE) as f32, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            GameEntity,
        ));
    }

    let hand_block_index = HAND_BLOCK_INDEX; // rng.gen_range(BLOCK_DISPLAY_RANGE); // HAND_BLOCK_INDEX; // 闪电是15

    // 生成手上方块
    commands.spawn((
        SpriteSheetBundle {
            texture: handle.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: hand_block_index,
            },
            transform: Transform::from_translation(vec3(x - STEP_SIZE as f32, y, 0.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..Default::default()
        },
        HandBlock {
            index: hand_block_index,
            ..HandBlock::default()
        },
        GameEntity,
    ));

    // 生成周围墙面
    for i in 1..=25 {
        for j in 0..=14 {
            if j == 0 || j == 14 {
                commands.spawn((
                    SpriteSheetBundle {
                        texture: handle.image.clone().unwrap(),
                        atlas: TextureAtlas {
                            layout: handle.layout.clone().unwrap(),
                            index: 6,
                        },
                        transform: Transform::from_translation(vec3(
                            -(WW + STEP_SIZE as f32) / 2.0 + (i * STEP_SIZE) as f32,
                            (WH - STEP_SIZE as f32) / 2.0 - (j * STEP_SIZE) as f32,
                            1.0,
                        ))
                        .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                        ..default()
                    },
                    // Wall,
                    Ground,
                    GameEntity,
                ));
            } else if i == 1 || i >= 19 {
                commands.spawn((
                    SpriteSheetBundle {
                        texture: handle.image.clone().unwrap(),
                        atlas: TextureAtlas {
                            layout: handle.layout.clone().unwrap(),
                            index: 6,
                        },
                        transform: Transform::from_translation(vec3(
                            -(WW + STEP_SIZE as f32) / 2.0 + (i * STEP_SIZE) as f32,
                            (WH - STEP_SIZE as f32) / 2.0 - (j * STEP_SIZE) as f32,
                            1.0,
                        ))
                        .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                        ..default()
                    },
                    Wall,
                    GameEntity,
                ));
            }
        }
    }

    // 测试障碍墙
    let wall_vec2 = vec![
        // 墙体
        [1, 1, 1, 0],
        [1, 1, 0, 0],
        [1, 0, 0, 0],
        [0, 0, 0, 0],
    ];

    for j in 0..wall_vec2.len() {
        for i in 0..wall_vec2[j].len() {
            if wall_vec2[i][j] == 1 {
                commands.spawn((
                    SpriteSheetBundle {
                        texture: handle.image.clone().unwrap(),
                        atlas: TextureAtlas {
                            layout: handle.layout.clone().unwrap(),
                            index: 6,
                        },
                        transform: Transform::from_translation(vec3(
                            -(WW + STEP_SIZE as f32) / 2.0 + ((i + 2) * STEP_SIZE) as f32,
                            (WH - STEP_SIZE as f32) / 2.0 - ((j + 1) * STEP_SIZE) as f32,
                            1.0,
                        ))
                        .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                        ..default()
                    },
                    Wall,
                    GameEntity,
                ));
            }
        }
    }

    // 生成方块组
    let (x, y) = BLOCK_INIT_POS;
    for j in 0..BLOCK_NUM_H {
        for i in 0..BLOCK_NUM_W {
            let texture_atlas_index = rng.gen_range(BLOCK_DISPLAY_RANGE);
            commands.spawn((
                SpriteSheetBundle {
                    texture: handle.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: texture_atlas_index,
                    },
                    transform: Transform::from_translation(vec3(
                        x + (i * STEP_SIZE) as f32,
                        y + (j * STEP_SIZE) as f32,
                        0.0,
                    ))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    ..default()
                },
                Block {
                    index: texture_atlas_index,
                    show: true,
                },
                GameEntity,
            ));
        }
    }

    // 测试固定渲染
    // let group = TEST_BLOCK_POS.iter().rev().cloned().collect::<Vec<_>>();
    // for pos_y in 0..group.len() {
    //     for pos_x in 0..group[pos_y].len() {
    //         let index = group[pos_y][pos_x];
    //         if index > 0 {
    //             commands.spawn((
    //                 SpriteSheetBundle {
    //                     texture: handle.image.clone().unwrap(),
    //                     atlas: TextureAtlas {
    //                         layout: handle.layout.clone().unwrap(),
    //                         index: index,
    //                     },
    //                     transform: Transform::from_translation(vec3(
    //                         x + (pos_x * STEP_SIZE) as f32,
    //                         y + (pos_y * STEP_SIZE) as f32,
    //                         0.0,
    //                     ))
    //                     .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
    //                     ..default()
    //                 },
    //                 Block {
    //                     index: index,
    //                     show: true,
    //                 },
    //                 GameEntity,
    //             ));
    //         }
    //     }
    // }

    next_state.set(GameState::InGame);
}

fn despawn_all_game_entities(
    mut commands: Commands,
    game_entities: Query<Entity, With<GameEntity>>,
) {
    for entity in game_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
