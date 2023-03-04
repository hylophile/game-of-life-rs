//! A simplified implementation of the classic game "Breakout".

use bevy::{
    prelude::*,
    // time::FixedTimestep,
    //, FixedTimesteps},
    window::PresentMode,
    // winit::WinitSettings,
};
use board_plugin::BoardPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use board_plugin::board::Board;
use menu_plugin::*;

mod board_plugin;
mod menu_plugin;

fn main() {
    App::new()
        // .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Conway's Game of Life".to_string(),
                width: 1280.,
                height: 900.,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_plugin(MenuPlugin {})
        .add_plugin(BoardPlugin {})
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    // mut board: ResMut<Board>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}
