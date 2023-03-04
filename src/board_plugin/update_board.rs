use bevy::prelude::*;

use crate::menu_plugin::Config;

use super::{board::Board, Cell, ALIVE_COLOR, DEAD_COLOR};

pub fn update_board_system(
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
