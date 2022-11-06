use bevy::prelude::*;

use crate::{board::{BoardPos, Piece, Board, BChange}, constants::{DESTROY_COLOR, MOVE_COLOR, SWAP_COLOR, PROMOTE_COLOR, PUSH_PREMOTE_COLOR}};

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
            let highlight_info = match change {
                BChange::Move { end, .. } => (board.get_tile_entity(end), MOVE_COLOR),
                BChange::MoveDestroy { end, ..} => 
                    (board.get_tile_entity(end), DESTROY_COLOR),
                BChange::BothMove { start2, ..} => 
                    (board.get_tile_entity(start2), SWAP_COLOR),
                BChange::Promotion { end, ..} => 
                    (board.get_tile_entity(end), PROMOTE_COLOR),
                BChange::PushPremote { start2, ..} => 
                    (board.get_tile_entity(start2), PUSH_PREMOTE_COLOR),
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

