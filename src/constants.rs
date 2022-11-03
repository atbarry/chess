use bevy::prelude::Color;

pub const BOARD_WIDTH: usize = 8;
pub const BOARD_HEIGHT: usize = 8;
pub const TILE_SIZE: f32 = 20.0;

pub const PIECE_MOVE_TIME: f32 = 1.0;
pub const PIECE_Z_LAYER: f32 = 10.0;

pub const HIGHLIGHT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub const DESTROY_COLOR: Color = Color::rgb(0.8, 0.1, 0.9);
pub const SWAP_COLOR: Color = Color::rgb(0.1, 0.8, 0.9);
pub const PROMOTE_COLOR: Color =  Color::rgb(1.0, 0.5, 0.4);
pub const SELECTED_COLOR: Color = Color::rgb(0.1, 0.3, 0.7);

pub const LIGHT_TILE_COLOR: Color = Color::rgb(0.8, 0.8, 1.0);
pub const DARK_TILE_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);