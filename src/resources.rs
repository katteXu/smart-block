use bevy::prelude::*;

use crate::state::GameState;
use crate::*;
// Resource
#[derive(Resource)]
pub struct GlobalTextAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

impl Default for GlobalTextAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}

#[derive(Resource)]
pub struct GlobalAudio {
    // 用户移动
    pub player_move: Option<Handle<AudioSource>>,
    // 用户投掷
    pub player_throw: Option<Handle<AudioSource>>,
    // 手里块
    pub hand_block_hit_block: Option<Handle<AudioSource>>,
    pub hand_block_hit_wall: Option<Handle<AudioSource>>,
    pub hand_block_hit_groud: Option<Handle<AudioSource>>,
    pub hand_block_black: Option<Handle<AudioSource>>,
    // 方块
    pub block_fall_down: Option<Handle<AudioSource>>,
    pub block_despawn: Option<Handle<AudioSource>>,

    // 时间清空
    pub time_clear: Option<Handle<AudioSource>>,
    // 背景音乐
    pub background_music: Option<Handle<AudioSource>>,
}
impl Default for GlobalAudio {
    fn default() -> Self {
        Self {
            player_move: None,
            player_throw: None,
            hand_block_hit_block: None,
            hand_block_hit_wall: None,
            hand_block_hit_groud: None,
            block_fall_down: None,
            block_despawn: None,
            background_music: None,
            hand_block_black: None,
            time_clear: None,
        }
    }
}

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextAtlas::default())
            .insert_resource(GlobalAudio::default())
            .add_systems(OnEnter(GameState::Loading), load_assets);
    }
}

fn load_assets(
    mut handle: ResMut<GlobalTextAtlas>,
    mut audio_handle: ResMut<GlobalAudio>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // 资源图片
    handle.image = Some(asset_server.load(SPRITE_SHEET_PATH));

    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(TILES_W as f32, TILES_H as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );

    // 资源网格
    handle.layout = Some(texture_atlas_layouts.add(layout));

    // 资源声音
    audio_handle.player_move = Some(asset_server.load("embedded://audio/move.ogg"));
    audio_handle.player_throw = Some(asset_server.load("embedded://audio/throw.ogg"));
    audio_handle.hand_block_black = Some(asset_server.load("embedded://audio/back.ogg"));
    audio_handle.hand_block_hit_block = Some(asset_server.load("embedded://audio/hit_block.wav"));
    // audio_handle.block_fall_down = Some(asset_server.load("embedded://audio/block_fall_down.wav"));
    audio_handle.time_clear = Some(asset_server.load("embedded://audio/success_bell.wav"));

    audio_handle.background_music = Some(asset_server.load("embedded://audio/bgm.mp3"));

    next_state.set(GameState::MainMenu);
}
