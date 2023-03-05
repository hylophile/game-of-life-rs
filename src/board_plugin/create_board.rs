use bevy::prelude::*;

use crate::board_plugin::{Cell, DEAD_COLOR};

use super::{board::Board, CELL_SIZE, GAP_BETWEEN_CELLS};

pub fn spawn_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    for y in 0..board.height {
        for x in 0..board.width {
            let cell_position = Vec2::new(
                100.0 + x as f32 * (CELL_SIZE.x + GAP_BETWEEN_CELLS),
                10.0 + y as f32 * (CELL_SIZE.y + GAP_BETWEEN_CELLS),
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
                        pos_x: x,
                        pos_y: y,
                    },
                ))
                .id();
            board.set_entity(x, y, entity);
            // println!("{}", entity);
        }
    }
}
