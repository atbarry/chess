use bevy::prelude::*;
use crate::PieceSpawner;
use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH, TILE_SIZE};

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

impl Board{
    pub fn new(spawner: PieceSpawner) -> Self{
        Self{
            board: [[None; BOARD_HEIGHT]; BOARD_WIDTH],
            tiles: Vec::new(),
            spawner,
        }
    }
    
    pub fn spawn_tiles(&mut self, commands: &mut Commands){
        for x in 0..BOARD_WIDTH{
            let mut row = Vec::new();
            for y in 0..BOARD_HEIGHT{
                let color =  if (x + y) % 2 == 0{
                    // self.spawner.white_tile.clone()
                    Color::rgb(0.9, 0.9, 0.9)
                }else{
                    // self.spawner.black_tile.clone()
                    Color::rgb(0.1, 0.1, 0.1)
                };

                let pos = Board::to_world_pos(x, y);

                let tile = commands.spawn_bundle(SpriteBundle{
                    // texture,
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                    sprite: Sprite{
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        color,
                        ..Default::default()
                    },
                    ..Default::default()
                }).id();
                row.push(tile);
            }
            self.tiles.push(row);
        }
    }

    pub fn to_world_pos(x: usize, y: usize) -> Vec2{
        Vec2::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE)
    }

    pub fn world_to_board(pos: Vec3) -> Option<(usize, usize)> {
        let x = (pos.x / TILE_SIZE) as usize;
        let y = (pos.y / TILE_SIZE) as usize;

        if x < BOARD_WIDTH && y < BOARD_HEIGHT{
            Some((x, y))
        }else{
            None
        }
    }

    pub fn spawn_pieces(&mut self, commands: &mut Commands) {
        // Spawn white pawns
        for x in 0..BOARD_WIDTH{
            self.board[x][1] = self.spawner.spawn_piece(
                commands,
                Piece::WhitePawn,
                Board::to_world_pos(x, 1),
            );
        }

        // Spawn white rooks
        self.board[0][0] = self.spawner.spawn_piece(
            commands,
            Piece::WhiteRook,
            Board::to_world_pos(0, 0),
        );

        self.board[BOARD_WIDTH - 1][0] = self.spawner.spawn_piece(
            commands,
            Piece::WhiteRook,
            Board::to_world_pos(BOARD_WIDTH - 1, 0),
        );

        // Spawn white knights
        self.board[1][0] = self.spawner.spawn_piece(
            commands,
            Piece::WhiteKnight,
            Board::to_world_pos(1, 0),
        );

        self.board[BOARD_WIDTH - 2][0] = self.spawner.spawn_piece(
            commands,
            Piece::WhiteKnight,
            Board::to_world_pos(BOARD_WIDTH - 2, 0),
        );

        // Spawn white bishops
        self.board[2][0] = self.spawner.spawn_piece(
            commands,
            Piece::WhiteBishop,
            Board::to_world_pos(2, 0),
        );

        self.board[BOARD_WIDTH - 3][0] = self.spawner.spawn_piece(
            commands,
            Piece::WhiteBishop,
            Board::to_world_pos(BOARD_WIDTH - 3, 0),
        );

        // Spawn white queen
        self.board[3][0] = self.spawner.spawn_piece(
            commands,
            Piece::WhiteQueen,
            Board::to_world_pos(3, 0),
        );

        // Spawn white king
        self.board[4][0] = self.spawner.spawn_piece(
            commands,
            Piece::WhiteKing,
            Board::to_world_pos(4, 0),
        );

        // Spawn black pawns
        for x in 0..BOARD_WIDTH{
            self.board[x][BOARD_HEIGHT - 2] = self.spawner.spawn_piece(
                commands,
                Piece::BlackPawn,
                Board::to_world_pos(x, BOARD_HEIGHT - 2),
            );
        }

        // Spawn black rooks
        self.board[0][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            Piece::BlackRook,
            Board::to_world_pos(0, BOARD_HEIGHT - 1),
        );

        self.board[BOARD_WIDTH - 1][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            Piece::BlackRook,
            Board::to_world_pos(BOARD_WIDTH - 1, BOARD_HEIGHT - 1),
        );

        // Spawn black knights
        self.board[1][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            Piece::BlackKnight,
            Board::to_world_pos(1, BOARD_HEIGHT - 1),
        );

        self.board[BOARD_WIDTH - 2][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            Piece::BlackKnight,
            Board::to_world_pos(BOARD_WIDTH - 2, BOARD_HEIGHT - 1),
        );

        // Spawn black bishops
        self.board[2][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            Piece::BlackBishop,
            Board::to_world_pos(2, BOARD_HEIGHT - 1),
        );

        self.board[BOARD_WIDTH - 3][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            Piece::BlackBishop,
            Board::to_world_pos(BOARD_WIDTH - 3, BOARD_HEIGHT - 1),
        );

        // Spawn black queen
        self.board[3][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            Piece::BlackQueen,
            Board::to_world_pos(3, BOARD_HEIGHT - 1),
        );

        // Spawn black king
        self.board[4][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            Piece::BlackKing,
            Board::to_world_pos(4, BOARD_HEIGHT - 1),
        );
    }
}

impl PieceSpawner {
    fn spawn_piece(&self, commands: &mut Commands, piece: Piece, world_pos: Vec2) -> Option<Entity> {
        let image = match piece {
            Piece::WhiteKing => self.white_king.clone(),
            Piece::WhiteQueen => self.white_queen.clone(),
            Piece::WhiteRook => self.white_rook.clone(),
            Piece::WhiteBishop => self.white_bishop.clone(),
            Piece::WhiteKnight => self.white_knight.clone(),
            Piece::WhitePawn => self.white_pawn.clone(),
            Piece::BlackKing => self.black_king.clone(),
            Piece::BlackQueen => self.black_queen.clone(),
            Piece::BlackRook => self.black_rook.clone(),
            Piece::BlackBishop => self.black_bishop.clone(),
            Piece::BlackKnight => self.black_knight.clone(),
            Piece::BlackPawn => self.black_pawn.clone(),
        };

        let a = commands.spawn_bundle(SpriteBundle {
            texture: image,
            transform: Transform::from_xyz(world_pos.x, world_pos.y, 10.0),
            ..Default::default()
        });

        Some(a.id())
    }
}