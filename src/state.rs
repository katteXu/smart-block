use bevy::prelude::*;

// State
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Copy, States)]
pub enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}
