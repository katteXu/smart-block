use bevy::{math::vec3, prelude::*, window::close_on_esc};
use rand::Rng;

// Window
const WW: f32 = 1200.0;
const WH: f32 = 720.0;

// Sprite sheet
const SPRITE_SHEET_PATH: &str = "assets.png";
const TILES_W: usize = 16;
const TILES_H: usize = 16;
const SPRITE_SHEET_W: usize = 4;
const SPRITE_SHEET_H: usize = 4;
const SPRITE_SCALE_FACTOR: f32 = 3.0;

// Step
const STEP_SIZE: usize = 48;

// Colors
const BG_COLOR: (u8, u8, u8) = (197, 204, 184);

// Player
const PLAYER_INIT_POS: (f32, f32) = (240.0, -290.0);

// ladder

// Block
const BLOCK_NUM_W: usize = 6;
const BLOCK_NUM_H: usize = 5;
const BLOCK_INIT_POS: (f32, f32) = (-528.0, -290.0);
// Resource
#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);

#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

// State
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Copy, States)]
enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}

// Player
#[derive(Component)]
struct Player;

// Block
#[derive(Component)]
struct Block;

#[derive(Component)]
struct HandBlock;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
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
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(OnEnter(GameState::GameInit), (setup_camera, init_game))
        .add_systems(
            Update,
            (handle_player_movement, handle_block_movement).run_if(in_state(GameState::InGame)),
            // .run_if(on_timer(Duration::from_secs_f32(0.2))),
        )
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_assets(
    mut texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    mut image_handle: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // 资源图片
    image_handle.0 = Some(asset_server.load(SPRITE_SHEET_PATH));

    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(TILES_W as f32, TILES_H as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );

    // 资源网格
    texture_atlas.0 = Some(texture_atlas_layouts.add(layout));

    next_state.set(GameState::GameInit);
}

fn init_game(
    mut commands: Commands,
    texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    image_handle: ResMut<GlobalSpriteSheetHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut rng = rand::thread_rng();

    // 生成玩家
    let (x, y) = PLAYER_INIT_POS;
    commands.spawn((
        SpriteSheetBundle {
            texture: image_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 0,
            },
            transform: Transform::from_translation(vec3(x, y, 1.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Player,
    ));

    // 生成梯子
    for i in 0..100 {
        commands.spawn(SpriteSheetBundle {
            texture: image_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 4,
            },
            transform: Transform::from_translation(vec3(x, y + (i * STEP_SIZE) as f32, 0.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        });
    }

    // 生成手上方块
    commands.spawn((
        SpriteSheetBundle {
            texture: image_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: rng.gen_range(8..=13),
            },
            transform: Transform::from_translation(vec3(x - STEP_SIZE as f32, y, 0.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..Default::default()
        },
        HandBlock,
    ));

    // 生成墙面
    for i in 1..=25 {
        for j in 0..=14 {
            if j == 0 || j == 14 {
                commands.spawn(SpriteSheetBundle {
                    texture: image_handle.0.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: texture_atlas.0.clone().unwrap(),
                        index: 6,
                    },
                    transform: Transform::from_translation(vec3(
                        -(WW + STEP_SIZE as f32) / 2.0 + (i * STEP_SIZE) as f32,
                        (WH - STEP_SIZE as f32) / 2.0 - (j * STEP_SIZE) as f32,
                        1.0,
                    ))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    ..default()
                });
            } else if i == 1 || i >= 19 {
                commands.spawn(SpriteSheetBundle {
                    texture: image_handle.0.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: texture_atlas.0.clone().unwrap(),
                        index: 6,
                    },
                    transform: Transform::from_translation(vec3(
                        -(WW + STEP_SIZE as f32) / 2.0 + (i * STEP_SIZE) as f32,
                        (WH - STEP_SIZE as f32) / 2.0 - (j * STEP_SIZE) as f32,
                        1.0,
                    ))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    ..default()
                });
            }
        }
    }

    // 生成方块组
    let (x, y) = BLOCK_INIT_POS;
    for i in 0..BLOCK_NUM_W {
        for j in 0..BLOCK_NUM_H {
            commands.spawn((
                SpriteSheetBundle {
                    texture: image_handle.0.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: texture_atlas.0.clone().unwrap(),
                        index: rng.gen_range(8..=13),
                    },
                    transform: Transform::from_translation(vec3(
                        x + (i * STEP_SIZE) as f32,
                        y + (j * STEP_SIZE) as f32,
                        0.0,
                    ))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    ..Default::default()
                },
                Block,
            ));
        }
    }

    next_state.set(GameState::InGame);
}

fn handle_player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut player_transform = player_query.single_mut();

    let w_key =
        keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp);

    let s_key =
        keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);

    let mut delta = Vec3::ZERO;

    // 只有上下操作
    if w_key && player_transform.translation.y < -PLAYER_INIT_POS.1 - STEP_SIZE as f32 {
        delta.y += 1.0;
    }
    if s_key && player_transform.translation.y > PLAYER_INIT_POS.1 {
        delta.y -= 1.0;
    }

    let delta = delta.normalize_or_zero();

    player_transform.translation += vec3(delta.x, delta.y, 0.0) * (STEP_SIZE as f32);
}

fn handle_block_movement(
    player_query: Query<&Transform, With<Player>>,
    mut hand_block_query: Query<&mut Transform, (With<HandBlock>, Without<Player>)>,
) {
    if player_query.is_empty() || hand_block_query.is_empty() {
        return;
    }

    let player_transform = player_query.single();

    let mut hand_block_transform = hand_block_query.single_mut();

    hand_block_transform.translation.y = player_transform.translation.y;
}
