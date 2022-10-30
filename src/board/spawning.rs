use bevy::prelude::*;
use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH, TILE_SIZE};
use super::{PieceType, PieceSpawner, Board, Tile, Piece};

impl PieceSpawner {
    fn spawn_piece(&self, commands: &mut Commands, piece: PieceType, world_pos: Vec2) -> Option<Entity> {
        let image = match piece {
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

        let entity_id = commands.spawn_bundle(SpriteBundle {
            texture: image,
            transform: Transform::from_xyz(world_pos.x, world_pos.y, 10.0),
            // sprite: Sprite {
            //     color: Color::ORANGE,
            //     ..Default::default()
            // },
            ..Default::default()
        }).insert(Piece {
            start_pos: world_pos,
            target_pos: None,
            ptype: piece,
        }).id();

        Some(entity_id)
    }
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
                Board::to_world_pos(x, 1),
            );
        }

        // Spawn white rooks
        self.board[0][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteRook,
            Board::to_world_pos(0, 0),
        );

        self.board[BOARD_WIDTH - 1][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteRook,
            Board::to_world_pos(BOARD_WIDTH - 1, 0),
        );

        // Spawn white knights
        self.board[1][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteKnight,
            Board::to_world_pos(1, 0),
        );

        self.board[BOARD_WIDTH - 2][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteKnight,
            Board::to_world_pos(BOARD_WIDTH - 2, 0),
        );

        // Spawn white bishops
        self.board[2][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteBishop,
            Board::to_world_pos(2, 0),
        );

        self.board[BOARD_WIDTH - 3][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteBishop,
            Board::to_world_pos(BOARD_WIDTH - 3, 0),
        );

        // Spawn white queen
        self.board[3][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteQueen,
            Board::to_world_pos(3, 0),
        );

        // Spawn white king
        self.board[4][0] = self.spawner.spawn_piece(
            commands,
            PieceType::WhiteKing,
            Board::to_world_pos(4, 0),
        );

        // Spawn black pawns
        for x in 0..BOARD_WIDTH{
            self.board[x][BOARD_HEIGHT - 2] = self.spawner.spawn_piece(
                commands,
                PieceType::BlackPawn,
                Board::to_world_pos(x, BOARD_HEIGHT - 2),
            );
        }

        // Spawn black rooks
        self.board[0][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackRook,
            Board::to_world_pos(0, BOARD_HEIGHT - 1),
        );

        self.board[BOARD_WIDTH - 1][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackRook,
            Board::to_world_pos(BOARD_WIDTH - 1, BOARD_HEIGHT - 1),
        );

        // Spawn black knights
        self.board[1][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackKnight,
            Board::to_world_pos(1, BOARD_HEIGHT - 1),
        );

        self.board[BOARD_WIDTH - 2][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackKnight,
            Board::to_world_pos(BOARD_WIDTH - 2, BOARD_HEIGHT - 1),
        );

        // Spawn black bishops
        self.board[2][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackBishop,
            Board::to_world_pos(2, BOARD_HEIGHT - 1),
        );

        self.board[BOARD_WIDTH - 3][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackBishop,
            Board::to_world_pos(BOARD_WIDTH - 3, BOARD_HEIGHT - 1),
        );

        // Spawn black queen
        self.board[3][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackQueen,
            Board::to_world_pos(3, BOARD_HEIGHT - 1),
        );

        // Spawn black king
        self.board[4][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::BlackKing,
            Board::to_world_pos(4, BOARD_HEIGHT - 1),
        );
    }
}

