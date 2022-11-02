use bevy::prelude::*;
use crate::{board::{Board, Tile, Piece, BChange}, constants::{SELECTED_COLOR, HIGHLIGHT_COLOR, DESTROY_COLOR, SWAP_COLOR, PROMOTE_COLOR}, input::MouseInfo};

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedSquare::default())
            .insert_resource(HiglightedSquares{ squares: Vec::new() })
            .add_system(on_click)
            .add_system(highlight_squares);
        
    }
}

pub struct SelectedSquare{
    changed: bool,
    tile: Option<Entity>,
    piece: Option<Piece>,
}

pub struct HiglightedSquares{
    squares: Vec<(Entity, Color)>,
}

pub fn on_click(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut selected: ResMut<SelectedSquare>,
    mut highlighted: ResMut<HiglightedSquares>,
    mouse: Res<MouseInfo>,
) {
    // if there was no click, don't do anything
    if !mouse.just_clicked {
        return;
    }

    selected.changed = true;
    let target_square = mouse.board_pos;
    
    // if there is no square under the mouse, deselect and exit
    if target_square.is_none() {
        selected.piece = None;
        selected.tile = None;
        return;
    }

    let target_square = target_square.expect("Should not fail if it makes it here");
    println!("Clicked on board at: {}, {}", target_square.x, target_square.y);

    if let Some(piece) = &selected.piece {
        if let Some(change) = board.check_valid_change(piece.board_pos, target_square) {
            println!("Valid move");
            board.apply_board_change(&mut commands, change);
            selected.piece = None;
            selected.tile = None;
            return;
        }
    }

    selected.piece = board.get_piece(target_square);  
    selected.tile = Some(board.get_tile_entity(target_square));  
    
    if let Some(piece) = &selected.piece {
        let moves = board.get_possible_moves(piece.board_pos);
        *highlighted = HiglightedSquares::from_board_changes(&board, moves);
    }
}

pub fn highlight_squares(
    mut selected: ResMut<SelectedSquare>,
    mut highlighted: ResMut<HiglightedSquares>,
    mut q_tile: Query<(Entity, &mut Sprite, &Tile)>,
) {
    if !selected.changed {
        return;
    }

    let entity_in_highlighted = |entity: Entity| -> Option<Color> {
        for (e, h) in highlighted.squares.iter() {
            if *e == entity {
                return Some(*h);
            }
        }
        
        None
    };

    for (entity, mut sprite, tile) in q_tile.iter_mut() {
        if Some(entity) == selected.tile {
            sprite.color = SELECTED_COLOR;
        } else if let Some(color) = entity_in_highlighted(entity) {
            sprite.color = color;
        } else {
            sprite.color = tile.normal_color;
        }
    }

    // clear the highlighted squares
    highlighted.squares.clear();
    selected.changed = false;
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

                BChange::Swap { start1, start2, end1, end2} => 
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