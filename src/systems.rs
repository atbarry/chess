use crate::{
    board::Board,
    components::{Moveable, Tile},
    constants::{PIECE_Z_LAYER, SELECTED_COLOR},
    resources::MouseInfo,
    resources::{HiglightedSquares, SelectedSquare},
};
use bevy::prelude::*;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_click)
            .add_system(highlight_squares)
            .add_system(move_pieces)
            .add_system(undo)
            .add_system(restart);
    }
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
    // dbg!("Clicked on board at: {}, {}", target_square.x, target_square.y);

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

fn undo(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut selected: ResMut<SelectedSquare>,
    mut highlighted: ResMut<HiglightedSquares>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Z) && input.pressed(KeyCode::LControl) {
        board.undo_last_change(&mut commands);

        selected.piece = None;
        selected.tile = None;
        selected.changed = true;
        highlighted.squares.clear();
    }
}

fn restart(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut selected: ResMut<SelectedSquare>,
    mut highlighted: ResMut<HiglightedSquares>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::R) && input.pressed(KeyCode::LControl) {
        board.restart_game(&mut commands);

        selected.piece = None;
        selected.tile = None;
        selected.changed = true;
        highlighted.squares.clear();
    }
}

fn move_pieces(
    mut commands: Commands,
    mut q_moveable: Query<(Entity, &mut Moveable, &mut Transform)>,
    time: Res<Time>,
) {
    fn smoothstep(x: f32) -> f32 {
        (x * std::f32::consts::PI / 2.0).sin().powi(3)
    }

    for (entity, mut moveable, mut transform) in q_moveable.iter_mut() {
        if moveable.timer.tick(time.delta()).just_finished() {
            transform.translation = moveable.target_pos.extend(PIECE_Z_LAYER);
            commands.entity(entity).remove::<Moveable>();
        } else {
            transform.translation = moveable
                .start_pos
                .lerp(moveable.target_pos, smoothstep(moveable.timer.percent()))
                .extend(PIECE_Z_LAYER);
        }
    }
}
