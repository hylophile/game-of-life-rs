use bevy::prelude::*;

use super::{board::Board, Cell, ALIVE_COLOR, DEAD_COLOR, HOVER_COLOR};

pub fn draw_on_board_system(
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
                board.set_old(cell.pos_x, cell.pos_y, !cell.alive);
                toggle_cell(cell, color);
            }
            Interaction::Hovered => {
                if buttons.pressed(MouseButton::Left) {
                    board.set_old(cell.pos_x, cell.pos_y, !cell.alive);
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
