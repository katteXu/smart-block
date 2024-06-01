use bevy::{prelude::*, window::close_on_esc};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_smart_block::animation::AnimationPlugin;
use bevy_smart_block::block::{BlockPlugin, HandBlock};
use bevy_smart_block::camera::MyCameraPlugin;
use bevy_smart_block::collision::CollisionPlugin;
use bevy_smart_block::gui::GuiPlugin;
use bevy_smart_block::main_menu::MainMenuPlugin;
use bevy_smart_block::player::PlayerPlugin;
use bevy_smart_block::resources::ResourcesPlugin;
use bevy_smart_block::settlement::SettlementPlugin;
use bevy_smart_block::state::GameState;
use bevy_smart_block::world::WorldPlugin;
use bevy_smart_block::*;
fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Smart Block"),
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
        .add_plugins(MainMenuPlugin)
        .add_plugins(MyCameraPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(BlockPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(SettlementPlugin)
        .add_systems(Update, close_on_esc)
        .add_systems(
            Update,
            test_debug_hand_block.run_if(in_state(GameState::InGame)),
        )
        .run();
}

fn test_debug_hand_block(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut hand_block_query: Query<&mut TextureAtlas, With<HandBlock>>,
) {
    if hand_block_query.is_empty() {
        return;
    }

    let mut texture_atlas = hand_block_query.single_mut();

    let key1 = keyboard_input.just_pressed(KeyCode::Digit1);
    let key2 = keyboard_input.just_pressed(KeyCode::Digit2);
    let key3 = keyboard_input.just_pressed(KeyCode::Digit3);
    let key4 = keyboard_input.just_pressed(KeyCode::Digit4);
    let key5 = keyboard_input.just_pressed(KeyCode::Digit5);

    if key1 {
        texture_atlas.index = 8;
    }

    if key2 {
        texture_atlas.index = 9;
    }

    if key3 {
        texture_atlas.index = 10;
    }

    if key4 {
        texture_atlas.index = 11;
    }

    if key5 {
        texture_atlas.index = 12;
    }
}
