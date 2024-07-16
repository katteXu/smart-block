use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use rand::Rng;

use crate::block::Block;
use crate::resources::GlobalTextAtlas;
use crate::state::GameState;
use crate::world::GameEntity;
use crate::*;

#[derive(Resource)]
pub struct Stage(pub usize);

impl Default for Stage {
    fn default() -> Self {
        Stage(1)
    }
}

#[derive(Resource)]
pub struct DespawnStageTextTimer(Timer);
impl Default for DespawnStageTextTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}
#[derive(Component)]
pub struct StageText;

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Stage>()
            .init_resource::<DespawnStageTextTimer>()
            .add_systems(OnEnter(GameState::InGame), spawn_game_stage)
            .add_systems(
                Update,
                despawn_stage_text.run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnEnter(GameState::InGame), create_block_group);
    }
}

fn spawn_game_stage(mut commands: Commands, stage: Res<Stage>) {
    // 生成关卡提示文案
    let text = format!("{} {}", STAGE_TEXT, stage.0);
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            ..default()
        },
        StageText,
    ));
}

// 销毁关卡文案  1s延时
fn despawn_stage_text(
    time: Res<Time>,
    mut commands: Commands,
    mut timer: ResMut<DespawnStageTextTimer>,
    mut text_query: Query<Entity, With<StageText>>,
) {
    if text_query.is_empty() {
        return;
    }

    let entity = text_query.single_mut();

    // 计时器
    if timer.0.tick(time.delta()).just_finished() {
        commands.entity(entity).despawn();
    }
}

// 检测新关卡是否
fn _new_stage(stage: Res<Stage>) -> bool {
    stage.is_changed()
}

fn create_block_group(mut commands: Commands, handle: ResMut<GlobalTextAtlas>) {
    println!("生成方块");
    // 初始位置
    let (x, y) = BLOCK_INIT_POS;

    // 测试固定渲染
    // let group = TEST_BLOCK_POS.iter().rev().cloned().collect::<Vec<_>>();
    let group = create_block(4, 5);
    for pos_y in 0..group.len() {
        for pos_x in 0..group[pos_y].len() {
            let index = group[pos_y][pos_x];
            if index > 0 {
                commands.spawn((
                    SpriteSheetBundle {
                        texture: handle.image.clone().unwrap(),
                        atlas: TextureAtlas {
                            layout: handle.layout.clone().unwrap(),
                            index: index,
                        },
                        transform: Transform::from_translation(vec3(
                            x + (pos_x * STEP_SIZE) as f32,
                            y + (pos_y * STEP_SIZE) as f32,
                            0.0,
                        ))
                        .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                        ..default()
                    },
                    Block {
                        show: true,
                        pos: vec2(
                            x + (pos_x * STEP_SIZE) as f32,
                            y + (pos_y * STEP_SIZE) as f32,
                        ),
                    },
                    GameEntity,
                ));
            }
        }
    }
}

// 生成方块组
fn create_block(row: usize, col: usize) -> Vec<Vec<usize>> {
    let mut block_group = vec![];

    let mut col0_index = None;

    for i in 0..row {
        let mut _row: Vec<usize> = vec![];
        let index = rand::thread_rng().gen_range(BLOCK_DISPLAY_RANGE);
        let other_index = rand::thread_rng().gen_range(BLOCK_DISPLAY_RANGE);
        for j in 0..col {
            if j <= 1 {
                let index = rand::thread_rng().gen_range(BLOCK_DISPLAY_RANGE);
                _row.push(index);
            } else {
                if let Some(index) = col0_index {
                    _row.push(index);
                } else {
                    _row.push(other_index);
                }
            }
        }
        col0_index = Some(index);
        block_group.push(_row);
    }

    println!("{:?}", block_group);

    // 翻转一下
    block_group.reverse();

    return block_group;
}
