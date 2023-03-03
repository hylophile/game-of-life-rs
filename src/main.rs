//! A simplified implementation of the classic game "Breakout".

use bevy::{
    prelude::*,
    time::FixedTimestep,
    //, FixedTimesteps},
    window::PresentMode,
    // winit::WinitSettings,
};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board::Board;
use menu_plugin::*;

mod board;
mod menu_plugin;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f64 = 1.0 / 15.0;
const CELL_SIZE: Vec2 = Vec2::new(10., 10.);
const N_CELLS_X: usize = 80;
const N_CELLS_Y: usize = 80;
const GAP_BETWEEN_CELLS: f32 = 1.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const ALIVE_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const DEAD_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const HOVER_COLOR: Color = Color::rgb(0.45, 0.45, 0.45);

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        // .insert_resource(WinitSettings::desktop_app())
        .insert_resource(Board::new(N_CELLS_X, N_CELLS_Y))
        .add_startup_system(setup)
        .add_plugin(MenuPlugin {})
        .add_system(button_system)
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(TIME_STEP).with_label("my_time"))
                .with_system(fixed_update),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn fixed_update(
    mut query: Query<(&mut BackgroundColor, &mut Cell)>,
    mut board: ResMut<Board>,
    config: Res<Config>,
) {
    if config.playing {
        let tiles = board.tiles();

        for x in 0..board.width {
            for y in 0..board.height {
                let current_state = tiles.get(x, y);
                let new_state = tiles.step_cell(x, y);
                if current_state != new_state {
                    board.set(x, y, new_state);
                    let e = board.get_entity(x, y);
                    let (mut color, mut cell): (Mut<BackgroundColor>, Mut<Cell>) =
                        query.get_mut(e).unwrap();

                    match new_state {
                        true => {
                            *color = ALIVE_COLOR.into();
                            cell.alive = true
                        }
                        false => {
                            *color = DEAD_COLOR.into();
                            cell.alive = false;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Component, Debug)]
struct Cell {
    alive: bool,
    pos_x: usize,
    pos_y: usize,
}

fn setup(
    mut commands: Commands,
    mut board: ResMut<Board>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    assert!(CELL_SIZE.x > 0.0);
    assert!(CELL_SIZE.y > 0.0);

    let n_columns = N_CELLS_X as usize;
    let n_rows = N_CELLS_Y as usize;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let cell_position = Vec2::new(
                100.0 + column as f32 * (CELL_SIZE.x + GAP_BETWEEN_CELLS),
                // offset_y +
                10.0 + row as f32 * (CELL_SIZE.y + GAP_BETWEEN_CELLS),
            );

            let entity = commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(CELL_SIZE.x), Val::Px(CELL_SIZE.y)),
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                left: Val::Px(cell_position.x),
                                top: Val::Px(cell_position.y),
                                ..default()
                            },
                            ..default()
                        },
                        background_color: DEAD_COLOR.into(),
                        ..default()
                    },
                    Cell {
                        alive: false,
                        pos_x: row,
                        pos_y: column,
                    },
                ))
                .id();
            board.set_entity(row, column, entity);
            // println!("{}", entity);
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut Cell), //, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    buttons: Res<Input<MouseButton>>,
    mut board: ResMut<Board>,
) {
    for (interaction, mut color, cell) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                board.set(cell.pos_x, cell.pos_y, !cell.alive);
                toggle_cell(cell, color);
            }
            Interaction::Hovered => {
                if buttons.pressed(MouseButton::Left) {
                    board.set(cell.pos_x, cell.pos_y, !cell.alive);
                    toggle_cell(cell, color);
                } else if !cell.alive {
                    *color = HOVER_COLOR.into();
                }
            }
            Interaction::None => {
                if !cell.alive {
                    *color = DEAD_COLOR.into();
                }
            }
        }
    }
}

fn toggle_cell(mut cell: Mut<Cell>, mut color: Mut<BackgroundColor>) {
    match cell.alive {
        true => {
            *color = DEAD_COLOR.into();
            cell.alive = false;
        }
        false => {
            *color = ALIVE_COLOR.into();
            cell.alive = true
        }
    }
}
