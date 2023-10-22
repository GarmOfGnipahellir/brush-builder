use macroquad::prelude::*;

struct CameraData {
    pivot: Vec3,
    rotation: Quat,
    distance: f32,
}

impl CameraData {
    fn position(&self) -> Vec3 {
        self.rotation * Vec3::new(0.0, 0.0, self.distance)
    }

    fn to_camera_3d(&self) -> Camera3D {
        Camera3D {
            position: self.position(),
            target: self.pivot,
            up: Vec3::Y,
            ..Default::default()
        }
    }
}

#[macroquad::main("Brush Builder")]
async fn main() {
    let mut camera_data = CameraData {
        pivot: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        distance: 2.0,
    };

    loop {
        clear_background(BLACK);

        let mouse_delta = mouse_delta_position();
        if is_mouse_button_down(MouseButton::Right) {
            camera_data.rotation *= Quat::from_rotation_y(mouse_delta.x);
            camera_data.rotation *= Quat::from_rotation_x(mouse_delta.y);
        }

        set_camera(&camera_data.to_camera_3d());

        draw_line_3d(Vec3::ZERO, Vec3::X, RED);
        draw_line_3d(Vec3::ZERO, Vec3::Y, GREEN);
        draw_line_3d(Vec3::ZERO, Vec3::Z, BLUE);

        draw_grid(32, 0.1, WHITE, GRAY);

        next_frame().await
    }
}
