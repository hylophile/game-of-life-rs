use bevy::{prelude::*, time::FixedTimestep};

use self::{
    board::Board, create_board::spawn_board, draw_on_board::draw_on_board_system,
    update_board::update_board_system,
};

mod board;
mod create_board;
mod draw_on_board;
mod update_board;

const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
pub const ALIVE_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const DEAD_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const HOVER_COLOR: Color = Color::rgb(0.45, 0.45, 0.45);

const TIME_STEP: f64 = 1.0 / 15.0;
const N_CELLS_X: usize = 80;
const N_CELLS_Y: usize = 80;
const CELL_SIZE: Vec2 = Vec2::new(5., 5.);
const GAP_BETWEEN_CELLS: f32 = 0.0;

#[derive(Component, Debug)]
pub struct Cell {
    alive: bool,
    pos_x: usize,
    pos_y: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct FixedUpdateStage;

pub struct BoardPlugin {}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            // .insert_resource(WinitSettings::desktop_app())
            .insert_resource(Board::new(N_CELLS_X, N_CELLS_Y))
            .add_startup_system(spawn_board)
            .add_system(draw_on_board_system)
            .add_stage_after(
                CoreStage::Update,
                FixedUpdateStage,
                SystemStage::parallel()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP).with_label("my_time"))
                    .with_system(update_board_system),
            );
    }
}
