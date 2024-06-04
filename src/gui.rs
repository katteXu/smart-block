use bevy::math::vec3;
use bevy::prelude::*;

use crate::block::Block;
use crate::stage::Stage;
use crate::state::{GameState, SettlementState};
use crate::world::GameEntity;
use crate::*;

pub struct GuiPlugin;

// UI 分数
#[derive(Component)]
pub struct TextScore;

#[derive(Resource)]
pub struct Score {
    pub total_score: u32,
    pub once_remove_block: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            total_score: 0,
            once_remove_block: 0,
        }
    }
}

// 高分展示
#[derive(Component)]
pub struct HighScore {
    pub animation_timer: Timer,
}
impl Default for HighScore {
    fn default() -> Self {
        Self {
            animation_timer: Timer::from_seconds(HIGH_SCORE_ANIMATION_DURATION, TimerMode::Once),
        }
    }
}

#[derive(Component)]
pub struct TextCountDown;

#[derive(Component)]
pub struct ClearNum;

#[derive(Component)]
pub struct BlockNum;

#[derive(Resource)]
pub struct CountDown(pub Timer);

impl Default for CountDown {
    fn default() -> Self {
        Self(Timer::from_seconds(COUNT_DOWN_SEC, TimerMode::Repeating))
    }
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CountDown>()
            .init_resource::<Score>()
            .add_systems(OnEnter(GameState::InGame), spawn_gui)
            .add_systems(
                Update,
                (update_score, update_count_down)
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(SettlementState::Not)),
            )
            .add_systems(
                Update,
                (update_count_down_text, update_block_number).run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnEnter(GameState::MainMenu), reset_resources);
    }
}

// 生成游戏内UI
fn spawn_gui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    stage: Res<Stage>,
    score: Res<Score>,
) {
    // 分数
    commands
        .spawn((
            NodeBundle {
                background_color: Color::BLACK.into(),
                style: Style {
                    width: Val::Px(SCORE_BLOCK_WIDTH),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    position_type: PositionType::Absolute,
                    top: Val::Px(SCORE_BLOCK_POS.1),
                    left: Val::Px(SCORE_BLOCK_POS.0),
                    ..Default::default()
                },
                ..default()
            },
            GameEntity,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    SCORE_TEXT,
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 28.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..Default::default()
            });

            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        format!("{:0>7}", score.total_score),
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 28.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..Default::default()
                },
                TextScore,
            ));
        });

    // 获胜剩余数
    commands
        .spawn((
            NodeBundle {
                background_color: Color::BLACK.into(),
                style: Style {
                    width: Val::Px(RIGHT_BLOCK_WIDTH),
                    height: Val::Px(RIGHT_BLOCK_HEIGHT),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    top: Val::Px(CLEAR_BLOCK_POS.1),
                    right: Val::Px(CLEAR_BLOCK_POS.0),
                    ..Default::default()
                },
                ..default()
            },
            GameEntity,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    CLEAR_TEXT,
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..Default::default()
            });

            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        CLEAR_NUM.to_string(),
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 32.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..Default::default()
                },
                ClearNum,
            ));
        });

    // 方块数
    commands
        .spawn((
            NodeBundle {
                background_color: Color::BLACK.into(),
                style: Style {
                    width: Val::Px(RIGHT_BLOCK_WIDTH),
                    height: Val::Px(RIGHT_BLOCK_HEIGHT),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    top: Val::Px(BLOCK_BLOCK_POS.1),
                    right: Val::Px(BLOCK_BLOCK_POS.0),
                    ..Default::default()
                },
                ..default()
            },
            GameEntity,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    BLOCK_TEXT,
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..Default::default()
            });

            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "16",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 32.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..Default::default()
                },
                BlockNum,
            ));
        });

    // 倒计时
    commands
        .spawn((
            NodeBundle {
                background_color: Color::BLACK.into(),
                style: Style {
                    width: Val::Px(RIGHT_BLOCK_WIDTH),
                    height: Val::Px(RIGHT_BLOCK_HEIGHT),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    top: Val::Px(COUNT_DOWN_BLOCK_POS.1),
                    right: Val::Px(COUNT_DOWN_BLOCK_POS.0),
                    ..Default::default()
                },
                ..default()
            },
            GameEntity,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "0:00",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 56.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..Default::default()
                },
                TextCountDown,
            ));
        });
    // 当前关卡
    commands
        .spawn((
            NodeBundle {
                background_color: Color::BLACK.into(),
                style: Style {
                    width: Val::Px(RIGHT_BLOCK_WIDTH),
                    height: Val::Px(RIGHT_BLOCK_HEIGHT),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    top: Val::Px(STAGE_BLOCK_POS.1),
                    right: Val::Px(STAGE_BLOCK_POS.0),
                    ..Default::default()
                },
                ..default()
            },
            GameEntity,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    STAGE_TEXT,
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..Default::default()
            });

            parent.spawn((TextBundle {
                text: Text::from_section(
                    stage.0.to_string(),
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 32.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..Default::default()
            },));
        });
}

