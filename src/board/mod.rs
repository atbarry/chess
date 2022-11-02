use crate::{constants::{BOARD_HEIGHT, BOARD_WIDTH, TILE_SIZE}, movement::Moveable};
use bevy::prelude::*;

pub mod spawning;
pub mod logic;

pub struct Board{
    board: Vec<Vec<Option<Piece>>>,
    tiles: Vec<Vec<Entity>>,
    spawner: PieceSpawner,
    turn: Side,
    turn_num: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BoardPos{
    pub x: usize,
    pub y: usize,
}

pub enum BChange{
    Move {
        start: BoardPos,
        end: BoardPos,
    },
    MoveDestroy {
        start: BoardPos,
        end: BoardPos,
        target: BoardPos,
    },
    Swap {
        start1: BoardPos,
        start2: BoardPos,
        end1: BoardPos,
        end2: BoardPos,
    },
    Promotion {
        start: BoardPos,
        end: BoardPos,
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceType{
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug)]
pub enum Side{
    White,
    Black,
}

#[derive(Component, Clone, Debug)]
pub struct Piece{
    pub piece_type: PieceType,
    pub side: Side,
    pub entity: Entity,
    pub board_pos: BoardPos,
    pub distance_moved: u32,
    pub num_moves: u32,
    pub turn_last_moved: u32,
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
        };

        self.turn_num += 1;
    }

    pub fn is_turn(&self, side: Side) -> bool {
        side.is_friendly(&self.turn)
    }

    pub fn apply_board_change(&mut self, commands: &mut Commands, board_change: BChange) {

        match board_change {
            BChange::Move { start, end } => {
                self.move_piece(commands, start, end);
            },
            BChange::MoveDestroy { start, end, target } => {
                let enemy = self.take(target).unwrap().entity;
                commands.entity(enemy).despawn();
                self.move_piece(commands, start, end);
            },
            BChange::Swap { start1, start2, end1, end2 } => {
                self.swap_piece(commands, start1, start2, end1, end2);
            },
            BChange::Promotion { start, end } => {
                let mut piece = self.get_piece(start).unwrap();
                self.promote_piece(commands, &mut piece, PieceType::Queen);
                self.set_piece(Some(piece), start);

                if let Some(enemy) = self.take(end) {
                    commands.entity(enemy.entity).despawn();
                }

                self.move_piece(commands, start, end);
            },
        }

        // finally change the turn
        self.change_turn();
    }   

    fn move_piece(&mut self, commands: &mut Commands, start: BoardPos, end: BoardPos) {
        //* Important gets the piece and leaves a none value in its place
        let mut piece = self.board[start.x][start.y].take().unwrap();

        commands.entity(piece.entity).insert(Moveable::new(
            start.self_to_world_pos(),
            end.self_to_world_pos(),
        ));

        piece.move_piece(end, self.turn_num);
        self.set_piece(Some(piece), end);
    }

    fn swap_piece(&mut self, commands: &mut Commands, start1: BoardPos, start2: BoardPos, end1: BoardPos, end2: BoardPos) {
        let mut piece1= self.take(start1).unwrap();
        let mut piece2 = self.take(start2).unwrap();

        commands.entity(piece1.entity).insert(Moveable::new(
            start1.self_to_world_pos(),
            end1.self_to_world_pos(),
        ));

        commands.entity(piece2.entity).insert(Moveable::new(
            start2.self_to_world_pos(),
            end2.self_to_world_pos(),
        ));

        piece1.move_piece(end1, self.turn_num);
        piece2.move_piece(end2, self.turn_num);

        // dbg!(&piece1);
        // dbg!(&piece2);

        self.set_piece(Some(piece1), end1);
        self.set_piece(Some(piece2), end2);
    }

    pub fn get_tile_entity(&self, board_pos: BoardPos) -> Entity{
        self.tiles[board_pos.x][board_pos.y]
    }

    pub fn get_piece(&self, board_pos: BoardPos) -> Option<Piece>{
        self.board[board_pos.x][board_pos.y].clone()
    }

    pub fn set_piece(&mut self, piece: Option<Piece>, board_pos: BoardPos) {
        self.board[board_pos.x][board_pos.y] = piece;
    }

    pub fn is_empty(&self, board_pos: BoardPos) -> bool {
        self.board[board_pos.x][board_pos.y].is_none()
    }

    pub fn is_occupied(&self, board_pos: BoardPos) -> bool {
        self.board[board_pos.x][board_pos.y].is_some()
    }

    pub fn take(&mut self, board_pos: BoardPos) -> Option<Piece> {
        self.board[board_pos.x][board_pos.y].take()
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

impl BChange {
    pub fn click_pos_to_activate_change(&self) -> BoardPos {
        #[allow(unused)]
        match self {
            BChange::Move { start, end } => *end,
            BChange::MoveDestroy { start, end, target } => *end,
            BChange::Swap { start1, start2, end1, end2 } => *start2,
            BChange::Promotion { start, end } => *end,
        }
    }

    pub fn convert_to_promotion(&self) -> BChange {
        #[allow(unused)]
        match self {
            BChange::Move { start, end } => BChange::Promotion { start: *start, end: *end },
            BChange::MoveDestroy { start, end, target } => BChange::Promotion { start: *start, end: *end },
            BChange::Swap { start1, start2, end1, end2 } => BChange::Promotion { start: *start1, end: *end1 },
            BChange::Promotion { start, end } => BChange::Promotion { start: *start, end: *end },
        }
    }
}

impl Piece{
    pub fn move_piece(&mut self, target: BoardPos, turn_num: u32) {
        self.distance_moved += {
            let x = self.board_pos.x as isize - target.x as isize;
            let y = self.board_pos.y as isize - target.y as isize;
            (x.abs() + y.abs()) as u32
        };

        self.board_pos = target;
        self.num_moves += 1;
        self.turn_last_moved = turn_num;
    }

    pub fn turns_since_last_move(&self, turn_num: u32) -> u32 {
        turn_num - self.turn_last_moved
    }
}