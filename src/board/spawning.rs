use bevy::prelude::*;
use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH, TILE_SIZE, PIECE_Z_LAYER};
use super::{PieceType, PieceSpawner, Board, Tile, Piece, BoardPos};

impl PieceSpawner {
    fn spawn_piece(&self, commands: &mut Commands, piece_type: PieceType, board_pos: BoardPos) -> Option<Piece> {
        let image = match piece_type {
            PieceType::WhiteKing => self.white_king.clone(),
            PieceType::WhiteQueen => self.white_queen.clone(),
            PieceType::WhiteRook => self.white_rook.clone(),
            PieceType::WhiteBishop => self.white_bishop.clone(),
            PieceType::WhiteKnight => self.white_knight.clone(),
            PieceType::WhitePawn => self.white_pawn.clone(),
            PieceType::BlackKing => self.black_king.clone(),
            PieceType::BlackQueen => self.black_queen.clone(),
            PieceType::BlackRook => self.black_rook.clone(),
            PieceType::BlackBishop => self.black_bishop.clone(),
            PieceType::BlackKnight => self.black_knight.clone(),
            PieceType::BlackPawn => self.black_pawn.clone(),
        };

        let world_pos = BoardPos::to_world_pos(board_pos.x, board_pos.y);

        let entity = commands.spawn_bundle(SpriteBundle {
            texture: image,
            transform: Transform::from_xyz(world_pos.x, world_pos.y, PIECE_Z_LAYER),
            // sprite: Sprite {
            //     color: Color::ORANGE,
            //     ..Default::default()
            // },
            ..Default::default()
        }).id();

        let piece = Piece{
            piece_type,
            entity,
            board_pos,
        };

        // commands.entity(entity).insert(piece.clone());

        Some(piece)
    }
}
impl Board{
    pub fn new(spawner: PieceSpawner) -> Self{
        Self{
            board: vec![vec![None; BOARD_WIDTH]; BOARD_HEIGHT],
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

                let pos = BoardPos::to_world_pos(x, y);

                let tile = commands.spawn_bundle(SpriteBundle{
                    // texture,
                    transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                    sprite: Sprite{
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        color,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Tile{
                    normal_color: color,
                })
                .id();
                row.push(tile);
            }
            self.tiles.push(row);
        }
    }


    pub fn spawn_pieces(&mut self, commands: &mut Commands) {
        // Spawn white pawns
        for x in 0..BOARD_WIDTH{
            self.board[x][1] = self.spawner.spawn_piece(
                commands,
                PieceType::WhitePawn,
                BoardPos::new(x, 1),
            );
        }

        // Spawn white rooks
        self.board[0][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteRook,
            BoardPos::new(0, 0),
        );

        self.board[BOARD_WIDTH - 1][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteRook,
            BoardPos::new(BOARD_WIDTH - 1, 0),
        );

        // Spawn white knights
        self.board[1][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteKnight,
            BoardPos::new(1, 0),
        );

        self.board[BOARD_WIDTH - 2][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteKnight,
            BoardPos::new(BOARD_WIDTH - 2, 0),
        );

        // Spawn white bishops
        self.board[2][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteBishop,
            BoardPos::new(2, 0),
        );

        self.board[BOARD_WIDTH - 3][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteBishop,
            BoardPos::new(BOARD_WIDTH - 3, 0),
        );

        // Spawn white queen
        self.board[3][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteQueen,
            BoardPos::new(3, 0),
        );

        // Spawn white king
        self.board[4][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteKing,
            BoardPos::new(4, 0),
        );

        // Spawn black pawns
        for x in 0..BOARD_WIDTH{
            self.board[x][BOARD_HEIGHT - 2] = self.spawner.spawn_piece(
                commands,
                PieceType::BlackPawn,
                BoardPos::new(x, BOARD_HEIGHT - 2),
            );
        }

        // Spawn black rooks
        self.board[0][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackRook,
            BoardPos::new(0, BOARD_HEIGHT - 1),
        );

        self.board[BOARD_WIDTH - 1][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackRook,
            BoardPos::new(BOARD_WIDTH - 1, BOARD_HEIGHT - 1),
        );

        // Spawn black knights
        self.board[1][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackKnight,
            BoardPos::new(1, BOARD_HEIGHT - 1),
        );

        self.board[BOARD_WIDTH - 2][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackKnight,
            BoardPos::new(BOARD_WIDTH - 2, BOARD_HEIGHT - 1),
        );

        // Spawn black bishops
        self.board[2][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackBishop,
            BoardPos::new(2, BOARD_HEIGHT - 1),
        );

        self.board[BOARD_WIDTH - 3][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackBishop,
            BoardPos::new(BOARD_WIDTH - 3, BOARD_HEIGHT - 1),
        );

        // Spawn black queen
        self.board[3][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackQueen,
            BoardPos::new(3, BOARD_HEIGHT - 1),
        );

        // Spawn black king
        self.board[4][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackKing,
            BoardPos::new(4, BOARD_HEIGHT - 1),
        );
    }
}

