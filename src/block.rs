use bevy::prelude::*;

use crate::{
    player::{HandBlock, HandBlockState},
    state::GameState,
    STEP_SIZE,
};

// Block
#[derive(Component, Debug)]
pub struct Block {
    pub index: usize,
    pub show: bool,
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_block_despawn).run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnExit(HandBlockState::Moving), handle_block_down);
    }
}

fn handle_block_despawn(mut commands: Commands, mut query: Query<(&Block, Entity), With<Block>>) {
    if query.is_empty() {
        return;
    }

    // for (block, entity) in query.iter() {
    //     if !block.show {
    //         commands.entity(entity).despawn();
    //     }
    // }
}

fn handle_block_down(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Block, Entity), With<Block>>,
    hand_block_query: Query<&Transform, (With<HandBlock>, Without<Block>)>,
) {
    if query.is_empty() || hand_block_query.is_empty() {
        return;
    }

    let hand_block_transform = hand_block_query.single();
    let mut items_x = Vec::new();

    for (transform, block, entity) in query.iter() {
        if !block.show {
            items_x.push(transform.translation.x);
            commands.entity(entity).despawn();
        }
    }

    for (mut transform, _, _) in query.iter_mut() {
        if items_x.contains(&transform.translation.x)
            && transform.translation.y > hand_block_transform.translation.y
        {
            transform.translation.y -= STEP_SIZE as f32;
        }
    }

    // for _ in events.read() {
    //     println!("throw event");
    // }
}
