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

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextAtlas::default())
            .add_systems(OnEnter(GameState::Loading), load_assets);
    }
}

fn load_assets(
    mut handle: ResMut<GlobalTextAtlas>,
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

    next_state.set(GameState::MainMenu);
}
