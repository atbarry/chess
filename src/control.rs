use bevy::prelude::*;
use crate::{board::{Board, Tile, Piece, BoardPos}, constants::{SELECTED_COLOR, HIGHLIGHT_COLOR}, input::MouseInfo};

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
    
    // if there is no square under the mouse, don't select anything
    if target_square.is_none() {
        selected.piece = None;
        selected.tile = None;
        return;
    }

    let target_square = target_square.expect("Should not fail if it makes it here");
    println!("Clicked on board at: {}, {}", target_square.x, target_square.y);

    if let Some(piece) = &selected.piece {
        if board.is_valid_move(piece.board_pos, target_square) {
            println!("Valid move");
            board.move_piece(&mut commands, piece.board_pos, target_square);
            selected.piece = None;
            selected.tile = None;
            return;
        }
    }

    selected.piece = board.get_piece(target_square);  
    selected.tile = Some(board.get_tile_entity(target_square));  
    
    if let Some(piece) = &selected.piece {
        let moves = board.get_valid_moves(piece.board_pos);
        *highlighted = HiglightedSquares::from_board_positions(&board, moves);
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
    pub fn from_board_positions(board: &Board, positions: Vec<BoardPos>) -> Self{
        let mut squares = Vec::new();
        for pos in positions{
            squares.push((board.get_tile_entity(pos), HIGHLIGHT_COLOR));
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