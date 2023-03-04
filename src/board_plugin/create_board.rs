use bevy::prelude::*;

use crate::board_plugin::{Cell, DEAD_COLOR, N_CELLS_X, N_CELLS_Y};

use super::board::Board;

const CELL_SIZE: Vec2 = Vec2::new(10., 10.);
const GAP_BETWEEN_CELLS: f32 = 1.0;

pub fn spawn_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
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
