use bevy::prelude::*;

use crate::{
    gui::HighScore,
    player::Player,
    state::{GameState, PlayerState},
    HIGH_SCORE_ANIMATION_SPEED,
};

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_timer_tick, player_animation).run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            (high_score_animation).run_if(in_state(GameState::InGame)),
        );
    }
}

// 动画计时器
fn animate_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

// 玩家动画  变更上下或者抛出方块
fn player_animation(
    mut query: Query<(&mut TextureAtlas, &mut PlayerState, &mut AnimationTimer), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (mut atlas, mut player_state, mut timer) = query.single_mut();

    match *player_state {
        PlayerState::Idle => atlas.index = 0,
        PlayerState::Moving => atlas.index = 1,
        PlayerState::Throwing => atlas.index = 2,
    };

    if timer.0.finished() {
        timer.0.reset();
        match *player_state {
            PlayerState::Idle => {}
            PlayerState::Moving => *player_state = PlayerState::Idle,
            PlayerState::Throwing => *player_state = PlayerState::Idle,
        }
    }
}

// 生成惊艳分数动画
fn high_score_animation(
    time: Res<Time>,
    mut commands: Commands,
    mut high_socre_query: Query<(&mut Transform, Entity, &mut HighScore), With<HighScore>>,
) {
    if high_socre_query.is_empty() {
        return;
    }

    let (mut transform, entity, mut high_score) = high_socre_query.single_mut();

    high_score.animation_timer.tick(time.delta());

    // 动画结束时销毁
    if high_score.animation_timer.just_finished() {
        commands.entity(entity).despawn_recursive();
    }
    transform.translation.y += HIGH_SCORE_ANIMATION_SPEED * time.delta_seconds();
}
