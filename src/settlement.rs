use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use crate::state::{GameState, SettlementState};

use crate::block::Block;
use crate::world::GameEntity;

pub struct SettlementPlugin;

// #[derive(Resource)]
// struct DespawnRemainderBlockTimer(Timer);
// impl Default for DespawnRemainderBlockTimer {
//     fn default() -> Self {
//         Self(Timer::from_seconds(0.8, TimerMode::Once))
//     }
// }

impl Plugin for SettlementPlugin {
    fn build(&self, app: &mut App) {
        app
            // .init_resource::<DespawnRemainderBlockTimer>()
            .init_state::<SettlementState>()
            .add_systems(
                OnEnter(SettlementState::Start),
                spawn_settlement.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    settle_total_score,
                    despawn_remainder_block.run_if(on_timer(Duration::from_secs_f32(0.6))),
                )
                    .run_if(in_state(SettlementState::Start)),
            );
    }
}

// 生成结算
fn spawn_settlement(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
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
                    "Total Score",
                    TextStyle {
                        font_size: 64.0,
                        ..default()
                    },
                ),
                ..default()
            });
        });
}

// 结算总分数
fn settle_total_score(mut next_state: ResMut<NextState<GameState>>) {
    println!("计算总分数");

    next_state.set(GameState::MainMenu);
}

// 销毁剩余方块
fn despawn_remainder_block(
    mut commands: Commands,
    mut block_query: Query<(&mut TextureAtlas, Entity), With<Block>>,
) {
    if block_query.is_empty() {
        return;
    }

    for (mut _text_atlas, entity) in block_query.iter_mut() {
        commands.entity(entity).despawn();
        return;
    }
}

// 计算剩余时间获得分数
fn _time_to_score() {}
