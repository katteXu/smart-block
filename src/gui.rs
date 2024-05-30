use bevy::prelude::*;

use crate::block::Block;
use crate::state::GameState;
use crate::world::GameEntity;
use crate::*;

pub struct GuiPlugin;

#[derive(Component)]
pub struct TextScore {
    pub total_score: u32,
    pub once_score: u32,
}
impl Default for TextScore {
    fn default() -> Self {
        Self {
            total_score: 0,
            once_score: 0,
        }
    }
}

#[derive(Component)]
pub struct TextCountDown;

#[derive(Component)]
pub struct BlockNum;

#[derive(Component)]
pub struct Stage;

#[derive(Resource)]
pub struct CountDown(pub Timer);

impl Default for CountDown {
    fn default() -> Self {
        Self(Timer::from_seconds(180.0, TimerMode::Once))
    }
}

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CountDown>()
            .add_systems(OnEnter(GameState::InGame), spawn_gui)
            .add_systems(
                Update,
                (update_score, update_block_number, update_count_down)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

// 生成游戏内UI
fn spawn_gui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                        "0000000",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 28.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..Default::default()
                },
                TextScore::default(),
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

            parent.spawn((TextBundle {
                text: Text::from_section(
                    "4",
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

            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "1",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 32.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..Default::default()
                },
                Stage,
            ));
        });
}

// 更新分数
fn update_score(mut query: Query<(&mut Text, &mut TextScore), With<TextScore>>) {
    if query.is_empty() {
        return;
    }

    let (mut text, mut text_score) = query.single_mut();

    if text_score.once_score > 0 {
        text_score.total_score += text_score.once_score;
        text_score.once_score = 0;
        text.sections[0].value = format!("{:0>7}", text_score.total_score);
    }
}

// 更新block数量
fn update_block_number(
    mut query: Query<&mut Text, With<BlockNum>>,
    block_query: Query<&Transform, With<Block>>,
) {
    if query.is_empty() || block_query.is_empty() {
        return;
    }

    let mut text = query.single_mut();
    let block_num = block_query.iter().count();

    text.sections[0].value = format!("{}", block_num);
}

// 更新倒计时
fn update_count_down(
    time: Res<Time>,
    mut count_down: ResMut<CountDown>,
    mut query: Query<&mut Text, With<TextCountDown>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if query.is_empty() {
        return;
    }
    count_down.0.tick(time.delta());

    if count_down.0.just_finished() {
        count_down.0.reset();
        next_state.set(GameState::MainMenu);
        return;
    }

    let mut text = query.single_mut();
    let total_time = COUNT_DOWN_SEC - count_down.0.elapsed_secs();
    let minite = (total_time / 60.0).floor();
    let seconds = (total_time % 60.0).floor();

    text.sections[0].value = format!("{:0>2}:{:0>2}", minite, seconds);
}
