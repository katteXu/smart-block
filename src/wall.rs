use bevy::prelude::*;

#[derive(Component)]
pub struct Wall;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, _app: &mut App) {}
}
