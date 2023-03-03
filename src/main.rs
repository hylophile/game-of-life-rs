//! A simplified implementation of the classic game "Breakout".

use bevy::{
    prelude::*,
    time::{FixedTimestep, FixedTimesteps},
    window::PresentMode,
    winit::WinitSettings,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board::Board;
use std::fmt;

mod board;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

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
        .insert_resource(Playing(false))
        .add_startup_system(setup)
        .add_system(button_system)
        .add_system(play_pause_system)
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(
                    FixedTimestep::step(1. / 15.)
                        // labels are optional. they provide a way to access the current
                        // FixedTimestep state from within a system
                        .with_label("my_time"),
                )
                .with_system(fixed_update),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn fixed_update(
    mut query: Query<(&mut BackgroundColor, &mut Cell)>,
    mut board: ResMut<Board>,
    playing: Res<Playing>,
) {
    // fn fixed_update(mut board: ResMut<Board>) {
    //mut last_time: Local<f32>, time: Res<Time>, fixed_timesteps: Res<FixedTimesteps>) {
    // dbg!(query);
    if playing.0 {
        let tiles = board.tiles();

        for x in 0..board.width {
            for y in 0..board.height {
                let current_state = tiles.get(x, y);
                let new_state = tiles.step_cell(x, y);
                // dbg!(x, y, board.alive_neighbors(x, y), current_state, new_state);
                if current_state != new_state {
                    board.set(x, y, new_state);
                    let e = board.get_entity(x, y);
                    let (mut color, mut cell): (Mut<BackgroundColor>, Mut<Cell>) =
                        query.get_mut(e).unwrap();
                    // let a = board.get(x, y);

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
                // dbg!(x);
            }
        }
        // println!("{}", *board)
    }
}

#[derive(Component, Debug)]
struct Cell {
    alive: bool,
    pos_x: usize,
    pos_y: usize,
}

#[derive(Component)]
struct PlayPauseButton;

#[derive(Resource, Debug)]
struct Playing(bool);

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut board: ResMut<Board>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    let font = asset_server.load("fonts/agave-r.ttf");
    assert!(CELL_SIZE.x > 0.0);
    assert!(CELL_SIZE.y > 0.0);

    let n_columns = N_CELLS_X as usize;
    let n_rows = N_CELLS_Y as usize;

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(80.0), Val::Px(30.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(10.0),
                        top: Val::Px(10.0),
                        ..default()
                    },
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: DEAD_COLOR.into(),
                ..default()
            },
            PlayPauseButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play",
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::rgb(0.1, 0.1, 0.1),
                },
            ));
        });

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

fn play_pause_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<PlayPauseButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut playing: ResMut<Playing>,
) {
    for (interaction, mut color, children) in &mut query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                let currently_playing = playing.0;
                if currently_playing {
                    text.sections[0].value = "Play".to_string();
                    *playing = Playing(false);
                } else {
                    text.sections[0].value = "Pause".to_string();
                    *playing = Playing(true);
                }
            }
            Interaction::Hovered => *color = Color::rgb(0.7, 0.7, 0.7).into(),
            Interaction::None => *color = DEAD_COLOR.into(),
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
