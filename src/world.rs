use std::time::Duration;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

use crate::state::GameState;
use crate::*;

use crate::block::Block;
use crate::player::HandBlock;
use crate::player::Player;

use self::animation::AnimationTimer;
use self::player::ActionTimer;
use self::player::PlayerState;
use self::resources::GlobalTextAtlas;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInit), init_world);
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
        ActionTimer(Stopwatch::new()),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    // 生成梯子
    for i in 0..100 {
        commands.spawn(SpriteSheetBundle {
            texture: handle.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 4,
            },
            transform: Transform::from_translation(vec3(x, y + (i * STEP_SIZE) as f32, 0.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        });
    }

    // 生成手上方块
    commands.spawn((
        SpriteSheetBundle {
            texture: handle.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: rng.gen_range(8..=13),
            },
            transform: Transform::from_translation(vec3(x - STEP_SIZE as f32, y, 0.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..Default::default()
        },
        HandBlock,
    ));

    // 生成墙面
    for i in 1..=25 {
        for j in 0..=14 {
            if j == 0 || j == 14 {
                commands.spawn(SpriteSheetBundle {
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
                });
            } else if i == 1 || i >= 19 {
                commands.spawn(SpriteSheetBundle {
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
                });
            }
        }
    }

    // 生成方块组
    let (x, y) = BLOCK_INIT_POS;
    for i in 0..BLOCK_NUM_W {
        for j in 0..BLOCK_NUM_H {
            commands.spawn((
                SpriteSheetBundle {
                    texture: handle.image.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: rng.gen_range(8..=13),
                    },
                    transform: Transform::from_translation(vec3(
                        x + (i * STEP_SIZE) as f32,
                        y + (j * STEP_SIZE) as f32,
                        0.0,
                    ))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    ..Default::default()
                },
                Block,
            ));
        }
    }

    next_state.set(GameState::InGame);
}
