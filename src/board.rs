pub mod spawning;
use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH, TILE_SIZE};
use bevy::prelude::*;

pub struct Board{
    board: [[Option<Entity>; BOARD_HEIGHT]; BOARD_WIDTH],
    tiles: Vec<Vec<Entity>>,
    spawner: PieceSpawner,
}

pub enum Piece{
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,
    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
}
pub struct PieceSpawner {
    pub white_king: Handle<Image>,
    pub white_queen: Handle<Image>,
    pub white_rook: Handle<Image>,
    pub white_bishop: Handle<Image>,
    pub white_knight: Handle<Image>,
    pub white_pawn: Handle<Image>,
    pub black_king: Handle<Image>,
    pub black_queen: Handle<Image>,
    pub black_rook: Handle<Image>,
    pub black_bishop: Handle<Image>,
    pub black_knight: Handle<Image>,
    pub black_pawn: Handle<Image>,
}

impl Board{
    pub fn to_world_pos(x: usize, y: usize) -> Vec2{
        Vec2::new((x as f32 + 0.5) * TILE_SIZE, (y as f32 + 0.5) * TILE_SIZE)
    }
    
    pub fn world_to_board(pos: Vec3) -> Option<(usize, usize)> {
        let x = pos.x / TILE_SIZE;
        let y = pos.y / TILE_SIZE;

        dbg!(x, y);
    
        if x >= 0.0 && y >= 0.0 {
            let (x, y) = (x as usize, y as usize);

            if x < BOARD_WIDTH && y < BOARD_HEIGHT {
                return Some((x, y));
            }
        }

        None
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Entity{
        self.tiles[x][y]
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<Entity>{
        self.board[x][y]
    }
}