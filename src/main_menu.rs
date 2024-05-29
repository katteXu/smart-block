use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::state::GameState;

#[derive(Component)]
pub struct MainMenuItem;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
            );
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
        })
        .insert(MainMenuItem);
}

fn despawn_main_menu(main_menu_query: Query<Entity, With<MainMenuItem>>, mut commands: Commands) {
    // despawn main menu
    for entity in main_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_main_menu_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text, With<Text>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut text = text_query.single_mut();
    for (interaction, mut bg_color, mut border_color, _) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                next_state.set(GameState::GameInit);
            }
            Interaction::Hovered => {
                *bg_color = Color::YELLOW.into();
                *border_color = Color::BLUE.into();
                text.sections[0].style.color = Color::BLUE;
            }
            Interaction::None => {
                *bg_color = Color::WHITE.into();
                *border_color = Color::BLACK.into();
                text.sections[0].style.color = Color::BLACK;
            }
        }
    }
}