// 更新分数
fn update_score(
    commands: Commands,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
    mut text_score_query: Query<&mut Text, With<TextScore>>,
) {
    if text_score_query.is_empty() {
        return;
    }
    let mut text_score = text_score_query.single_mut();

    if score.once_remove_block > 0 {
        let once_score = score.once_remove_block.pow(2) * ONEC_BLOCK_SCORE;
        score.total_score += once_score;
        if score.once_remove_block > 1 {
            spawn_hight_score(commands, asset_server.load(FONT_PATH), once_score);
        }
        score.once_remove_block = 0;
        text_score.sections[0].value = format!("{:0>7}", score.total_score);
    }
}

// 更新block数量
fn update_block_number(
    mut query: Query<&mut Text, With<BlockNum>>,
    block_query: Query<&Transform, With<Block>>,
) {
    if query.is_empty() {
        return;
    }
    let mut text = query.single_mut();
    let block_num = if block_query.is_empty() {
        0
    } else {
        block_query.iter().count()
    };

    text.sections[0].value = format!("{}", block_num);
}

// 更新倒计时 如果结束则跳转状态
fn update_count_down(
    time: Res<Time>,
    mut count_down: ResMut<CountDown>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    count_down.0.tick(time.delta());

    if count_down.0.just_finished() {
        count_down.0.reset();
        next_state.set(GameState::MainMenu);
        return;
    }
}

// 更新倒计时文案
fn update_count_down_text(
    count_down: Res<CountDown>,
    mut query: Query<&mut Text, With<TextCountDown>>,
) {
    if query.is_empty() {
        return;
    }

    let mut text = query.single_mut();
    let total_time = COUNT_DOWN_SEC - count_down.0.elapsed_secs();
    let minite = (total_time / 60.0).floor();
    let seconds = (total_time % 60.0).floor();

    text.sections[0].value = format!("{:0>2}:{:0>2}", minite, seconds);
}

// 生成高分提示
pub fn spawn_hight_score(mut commands: Commands, font_handle: Handle<Font>, score_value: u32) {
    let (x, y) = HIGH_SCORE_POS_PERCENT;
    commands.spawn((
        Text2dBundle {
            transform: Transform::from_translation(vec3(x, y, 1.0)),
            text: Text::from_section(
                score_value.to_string(),
                TextStyle {
                    font: font_handle.clone_weak(),
                    font_size: 72.0,
                    color: Color::YELLOW,
                    ..default()
                },
            ),
            ..Default::default()
        },
        HighScore::default(),
        GameEntity,
    ));
}

// 重置资源
fn reset_resources(
    mut total_score: ResMut<Score>,
    mut stage: ResMut<Stage>,
    mut count_down: ResMut<CountDown>,
) {
    total_score.total_score = 0;
    stage.0 = 1;
    count_down.0.reset();
}
