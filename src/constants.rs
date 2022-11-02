use bevy::prelude::Color;

pub const BOARD_WIDTH: usize = 8;
pub const BOARD_HEIGHT: usize = 8;
pub const TILE_SIZE: f32 = 20.0;

pub const PIECE_MOVE_TIME: f32 = 1.0;
pub const PIECE_Z_LAYER: f32 = 10.0;

pub const HIGHLIGHT_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);
pub const DESTROY_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
pub const SWAP_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);
pub const PROMOTE_COLOR: Color = Color::rgb(1.0, 1.0, 0.0);
pub const SELECTED_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);