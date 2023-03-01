//! A simplified implementation of the classic game "Breakout".

use bevy::{
    prelude::*,
    reflect::erased_serde::__private::serde::forward_to_deserialize_any,
    // sprite::collide_aabb::{collide, Collision},
    // sprite::MaterialMesh2dBundle,
    time::{FixedTimestep, FixedTimesteps},
    winit::WinitSettings,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::fmt;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

const BRICK_SIZE: Vec2 = Vec2::new(10., 10.);

const N_BRICKS_X: usize = 100;
const N_BRICKS_Y: usize = 100;

const GAP_BETWEEN_BRICKS: f32 = 1.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const ALIVE_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const DEAD_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const HOVER_COLOR: Color = Color::rgb(0.45, 0.45, 0.45);

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugin(WorldInspectorPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        // .insert_resource(WinitSettings::desktop_app())
        .insert_resource(Board::new(N_BRICKS_X, N_BRICKS_Y))
        .insert_resource(Playing(false))
        .add_startup_system(setup)
        .add_system(button_system)
        .add_system(play_pause_system)
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(
                    FixedTimestep::step(1. / 60.)
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
    // info!(
    //     "time since last fixed_update: {}\n",
    //     time.raw_elapsed_seconds() - *last_time
    // );

    // let state = fixed_timesteps.get("my_time").unwrap();

    // info!("fixed timestep: {}\n", 0.5);
    // info!(
    //     "time accrued toward next fixed_update: {}\n",
    //     state.accumulator()
    // );
    // info!(
    //     "time accrued toward next fixed_update (% of timestep): {}",
    //     state.overstep_percentage()
    // );
    // *last_time = time.raw_elapsed_seconds();
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

#[derive(Resource)]
struct Board {
    tiles: Vec<(bool, Entity)>,
    width: usize,
    height: usize,
}

impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tiles = self.tiles();
        for x in 0..self.width {
            for y in 0..self.height {
                match self.get(x, y) {
                    true => write!(f, "X ({}) | ", tiles.alive_neighbors(x, y))?,
                    false => write!(f, "_ ({}) | ", tiles.alive_neighbors(x, y))?,
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

const NEIGBOURHOOD_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

struct Tiles {
    tiles: Vec<bool>,
    width: usize,
    height: usize,
}

impl Tiles {
    fn get(&self, x: usize, y: usize) -> bool {
        self.tiles[y * self.width + x]
    }

    fn alive_neighbors(&self, x: usize, y: usize) -> usize {
        NEIGBOURHOOD_OFFSETS
            .iter()
            .map(|(x_d, y_d)| {
                (
                    (x as isize + x_d).rem_euclid(self.width as isize),
                    (y as isize + y_d).rem_euclid(self.height as isize),
                )
            })
            .filter(|(x_q, y_q)| self.get(*x_q as usize, *y_q as usize))
            .count()
    }

    fn step_cell(&self, x: usize, y: usize) -> bool {
        let n = self.alive_neighbors(x, y);
        match self.get(x, y) {
            true => {
                if n < 2 || n > 3 {
                    // self.set(x, y, false)
                    return false;
                }
                return true;
            }
            false => {
                if n == 3 {
                    // self.set(x, y, true)
                    return true;
                }
                return false;
            }
        }
    }
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: vec![(false, Entity::from_raw(0)); width * height],
            width,
            height,
        }
    }

    fn tiles(&self) -> Tiles {
        Tiles {
            tiles: self.tiles.iter().map(|t| t.0).collect(),
            width: self.width,
            height: self.height,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.tiles[y * self.width + x].0
    }

    fn get_entity(&self, x: usize, y: usize) -> Entity {
        self.tiles[y * self.width + x].1
    }

    fn set_entity(&mut self, x: usize, y: usize, e: Entity) {
        self.tiles[y * self.width + x].1 = e
    }

    fn set(&mut self, x: usize, y: usize, v: bool) {
        self.tiles[y * self.width + x].0 = v
    }
}

#[test]
fn test_board() {
    let mut b = Board::new(4, 4);
    b.set(0, 0, true);
    b.set(0, 1, true);
    b.set(0, 2, true);
    // b.set(1, 1, true);
    // b.set(3, 3, true);
    // println!("{}", b.alive_neighbors(1, 1));
    println!("{}", b);
}

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
    // Bricks
    // Negative scales result in flipped sprites / meshes,
    // which is definitely not what we want here
    assert!(BRICK_SIZE.x > 0.0);
    assert!(BRICK_SIZE.y > 0.0);

    // let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    // // let bottom_edge_of_bricks = 0.0; //paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
    // let bottom_edge_of_bricks = BOTTOM_WALL + GAP_BETWEEN_BRICKS_AND_SIDES;
    // let total_height_of_bricks = TOP_WALL - BOTTOM_WALL - GAP_BETWEEN_BRICKS_AND_SIDES;

    // assert!(total_width_of_bricks > 0.0);
    // assert!(total_height_of_bricks > 0.0);

    // Given the space available, compute how many rows and columns of bricks we can fit
    let n_columns = N_BRICKS_X as usize; //(total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_rows = N_BRICKS_Y as usize; //(total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
                                      // let n_vertical_gaps = n_columns - 1;

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
                // transform: Transform {
                //     translation: brick_position.extend(0.0),
                //     // scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                //     ..default()
                // },
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
            let brick_position = Vec2::new(
                100.0 + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                // offset_y +
                10.0 + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            let entity = commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(BRICK_SIZE.x), Val::Px(BRICK_SIZE.y)),
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                left: Val::Px(brick_position.x),
                                top: Val::Px(brick_position.y),
                                ..default()
                            },
                            ..default()
                        },
                        background_color: DEAD_COLOR.into(),
                        // transform: Transform {
                        //     translation: brick_position.extend(0.0),
                        //     // scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                        //     ..default()
                        // },
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
    query: Query<(&Interaction, &Children), (Changed<Interaction>, With<PlayPauseButton>)>,
    mut text_query: Query<&mut Text>,
    mut playing: ResMut<Playing>,
) {
    // let interaction = query;
    // match *interaction {}
    // dbg!(query);
    for (interaction, children) in query.iter() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match interaction {
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
            _ => (),
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
                // println!("{}", *board);
                toggle_cell(cell, color);
            }
            Interaction::Hovered => {
                if buttons.pressed(MouseButton::Left) {
                    board.set(cell.pos_x, cell.pos_y, !cell.alive);
                    // println!("{}", *board);
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
