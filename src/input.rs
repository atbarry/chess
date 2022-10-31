use bevy::{prelude::*, render::camera::RenderTarget};

use crate::board::BoardPos;

pub struct InputPlugin; 

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseInfo{
            world_cords: None,
            board_pos: None,
            just_clicked: false,
        }).add_system(update_mouse);

    }
}

pub struct MouseInfo {
    pub world_cords: Option<Vec2>,
    pub board_pos: Option<BoardPos>,
    pub just_clicked: bool, 
}


fn update_mouse(
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mouse_button: Res<Input<MouseButton>>,
    mut mouse: ResMut<MouseInfo>
) {
    mouse.world_cords = mouse_to_world(wnds, q_camera);

    mouse.board_pos = match mouse.world_cords {
        Some(pos) => BoardPos::world_to_board(pos.extend(0.0)),
        None => None,
    };

    mouse.just_clicked = mouse_button.just_pressed(MouseButton::Left);
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


