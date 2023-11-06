use macroquad::prelude::*;

struct CameraData {
    pivot: Vec3,
    distance: f32,
    yaw: f32,
    pitch: f32,
    transform: Mat4,
}

impl Default for CameraData {
    fn default() -> Self {
        Self {
            pivot: Vec3::ZERO,
            distance: 2.0,
            yaw: std::f32::consts::FRAC_PI_4,
            pitch: std::f32::consts::FRAC_PI_8,
            transform: Mat4::from_rotation_translation(
                Quat::from_euler(
                    EulerRot::YXZ,
                    std::f32::consts::FRAC_PI_4,
                    std::f32::consts::FRAC_PI_8,
                    0.0,
                ),
                vec3(-2.0, 1.0, -2.0),
            ),
        }
    }
}

impl CameraData {
    fn right(&self) -> Vec3 {
        self.transform.col(0).xyz()
    }

    fn up(&self) -> Vec3 {
        self.transform.col(1).xyz()
    }

    fn forward(&self) -> Vec3 {
        self.transform.col(2).xyz()
    }

    fn position(&self) -> Vec3 {
        self.transform.col(3).xyz()
    }

    fn to_camera_3d(&self) -> Camera3D {
        Camera3D {
            position: self.position(),
            target: self.position() + self.forward(),
            up: self.up(),
            ..Default::default()
        }
    }

    fn compute_transform(&mut self) {
        let rotation = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);
        let translation = rotation * vec3(0.0, 0.0, -self.distance) + self.pivot;
        self.transform = Mat4::from_rotation_translation(rotation, translation);
    }

    fn add_orbit_yaw_input(&mut self, input: f32) {
        self.yaw += input;
    }

    fn add_orbit_pitch_input(&mut self, input: f32) {
        self.pitch += input;
    }

    fn add_orbit_input(&mut self, input: Vec2) {
        self.add_orbit_yaw_input(input.x);
        self.add_orbit_pitch_input(input.y);
    }

    fn add_pan_right_input(&mut self, input: f32) {
        self.pivot += self.right() * input;
    }

    fn add_pan_up_input(&mut self, input: f32) {
        self.pivot += self.up() * input;
    }

    fn add_pan_input(&mut self, input: Vec2) {
        self.add_pan_right_input(input.x);
        self.add_pan_up_input(input.y);
    }

    fn add_zoom_input(&mut self, input: f32) {
        self.distance = (self.distance.sqrt() + input).powi(2);
    }
}

#[macroquad::main("Brush Builder")]
async fn main() {
    let mut camera_data = CameraData::default();

    loop {
        clear_background(BLACK);

        let mouse_delta = mouse_delta_position();
        if is_mouse_button_down(MouseButton::Middle) {
            if is_key_down(KeyCode::LeftShift) {
                camera_data.add_pan_input(-mouse_delta);
            } else {
                camera_data.add_orbit_input(mouse_delta * vec2(1.0, -1.0));
            }
        }
        camera_data.add_zoom_input(mouse_wheel().1 * -0.001);

        camera_data.compute_transform();
        set_camera(&camera_data.to_camera_3d());

        draw_line_3d(Vec3::ZERO, Vec3::X, RED);
        draw_line_3d(Vec3::ZERO, Vec3::Y, GREEN);
        draw_line_3d(Vec3::ZERO, Vec3::Z, BLUE);

        draw_cube_wires(Vec3::ZERO, Vec3::ONE, WHITE);

        // draw_grid(32, 0.1, WHITE, GRAY);

        next_frame().await
    }
}
