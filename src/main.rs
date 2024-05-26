use bevy::{prelude::*, window::close_on_esc};

// Window
const WW: f32 = 1200.0;
const WH: f32 = 700.0;

// Sprite sheet
const SPRITE_SHEET_PATH: &str = "assets.png";
const TILES_W: usize = 16;
const TILES_H: usize = 16;
const SPRITE_SHEET_W: usize = 4;
const SPRITE_SHEET_H: usize = 4;
const SPRITE_SCALE_FACTOR: f32 = 3.0;

// Colors
const BG_COLOR: (u8, u8, u8) = (197, 204, 184);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .add_systems(Startup, (setup_camera, setup_assets))
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(SPRITE_SHEET_PATH);

    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(TILES_W as f32, TILES_H as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );

    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(
        (SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..Default::default()
        }),
    );
}
