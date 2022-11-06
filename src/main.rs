use bevy::{prelude::*, render::{texture::ImageSettings, camera::ScalingMode}};
use bevy_inspector_egui::WorldInspectorPlugin;
use board::{Board, PieceSpawner}; 
use systems::SystemsPlugin;
use constants::*;
use input::InputPlugin;
use resources::ResourcesPlugin;

mod systems;
mod board;
mod constants;
mod components;
mod resources;
mod input; 

#[cfg(debug_assertions)]
fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ResourcesPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(SystemsPlugin)
        .add_startup_system(create_board)
        .add_startup_system(spawn_camera)
        .run();
}

#[cfg(not(debug_assertions))]
fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(ResourcesPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(SystemsPlugin)
        .add_startup_system(create_board)
        .add_startup_system(spawn_camera)
        .run();
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







