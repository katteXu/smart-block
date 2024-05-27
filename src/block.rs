use bevy::prelude::*;

use crate::state::GameState;

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
            (handle_block_down, handle_block_despawn).run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_block_despawn(mut commands: Commands, mut query: Query<(&Block, Entity), With<Block>>) {
    if query.is_empty() {
        return;
    }

    for (block, entity) in query.iter() {
        if !block.show {
            commands.entity(entity).despawn();
        }
    }
}

fn handle_block_down(
    mut _query: Query<(&mut Transform, &Block), With<Block>>,
    // mut events: EventReader<PlayerThrowEvent>,
) {
    // for (mut transform, block) in query.iter_mut() {
    //     // println!("{:?} {:?}", block, transform);
    // }

    // for _ in events.read() {
    //     println!("throw event");
    // }
}
