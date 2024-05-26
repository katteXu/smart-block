use bevy::{prelude::*, window::close_on_esc};
use bevy_smart_block::animation::AnimationPlugin;
use bevy_smart_block::block::BlockPlugin;
use bevy_smart_block::camera::MyCameraPlugin;
use bevy_smart_block::gui::GuiPlugin;
use bevy_smart_block::player::PlayerPlugin;
use bevy_smart_block::resources::ResourcesPlugin;
use bevy_smart_block::state::GameState;
use bevy_smart_block::world::WorldPlugin;
use bevy_smart_block::*;

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
        // add plugins
        .add_plugins(MyCameraPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(BlockPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(AnimationPlugin)
        .add_systems(Update, close_on_esc)
        .run();
}
