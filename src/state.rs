use bevy::prelude::*;

// 游戏状态
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Copy, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    GameInit,
    InGame,
}

// 玩家状态
#[derive(Component, Default, PartialEq, Eq)]
pub enum PlayerState {
    #[default]
    Idle,
    Moving,
    Throwing,
}

// 手里方块状态
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Copy, States)]
pub enum HandBlockState {
    #[default]
    Idle,
    Moving,
    Backing,
}

// 方块组装填
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Copy, States)]
pub enum BlockGroupState {
    #[default]
    Static,
    FallDown,
}

// 游戏结束 结算状态
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Copy, States)]
pub enum SettlementState {
    #[default]
    Not,
    Start,
    DespawnBlock,
    TimeToScore,
    End,
}
