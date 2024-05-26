use bevy::prelude::*;

// Block
#[derive(Component)]
pub struct Block;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_startup_system(setup_block)
        //     .add_system(move_block);
    }
}
