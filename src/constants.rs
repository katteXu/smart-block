use std::ops::RangeInclusive;

// Window
pub const WW: f32 = 1200.0;
pub const WH: f32 = 720.0;

// Sprite sheet
pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const TILES_W: usize = 16;
pub const TILES_H: usize = 16;
pub const SPRITE_SHEET_W: usize = 4;
pub const SPRITE_SHEET_H: usize = 4;
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;

// Step
pub const STEP_SIZE: usize = 48;

// Colors
pub const BG_COLOR: (u8, u8, u8) = (197, 204, 184);

// Player
pub const PLAYER_INIT_POS: (f32, f32) = (240.0, -290.0);

// Kd Tree
pub const KD_TREE_REFRESH_RATE: f32 = 0.2;

// Block
pub const BLOCK_NUM_W: usize = 5;
pub const BLOCK_NUM_H: usize = 6;
pub const BLOCK_INIT_POS: (f32, f32) = (-528.0, -290.0);
pub const BLOCK_DISPLAY_RANGE: RangeInclusive<usize> = 8..=11;
pub const HAND_BLOCK_SPEED: f32 = 1280.0;
pub const HAND_BLOCK_INDEX: usize = 8;

//
// 8-13
pub const TEST_BLOCK_POS: [[usize; 5]; 4] = [
    // stage1
    // [13, 11, 11, 11, 0],
    // [8, 13, 13, 13, 0],
    // [9, 9, 9, 9, 0],
    // [11, 8, 8, 8, 0],
    // stage2
    [13, 8, 8, 8, 0],
    [11, 13, 13, 13, 0],
    [9, 8, 8, 8, 0],
    [8, 13, 13, 13, 0],
];
