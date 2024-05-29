use bevy::prelude::*;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Ground;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, _app: &mut App) {}
}
