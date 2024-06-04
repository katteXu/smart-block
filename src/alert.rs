use std::borrow::Borrow;

use bevy::{ecs::event, prelude::*};

use crate::state::GameState;

pub struct AlertPlugin;

impl Plugin for AlertPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DespawnAlertTextTimer>()
            .add_event::<AlertEvent>()
            .add_systems(
                Update,
                (spawn_game_alert, despawn_alert_text).run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Event)]
pub struct AlertEvent(pub Option<String>);
impl Default for AlertEvent {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Component)]
struct AlertText;

#[derive(Resource)]
struct DespawnAlertTextTimer(Timer);
impl Default for DespawnAlertTextTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Repeating))
    }
}

// 生成提示文案
fn spawn_game_alert(mut commands: Commands, mut event: EventReader<AlertEvent>) {
    for e in event.read() {
        if let Some(text) = &e.0 {
            // 生成提示文案
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font_size: 24.0,
                            color: Color::RED,
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0.0, 200.0, 1.0),
                    ..default()
                },
                AlertText,
            ));
        }
    }
}

// 销毁提示文案  1s延时
fn despawn_alert_text(
    time: Res<Time>,
    mut commands: Commands,
    mut timer: ResMut<DespawnAlertTextTimer>,
    mut text_query: Query<Entity, With<AlertText>>,
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
