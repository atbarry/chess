use bevy::prelude::*;
use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH, TILE_SIZE, PIECE_Z_LAYER};
use super::{PieceType, PieceSpawner, Board, Tile, Piece, BoardPos, Side};

impl PieceSpawner {
    fn spawn_piece(&self, commands: &mut Commands, piece_type: PieceType, side: Side, board_pos: BoardPos) -> Option<Piece> {
        let image = match (side, piece_type) {
            (Side::White, PieceType::King) => self.white_king.clone(),
            (Side::White, PieceType::Queen) => self.white_queen.clone(),
            (Side::White, PieceType::Rook) => self.white_rook.clone(),
            (Side::White, PieceType::Bishop) => self.white_bishop.clone(),
            (Side::White, PieceType::Knight) => self.white_knight.clone(),
            (Side::White, PieceType::Pawn) => self.white_pawn.clone(),
            (Side::Black, PieceType::King) => self.black_king.clone(),
            (Side::Black, PieceType::Queen) => self.black_queen.clone(),
            (Side::Black, PieceType::Rook) => self.black_rook.clone(),
            (Side::Black, PieceType::Bishop) => self.black_bishop.clone(),
            (Side::Black, PieceType::Knight) => self.black_knight.clone(),
            (Side::Black, PieceType::Pawn) => self.black_pawn.clone(),
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
            side,
            entity,
            board_pos,
            distance_moved: 0,
            num_moves: 0,
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
            turn: Side::White,
        }
    }

    pub fn change_piece_type_on_move(&mut self, commands: &mut Commands, piece: &mut Piece, new_type: PieceType) {
        piece.piece_type = new_type;
        // get rid of the old entity
        commands.entity(piece.entity).despawn();
        // spawn a new entity with the new type
        *piece = self.spawner.spawn_piece(commands, new_type, piece.side, piece.board_pos).unwrap();
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
        // Spawn pawns
        for x in 0..BOARD_WIDTH{
            self.board[x][1] = self.spawner.spawn_piece(
                commands,
                PieceType::Pawn,
                Side::White,
                BoardPos::new(x, 1).unwrap(),
            );

            self.board[x][6] = self.spawner.spawn_piece(
                commands,
                PieceType::Pawn,
                Side::Black,
                BoardPos::new(x, 6).unwrap(),
            );
        }

        // Spawn rooks
        self.board[0][0] = self.spawner.spawn_piece(
            commands,
            PieceType::Rook,
            Side::White,
            BoardPos::new(0, 0).unwrap(),
        );

        self.board[BOARD_WIDTH - 1][0] = self.spawner.spawn_piece(
            commands,
            PieceType::Rook,
            Side::White,
            BoardPos::new(BOARD_WIDTH - 1, 0).unwrap(),
        );

        self.board[0][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::Rook,
            Side::Black,
            BoardPos::new(0, BOARD_HEIGHT - 1).unwrap(),
        );

        self.board[BOARD_WIDTH - 1][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::Rook,
            Side::Black,
            BoardPos::new(BOARD_WIDTH - 1, BOARD_HEIGHT - 1).unwrap(),
        );

        // Spawn knights
        self.board[1][0] = self.spawner.spawn_piece(
            commands,
            PieceType::Knight,
            Side::White,
            BoardPos::new(1, 0).unwrap(),
        );

        self.board[BOARD_WIDTH - 2][0] = self.spawner.spawn_piece(
            commands,
            PieceType::Knight,
            Side::White,
            BoardPos::new(BOARD_WIDTH - 2, 0).unwrap(),
        );

        self.board[1][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::Knight,
            Side::Black,
            BoardPos::new(1, BOARD_HEIGHT - 1).unwrap(),
        );

        self.board[BOARD_WIDTH - 2][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::Knight,
            Side::Black,
            BoardPos::new(BOARD_WIDTH - 2, BOARD_HEIGHT - 1).unwrap(),
        );

        // Spawn bishops
        self.board[2][0] = self.spawner.spawn_piece(
            commands,
            PieceType::Bishop,
            Side::White,
            BoardPos::new(2, 0).unwrap(),
        );

        self.board[BOARD_WIDTH - 3][0] = self.spawner.spawn_piece(
            commands,
            PieceType::Bishop,
            Side::White,
            BoardPos::new(BOARD_WIDTH - 3, 0).unwrap(),
        );

        self.board[2][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::Bishop,
            Side::Black,
            BoardPos::new(2, BOARD_HEIGHT - 1).unwrap(),
        );

        self.board[BOARD_WIDTH - 3][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::Bishop,
            Side::Black,
            BoardPos::new(BOARD_WIDTH - 3, BOARD_HEIGHT - 1).unwrap(),
        );

        // Spawn queen
        self.board[3][0] = self.spawner.spawn_piece(
            commands,
            PieceType::Queen,
            Side::White,
            BoardPos::new(3, 0).unwrap(),
        );

        self.board[3][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::Queen,
            Side::Black,
            BoardPos::new(3, BOARD_HEIGHT - 1).unwrap(),
        );

        // Spawn king
        self.board[4][0] = self.spawner.spawn_piece(
            commands,
            PieceType::King,
            Side::White,
            BoardPos::new(4, 0).unwrap(),
        );

        self.board[4][BOARD_HEIGHT - 1] = self.spawner.spawn_piece(
            commands,
            PieceType::King,
            Side::Black,
            BoardPos::new(4, BOARD_HEIGHT - 1).unwrap(),
        );
    }
}

