//! A simplified implementation of the classic game "Breakout".

use bevy::{
    prelude::*,
    // sprite::collide_aabb::{collide, Collision},
    // sprite::MaterialMesh2dBundle,
    time::{FixedTimestep, FixedTimesteps},
    winit::WinitSettings,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::fmt;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
// const PADDLE_SIZE: Vec3 = Vec3::new(220.0, 20.0, 0.0);
// const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
// const PADDLE_SPEED: f32 = 500.0;
// // How close can the paddle get to the wall
// const PADDLE_PADDING: f32 = 10.0;

// // We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
// const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
// const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
// const BALL_SPEED: f32 = 400.0;
// const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

// const WALL_THICKNESS: f32 = 10.0;
// // x coordinates
// const LEFT_WALL: f32 = -450.;
// const RIGHT_WALL: f32 = 450.;
// // y coordinates
// const BOTTOM_WALL: f32 = -300.;
// const TOP_WALL: f32 = 300.;

const BRICK_SIZE: Vec2 = Vec2::new(20., 20.);

const N_BRICKS_X: usize = 6;
const N_BRICKS_Y: usize = 6;
// const N_BRICKS_X: usize = 50;
// const N_BRICKS_Y: usize = 50;

// These values are exact
// const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 1.0;
// These values are lower bounds, as the number of bricks is computed
// const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
// const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

// const SCOREBOARD_FONT_SIZE: f32 = 40.0;
// const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
// const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
// const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
// const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
// const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const ALIVE_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const DEAD_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const HOVER_COLOR: Color = Color::rgb(0.45, 0.45, 0.45);

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        // .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(Board::new(N_BRICKS_X, N_BRICKS_Y))
        .add_startup_system(setup)
        .add_system(button_system)
        // .add_event::<CollisionEvent>()
        // .add_system_set(
        //     SystemSet::new().with_run_criteria(FixedTimestep::step(TIME_STEP as f64)), // .with_system(check_for_collisions)
        // .with_system(move_paddle.before(check_for_collisions))
        // .with_system(apply_velocity.before(check_for_collisions)), // .with_system(play_collision_sound.after(check_for_collisions)),
        // )
        // .add_system(update_scoreboard)
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel()
                .with_run_criteria(
                    FixedTimestep::step(5.0)
                        // labels are optional. they provide a way to access the current
                        // FixedTimestep state from within a system
                        .with_label("my_time"),
                )
                .with_system(fixed_update),
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn fixed_update(mut query: Query<(&mut BackgroundColor, &mut Cell)>, mut board: ResMut<Board>) {
    // fn fixed_update(mut board: ResMut<Board>) {
    //mut last_time: Local<f32>, time: Res<Time>, fixed_timesteps: Res<FixedTimesteps>) {
    // dbg!(query);
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
    println!("{}", *board)
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

    // asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

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

    // Because we need to round the number of columns,
    // the space on the top and sides of the bricks only captures a lower bound, not an exact value
    // let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    // let left_edge_of_bricks = center_of_bricks
    //     // Space taken up by the bricks
    //     - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
    //     // Space taken up by the gaps
    //     - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

    // In Bevy, the `translation` of an entity describes the center point,
    // not its bottom-left corner
    // let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
    // let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;

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

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut Cell), //, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    buttons: Res<Input<MouseButton>>,
    mut board: ResMut<Board>,
) {
    for (interaction, mut color, mut cell) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                board.set(cell.pos_x, cell.pos_y, !cell.alive);
                println!("{}", *board);
                toggle_cell(cell, color);
            }
            Interaction::Hovered => {
                if buttons.pressed(MouseButton::Left) {
                    board.set(cell.pos_x, cell.pos_y, !cell.alive);
                    println!("{}", *board);
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
