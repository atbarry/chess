use bevy::{prelude::*, render::texture::ImageSettings};
use board::Board;
use constants::*;

mod input;
mod board;
mod constants;

pub struct StarterPlugin;

impl Plugin for StarterPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ImageSettings::default_nearest())
            .add_startup_system(create_board)
            .add_startup_system(spawn_camera);

    }
}

fn spawn_camera(mut commands: Commands) {
    // Spawn a camera
    commands.spawn_bundle(Camera2dBundle{
        transform: Transform::from_xyz(
            BOARD_WIDTH as f32 * TILE_SIZE / 2.0,
            BOARD_HEIGHT as f32 * TILE_SIZE / 2.0,
            999.0,
        ),

        projection: OrthographicProjection {
            far: 1000.0,
            left: 0.0 - TILE_SIZE * 2.0,
            right: BOARD_WIDTH as f32 * TILE_SIZE + TILE_SIZE * 2.0,
            bottom: 0.0 - TILE_SIZE * 2.0,
            top: BOARD_HEIGHT as f32 * TILE_SIZE + TILE_SIZE * 2.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn create_board(mut commands: Commands, server: Res<AssetServer>) {
    // Load the sprites
    let spawner = PieceSpawner{
        white_king: server.load("pieces/white_king.png"),
        white_queen: server.load("pieces/white_queen.png"),
        white_rook: server.load("pieces/white_rook.png"),
        white_bishop: server.load("pieces/white_bishop.png"),
        white_knight: server.load("pieces/white_knight.png"),
        white_pawn: server.load("pieces/white_pawn.png"),
        black_king: server.load("pieces/black_king.png"),
        black_queen: server.load("pieces/black_queen.png"),
        black_rook: server.load("pieces/black_rook.png"),
        black_bishop: server.load("pieces/black_bishop.png"),
        black_knight: server.load("pieces/black_knight.png"),
        black_pawn: server.load("pieces/black_pawn.png"),
    };

    let mut board = Board::new(spawner);
    board.spawn_tiles(&mut commands);
    board.spawn_pieces(&mut commands);

    commands.insert_resource(board);
}

pub struct PieceSpawner {
    white_king: Handle<Image>,
    white_queen: Handle<Image>,
    white_rook: Handle<Image>,
    white_bishop: Handle<Image>,
    white_knight: Handle<Image>,
    white_pawn: Handle<Image>,
    black_king: Handle<Image>,
    black_queen: Handle<Image>,
    black_rook: Handle<Image>,
    black_bishop: Handle<Image>,
    black_knight: Handle<Image>,
    black_pawn: Handle<Image>,
}


    

