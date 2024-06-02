use std::ops::RangeInclusive;

// Window
pub const WW: f32 = 1200.0;
pub const WH: f32 = 720.0;

// Sprite sheet
pub const SPRITE_SHEET_PATH: &str = "embedded://assets.png";
pub const FONT_PATH: &str = "embedded://fonts/font.ttf";
pub const TILES_W: usize = 16;
pub const TILES_H: usize = 16;
pub const SPRITE_SHEET_W: usize = 4;
pub const SPRITE_SHEET_H: usize = 4;
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;

// Step
pub const STEP_SIZE: usize = 48;

// Colors
pub const BG_COLOR: (u8, u8, u8) = (74, 91, 198);

// Player
pub const PLAYER_INIT_POS: (f32, f32) = (240.0, -288.0);

// Ladder
pub const LADDER_NUM: usize = 13;

// Arrow
pub const ARROW_TEXTATLAS_INDEX: usize = 7;

// Kd Tree
pub const KD_TREE_REFRESH_RATE: f32 = 0.2;

// Block
pub const BLOCK_NUM_W: usize = 4;
pub const BLOCK_NUM_H: usize = 4;
pub const BLOCK_INIT_POS: (f32, f32) = (-528.0, -288.0);
pub const BLOCK_DISPLAY_RANGE: RangeInclusive<usize> = 8..=11;
pub const HAND_BLOCK_SPEED: f32 = 1280.0;
pub const HAND_BLOCK_INDEX: usize = 15;
pub const FALL_DOWN_TIMER: f32 = 0.12;
pub const BLOCK_BEFORE_REMOVE_INDEX: usize = 14;
// 闪电 万能块索引
pub const LIGHT_BLOCK_INDEX: usize = 15;
// UI text
pub const SCORE_TEXT: &str = "SCORE";
pub const BLOCK_TEXT: &str = "BLOCK";
pub const CLEAR_TEXT: &str = "CLEAR";
pub const STAGE_TEXT: &str = "STAGE";

// UI SCORE
pub const SCORE_BLOCK_WIDTH: f32 = 220.0;
pub const SCORE_BLOCK_POS: (f32, f32) = (48.0, 18.0);
pub const ONEC_BLOCK_SCORE: u32 = 100;
pub const HIGH_SCORE_POS_PERCENT: (f32, f32) = (-300.0, 100.0);
pub const HIGH_SCORE_ANIMATION_DURATION: f32 = 0.5;
pub const HIGH_SCORE_ANIMATION_SPEED: f32 = 64.0;
pub const EVERY_SECOND_SCORE: u64 = 100;
// UI CLEAR
pub const CLEAR_BLOCK_POS: (f32, f32) = (48.0, 96.0);
pub const CLEAR_NUM: usize = 4;
// UI BLOCK
pub const BLOCK_BLOCK_POS: (f32, f32) = (48.0, 240.0);

// UI COUNT_DOWN
pub const COUNT_DOWN_BLOCK_POS: (f32, f32) = (48.0, 384.0);
pub const COUNT_DOWN_SEC: f32 = 180.0;
// UI STAGE
pub const STAGE_BLOCK_POS: (f32, f32) = (48.0, 528.0);

pub const RIGHT_BLOCK_WIDTH: f32 = 240.0;
pub const RIGHT_BLOCK_HEIGHT: f32 = 96.0;

//
// 元素精灵图索引 8-13
pub const TEST_BLOCK_POS: [[usize; 5]; 4] = [
    // stage1
    [13, 11, 11, 11, 0],
    [8, 13, 13, 13, 0],
    [9, 9, 9, 9, 0],
    [11, 8, 8, 8, 0],
    // stage2
    // [13, 8, 8, 8, 0],
    // [11, 8, 13, 8, 0],
    // [9, 8, 8, 8, 0],
    // [13, 8, 13, 8, 0],

    // stage test
    // [0, 0, 0, 0, 0],
    // [10, 0, 0, 0, 0],
    // [9, 11, 0, 0, 0],
    // [13, 8, 0, 0, 0],
];
