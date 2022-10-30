use bevy::{prelude::*, render::camera::RenderTarget};
use crate::{board::{Board, Tile, Piece, BoardPos}, constants::PIECE_Z_LAYER};

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedSquare::default())
            .insert_resource(HiglightedSquares{ squares: Vec::new() })
            .add_system(on_click)
            .add_system(highlight_squares);
        
    }
}

pub struct SelectedSquare{
    changed: bool,
    tile: Option<Entity>,
    piece: Option<Piece>,
}

pub struct HiglightedSquares{
    squares: Vec<(Entity, Color)>,
}

impl Default for SelectedSquare {
    fn default() -> Self {
        Self{
            changed: false,
            tile: None,
            piece: None,
        }
    }
}

pub fn on_click(
    mut commands: Commands,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<Input<MouseButton>>,
    mut board: ResMut<Board>,
    mut selected: ResMut<SelectedSquare>,
    mut highlighted: ResMut<HiglightedSquares>,
) {
    // if there was no click, don't do anything
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    selected.changed = true;
    
    if let Some(target_square) = get_square_from_mouse(wnds, q_camera) {
        println!("Clicked on board at: {}, {}", target_square.x, target_square.y);

        if let Some(piece) = &selected.piece {
            board.move_piece(&mut commands, piece.board_pos, target_square);
            selected.piece = None;
            selected.tile = None;
        } else {
            selected.piece = board.get_piece(target_square);  
            selected.tile = Some(board.get_tile_entity(target_square));  
             // highlighted.squares = vec![(board.get_tile_entity(x + 1, y + 1), Color::rgb(0.0, 7.0, 1.0))];
        }
        
    } else {
        selected.piece = None;
        selected.tile = None;
    }
}

pub fn highlight_squares(
    mut selected: ResMut<SelectedSquare>,
    mut highlighted: ResMut<HiglightedSquares>,
    mut q_tile: Query<(Entity, &mut Sprite, &Tile)>,
) {
    if !selected.changed || selected.tile.is_none() {
        return;
    }

    let entity_in_highlighted = |entity: Entity| -> Option<Color> {
        for (e, h) in highlighted.squares.iter() {
            if *e == entity {
                return Some(*h);
            }
        }
        
        None
    };

    for (entity, mut sprite, tile) in q_tile.iter_mut() {
        if Some(entity) == selected.tile {
            sprite.color = Color::rgb(0.8, 0.1, 0.1);
        } else if let Some(color) = entity_in_highlighted(entity) {
            sprite.color = color;
        } else {
            sprite.color = tile.normal_color;
        }
    }

    // clear the highlighted squares
    highlighted.squares.clear();
    selected.changed = false;
}

fn get_square_from_mouse(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) -> Option<BoardPos> {
    if let Some(world_pos) = mouse_to_world(wnds, q_camera){
        let board_pos = BoardPos::world_to_board(world_pos.extend(PIECE_Z_LAYER));
        return board_pos;
    } 

    None
}


fn mouse_to_world(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        return Some(world_pos);
    } else{
        return None;
    }
}