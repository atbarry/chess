use bevy::prelude::*;

use crate::{board::{BoardPos, Piece, Board, BChange}, constants::{DESTROY_COLOR, HIGHLIGHT_COLOR, SWAP_COLOR, PROMOTE_COLOR}};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SelectedSquare::default())
            .insert_resource(HiglightedSquares{ squares: Vec::new() })
            .insert_resource(MouseInfo{
                world_cords: None,
                board_pos: None,
                just_clicked: false,
            });
    }
}

pub struct MouseInfo {
    pub world_cords: Option<Vec3>,
    pub board_pos: Option<BoardPos>,
    pub just_clicked: bool, 
}


pub struct SelectedSquare{
    pub changed: bool,
    pub tile: Option<Entity>,
    pub piece: Option<Piece>,
}

pub struct HiglightedSquares{
    pub squares: Vec<(Entity, Color)>,
}


impl HiglightedSquares {
    pub fn from_board_changes(board: &Board, board_changes: Vec<BChange>) -> Self{
        let mut squares = Vec::new();
        for change in board_changes{
            #[allow(unused)]
            let highlight_info = match change {
                BChange::Move { start, end } => (board.get_tile_entity(end), HIGHLIGHT_COLOR),
                BChange::MoveDestroy { start, end, target, } => 
                    (board.get_tile_entity(end), DESTROY_COLOR),

                BChange::BothMove { start1, start2, end1, end2} => 
                    (board.get_tile_entity(start2), SWAP_COLOR),
                BChange::Promotion { start, end } => 
                    (board.get_tile_entity(end), PROMOTE_COLOR),
            };

            squares.push(highlight_info);
        }

        Self{
            squares,
        }
    }
}


impl Default for SelectedSquare {
    fn default() -> Self {
        Self{
            changed: false,
            tile: None,
            piece: None,
        }
    }
}

