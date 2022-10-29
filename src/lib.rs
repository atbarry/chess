use bevy::{prelude::*, render::{texture::{ImageSettings}, camera::ScalingMode}};
use bevy_tiled_camera::*;
use board::{Board, PieceSpawner}; 
use input::board_click_index;
use constants::*;

mod input;
mod board;
mod constants;

pub struct StarterPlugin;

impl Plugin for StarterPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ImageSettings::default_nearest())
            .add_plugin(TiledCameraPlugin)
            .add_startup_system(create_board)
            .add_startup_system(spawn_camera)
            .add_system(on_board_click);

    }
}

fn on_board_click(
    commands: Commands,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<Input<MouseButton>>,
    board: Res<Board>,
) {
    if let Some((x, y)) = board_click_index(wnds, q_camera, mouse) {
        println!("Clicked on board at: {}, {}", x, y);
        let clicked_tile = board.get_tile(x, y);

        
    }
}

fn spawn_camera(mut commands: Commands) {
    let size = TILE_SIZE * (BOARD_WIDTH + 2) as f32;

    let camera_bundle = Camera2dBundle{
        transform: Transform::from_xyz(
            BOARD_WIDTH as f32 * TILE_SIZE / 2.0,
            BOARD_HEIGHT as f32 * TILE_SIZE / 2.0,
            999.0,
        ),

        projection: OrthographicProjection {
            far: 1000.0,
            scaling_mode: ScalingMode::Auto { min_width: size, min_height: size},
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn_bundle(camera_bundle);

    // let tiled_camera_bun = TiledCameraBundle::unit_cam([BOARD_WIDTH as u32, BOARD_HEIGHT as u32])
    //     .with_camera_position([0.0, 0.0])
    //     .with_pixels_per_tile([TILE_SIZE as u32, TILE_SIZE as u32]);

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
