use crate::{constants::{BOARD_HEIGHT, BOARD_WIDTH, TILE_SIZE}, movement::Moveable};
use bevy::prelude::*;

pub mod spawning;
pub mod logic;

pub struct Board{
    board: Vec<Vec<Option<Piece>>>,
    tiles: Vec<Vec<Entity>>,
    spawner: PieceSpawner,
}

#[derive(Clone, Copy)]
pub struct BoardPos{
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub enum PieceType{
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

#[derive(Component, Clone)]
pub struct Piece{
    pub piece_type: PieceType,
    pub entity: Entity,
    pub board_pos: BoardPos,
}

#[derive(Component)]
pub struct Tile{
    pub normal_color: Color,
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
    pub fn move_piece(&mut self, commands: &mut Commands, start: BoardPos, target: BoardPos) {
        let mut piece = self.board[start.x][start.y].take().unwrap();

        // if there is a piece in the destination, remove it
        if let Some(piece) = &self.board[target.x][target.y] {
            commands.entity(piece.entity).despawn();
        }

        commands.entity(piece.entity).insert(Moveable::new(
            start.self_to_world_pos(),
            target.self_to_world_pos(),
        ));

        piece.board_pos = target;
        self.board[target.x][target.y] = Some(piece);
    }   

    pub fn get_tile_entity(&self, board_pos: BoardPos) -> Entity{
        self.tiles[board_pos.x][board_pos.y]
    }

    pub fn get_piece(&self, board_pos: BoardPos) -> Option<Piece>{
        self.board[board_pos.x][board_pos.y].clone()
    }
}

impl BoardPos{
    pub fn new(x: usize, y: usize) -> Self{
        if x >= BOARD_WIDTH || y >= BOARD_HEIGHT{
            panic!("BoardPos out of bounds");
        }

        Self{
            x,
            y,
        }
    }

    pub fn to_world_pos(x: usize, y: usize) -> Vec2{
        Vec2::new((x as f32 + 0.5) * TILE_SIZE, (y as f32 + 0.5) * TILE_SIZE)
    }

    pub fn self_to_world_pos(&self) -> Vec2{
        Self::to_world_pos(self.x, self.y)
    }
    
    pub fn world_to_board(pos: Vec3) -> Option<BoardPos> {
        let x = pos.x / TILE_SIZE;
        let y = pos.y / TILE_SIZE;

        dbg!(x, y);
    
        if x >= 0.0 && y >= 0.0 {
            let (x, y) = (x as usize, y as usize);

            if x < BOARD_WIDTH && y < BOARD_HEIGHT {
                return Some(Self::new(x, y));
            }
        }

        None
    }
}