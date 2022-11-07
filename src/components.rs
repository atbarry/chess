use bevy::prelude::*;

use crate::constants::PIECE_MOVE_TIME;

#[derive(Component)]
pub struct Moveable {
    pub start_pos: Vec2,
    pub target_pos: Vec2,
    pub timer: Timer,
}

#[derive(Component)]
pub struct Tile {
    pub normal_color: Color,
}

impl Moveable {
    pub fn new(start_pos: Vec2, target_pos: Vec2) -> Self {
        // dbg!("Moving from {:?} to {:?}", start_pos, target_pos);
        Self {
            start_pos,
            target_pos,
            timer: Timer::from_seconds(PIECE_MOVE_TIME, false),
        }
    }
}
