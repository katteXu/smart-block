use std::time::Duration;

use bevy::audio::PlaybackMode;
use bevy::audio::Volume;
use bevy::prelude::*;

use crate::block::Block;
use crate::gui::CountDown;
use crate::gui::TextScore;
use crate::state::{GameState, SettlementState};
use crate::world::GameEntity;
use crate::*;

use self::gui::Score;
use self::resources::GlobalAudio;
use self::stage::Stage;

pub struct SettlementPlugin;

// 时间转化分数
#[derive(Resource)]
pub struct TimeToScore(u64);
impl Default for TimeToScore {
    fn default() -> Self {
        Self(0)
    }
}

// 结算开始倒计时
#[derive(Resource)]
struct SettleStartTimer(Timer);
impl Default for SettleStartTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

// 消除每一个方块倒计时
#[derive(Resource)]
struct DespawnBlockTimer(Timer);
impl Default for DespawnBlockTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

// 剩余时间
#[derive(Resource)]
struct RemainTime(Option<u64>);
impl Default for RemainTime {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Component)]
struct TextTimeToScore;

impl Plugin for SettlementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeToScore>()
            .init_resource::<RemainTime>()
            .init_state::<SettlementState>()
            .init_resource::<SettleStartTimer>()
            .init_resource::<DespawnBlockTimer>()
            .add_systems(
                OnEnter(SettlementState::Start),
                spawn_settlement.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    // 开始结算
                    settle_start.run_if(in_state(SettlementState::Start)),
                    // 消除方块
                    despawn_remainder_block.run_if(in_state(SettlementState::DespawnBlock)),
                    // 计算分数
                    time_to_score.run_if(in_state(SettlementState::TimeToScore)),
                    // 结算结束
                ),
            )
            .add_systems(OnEnter(SettlementState::End), update_total_score);
    }
}

// 生成结算
fn spawn_settlement(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load(FONT_PATH);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            GameEntity,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "CLEAR!",
                    TextStyle {
                        font_size: 32.0,
                        font: font_handle.clone(),
                        ..default()
                    },
                ),
                ..default()
            });

            parent.spawn(TextBundle {
                text: Text::from_section(
                    "CONGRATULATIONS!",
                    TextStyle {
                        font_size: 32.0,
                        font: font_handle.clone(),
                        ..default()
                    },
                ),
                ..default()
            });

            parent.spawn((
                TextBundle {
                    text: Text::from_sections([TextSection::new(
                        "time score: 00000",
                        TextStyle {
                            font: font_handle.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                    )]),
                    ..default()
                },
                TextTimeToScore,
            ));
        });
}

// 结算总分数
fn settle_start(
    time: Res<Time>,
    mut settle_start_timer: ResMut<SettleStartTimer>,
    mut next_state: ResMut<NextState<SettlementState>>,
) {
    // println!("计算总分数");
    settle_start_timer.0.tick(time.delta());

    if settle_start_timer.0.just_finished() {
        println!("开始清算");
        next_state.set(SettlementState::DespawnBlock);
    }
}

// 销毁剩余方块
fn despawn_remainder_block(
    time: Res<Time>,
    mut timer: ResMut<DespawnBlockTimer>,
    mut commands: Commands,
    mut block_query: Query<(&mut TextureAtlas, Entity), With<Block>>,
    mut next_state: ResMut<NextState<SettlementState>>,
) {
    if block_query.is_empty() {
        // 没有剩余则直接进入分数统计
        next_state.set(SettlementState::TimeToScore);
        return;
    }
    timer.0.tick(time.delta());

    for (mut text_atlas, entity) in block_query.iter_mut() {
        text_atlas.index = BLOCK_BEFORE_REMOVE_INDEX;

        if timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
        return;
    }
}

// 计算剩余时间获得分数
fn time_to_score(
    mut count_down: ResMut<CountDown>,
    mut time_to_score: ResMut<TimeToScore>,
    mut time_to_score_text_query: Query<&mut Text, With<TextTimeToScore>>,
    mut remain_time: Local<RemainTime>,
    mut next_state: ResMut<NextState<SettlementState>>,
    mut commands: Commands,
    audio_handles: Res<GlobalAudio>,
) {
    if time_to_score_text_query.is_empty() {
        return;
    }

    let mut time_to_score_text = time_to_score_text_query.single_mut();

    // 每0.5秒计算一次
    let pass_time = 0.5;

    // 获取剩余时间
    if remain_time.0.is_none() {
        let time = count_down.0.remaining();
        remain_time.0 = Some(time.as_secs());
    }

    // 获取倒计时
    count_down.0.tick(Duration::from_secs_f32(pass_time));

    if count_down.0.remaining().as_secs() != 0 {
        time_to_score.0 += (EVERY_SECOND_SCORE as f32 * pass_time) as u64;
        time_to_score_text.sections[0].value = format!("time score: {:0>5}", time_to_score.0);
    }

    if count_down.0.just_finished() {
        println!("清算倒计时结束");
        if let Some(time_clear_sound) = audio_handles.time_clear.clone() {
            commands.spawn(AudioBundle {
                source: time_clear_sound,
                settings: PlaybackSettings {
                    // mode: PlaybackMode::Once,
                    // volume: Volume::new(2.0),
                    ..default()
                },
            });
        }
        if let Some(time) = remain_time.0 {
            time_to_score.0 = (time * EVERY_SECOND_SCORE) as u64;
            time_to_score_text.sections[0].value = format!("time score: {:0>5}", time_to_score.0);

            next_state.set(SettlementState::End);
        }
    }
}

// 局部分数更新总分数
fn update_total_score(
    mut time_to_score: ResMut<TimeToScore>,
    mut stage: ResMut<Stage>,
    mut score: ResMut<Score>,
    mut query: Query<&mut Text, With<TextScore>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut settlement_next_state: ResMut<NextState<SettlementState>>,
) {
    if query.is_empty() {
        return;
    }
    let mut text = query.single_mut();
    score.total_score += time_to_score.0 as u32;
    text.sections[0].value = format!("{:0>7}", score.total_score);

    println!("更新总分完成");
    // 清空局部分数
    time_to_score.0 = 0;

    // 更新关卡
    stage.0 += 1;
    next_state.set(GameState::GameInit);
    settlement_next_state.set(SettlementState::Not);
}
