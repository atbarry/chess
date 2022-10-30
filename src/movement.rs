use bevy::prelude::*;

use crate::constants::{PIECE_Z_LAYER, PIECE_MOVE_TIME};

pub struct MovePlugin;

impl Plugin for MovePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_pieces);
    }
}

fn move_pieces(
    mut commands: Commands,
    mut q_moveable: Query<(Entity, &mut Moveable, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut moveable, mut transform) in q_moveable.iter_mut() {
        if moveable.timer.tick(time.delta()).just_finished() {
            transform.translation = moveable.target_pos.extend(PIECE_Z_LAYER);
            commands.entity(entity).remove::<Moveable>();
        } else {
            transform.translation = moveable.start_pos.lerp(
                moveable.target_pos, smoothstep(moveable.timer.percent())
            ).extend(PIECE_Z_LAYER);
        }
    }
}

fn smoothstep(x: f32) -> f32 {
    (x * std::f32::consts::PI / 2.0).sin().powi(3)
}


#[derive(Component)]
pub struct Moveable {
    start_pos: Vec2,
    target_pos: Vec2,
    timer: Timer,
}

impl Moveable {
    pub fn new(start_pos: Vec2, target_pos: Vec2) -> Self {
        Self {
            start_pos,
            target_pos,
            timer: Timer::from_seconds(PIECE_MOVE_TIME, false),
        }
    }
}