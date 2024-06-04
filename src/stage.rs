use bevy::prelude::*;

use crate::state::GameState;
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
            );
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
