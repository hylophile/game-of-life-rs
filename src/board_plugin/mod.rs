use bevy::{prelude::*, time::FixedTimestep};
use rand::distributions::{Distribution, Uniform};

use crate::menu_plugin::AddNoiseEvent;

use self::{
    board::Board, create_board::spawn_board, draw_on_board::draw_on_board_system,
    update_board::update_board_system,
};

pub mod board;
mod create_board;
mod draw_on_board;
mod update_board;

const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
pub const ALIVE_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const DEAD_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const HOVER_COLOR: Color = Color::rgb(0.45, 0.45, 0.45);

const TIME_STEP: f64 = 1.0 / 20.0;
const N_CELLS_X: usize = 200;
const N_CELLS_Y: usize = 200;

const CELL_SIZE: Vec2 = Vec2::new(4., 4.);
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
            .add_system(add_noise_system)
            .add_stage_after(
                CoreStage::Update,
                FixedUpdateStage,
                SystemStage::parallel()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP).with_label("my_time"))
                    .with_system(update_board_system),
            );
    }
}

fn add_noise_system(
    mut ev_add_noise: EventReader<AddNoiseEvent>,
    mut board: ResMut<Board>,
    mut query: Query<&mut BackgroundColor, With<Cell>>,
) {
    for _ev in ev_add_noise.iter() {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..20);

        for x in 0..board.width {
            for y in 0..board.height {
                let t = die.sample(&mut rng);
                if t == 1 {
                    board.set_old(x, y, true);
                    let e = board.get_entity(x, y);
                    let mut color: Mut<BackgroundColor> = query.get_mut(e).unwrap();
                    *color = ALIVE_COLOR.into();
                }
            }
        }
    }
}
