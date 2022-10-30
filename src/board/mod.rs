use crate::{constants::{BOARD_HEIGHT, BOARD_WIDTH, TILE_SIZE}, movement::Moveable};
use bevy::prelude::*;

pub mod spawning;
pub mod logic;

pub struct Board{
    board: Vec<Vec<Option<Piece>>>,
    tiles: Vec<Vec<Entity>>,
    spawner: PieceSpawner,
    turn: Side,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoardPos{
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy)]
pub enum PieceType{
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy)]
pub enum Side{
    White,
    Black,
}

#[derive(Component, Clone)]
pub struct Piece{
    pub piece_type: PieceType,
    pub side: Side,
    pub entity: Entity,
    pub board_pos: BoardPos,
    pub distance_moved: usize,
    pub num_moves: usize,
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
    fn change_turn(&mut self) {
        self.turn = match self.turn {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }

    pub fn is_turn(&self, side: Side) -> bool {
        side.is_friendly(&self.turn)
    }

    pub fn move_piece(&mut self, commands: &mut Commands, start: BoardPos, target: BoardPos) {
        let mut piece = self.board[start.x][start.y].take().unwrap();

        // if there is a piece in the destination, remove it
        if let Some(piece) = &self.board[target.x][target.y] {
            commands.entity(piece.entity).despawn();
        }

        // if the piece is a pawn and it is moving to the last row, promote it
        match (piece.piece_type, target.y) {
            (PieceType::Pawn, 0) => {
                self.change_piece_type_on_move(commands, &mut piece, PieceType::Queen);
            }
            (PieceType::Pawn, 7) => {
                self.change_piece_type_on_move(commands, &mut piece, PieceType::Queen);
            }
            _ => {}
        }

        commands.entity(piece.entity).insert(Moveable::new(
            start.self_to_world_pos(),
            target.self_to_world_pos(),
        ));

        piece.move_piece(target);
        self.board[target.x][target.y] = Some(piece);

        // finally change the turn
        self.change_turn();
    }   

    pub fn get_tile_entity(&self, board_pos: BoardPos) -> Entity{
        self.tiles[board_pos.x][board_pos.y]
    }

    pub fn get_piece(&self, board_pos: BoardPos) -> Option<Piece>{
        self.board[board_pos.x][board_pos.y].clone()
    }
}

impl BoardPos{
    pub fn new(x: usize, y: usize) -> Result<Self, String> {
        if x >= BOARD_WIDTH || y >= BOARD_HEIGHT{
            return Err(format!("BoardPos out of bounds: x: {}, y: {}", x, y));
        }

        Ok(Self { x, y})
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
                return Some(Self::new(x, y).unwrap());
            }
        }

        None
    }
}

impl Side {
    pub fn is_friendly(&self, other: &Self) -> bool {
        match (self, other) {
            (Side::White, Side::White) => true,
            (Side::Black, Side::Black) => true,
            _ => false,
        }
    }

    pub fn is_enemy(&self, other: &Self) -> bool {
        !self.is_friendly(other)
    }
}

impl Piece{
    pub fn move_piece(&mut self, target: BoardPos) {
        self.distance_moved += {
            let x = self.board_pos.x as isize - target.x as isize;
            let y = self.board_pos.y as isize - target.y as isize;
            (x.abs() + y.abs()) as usize
        };

        self.board_pos = target;
        self.num_moves += 1;
    }
}