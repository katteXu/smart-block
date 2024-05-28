use bevy::prelude::*;

use crate::{
    player::Player,
    state::{GameState, PlayerState},
};

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_timer_tick, player_animation).run_if(in_state(GameState::InGame)),
        );
    }
}

fn animate_timer_tick(
    time: Res<Time>,
    mut query: Query<&mut AnimationTimer, With<AnimationTimer>>,
) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn player_animation(
    mut query: Query<(&mut TextureAtlas, &PlayerState, &AnimationTimer), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (mut atlas, player_state, timer) = query.single_mut();

    if timer.0.just_finished() {
        match player_state {
            PlayerState::Idle => atlas.index = 0,
            PlayerState::Moving => atlas.index = 1,
            PlayerState::Throwing => atlas.index = 2,
        }
    }
}
