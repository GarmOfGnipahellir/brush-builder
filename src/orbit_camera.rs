use bevy::prelude::*;

#[derive(Component)]
pub struct OrbitCamera {
    pivot: Vec3,
    distance: f32,
    yaw: f32,
    pitch: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            pivot: Vec3::ZERO,
            distance: 5.0,
            yaw: std::f32::consts::FRAC_PI_4,
            pitch: -std::f32::consts::FRAC_PI_8,
        }
    }
}

impl OrbitCamera {
    fn update_transform(&self, transform: &mut Transform) {
        transform.rotation = Quat::from_euler(EulerRot::YXZ, self.yaw, self.pitch, 0.0);
        transform.translation =
            transform.rotation * Vec3::new(0.0, 0.0, self.distance) + self.pivot;
    }
}

fn orbit_camera(mut query: Query<(&mut Transform, &OrbitCamera)>) {
    for (mut transform, orbit_camera) in &mut query {
        orbit_camera.update_transform(&mut transform);
    }
}

#[cfg(not(target_os = "macos"))]
mod default {
    use super::*;
    use bevy::input::mouse::{MouseMotion, MouseWheel};

    pub(super) fn orbit_camera_rotate_input(
        keycode: Res<Input<KeyCode>>,
        button: Res<Input<MouseButton>>,
        mut motion_evr: EventReader<MouseMotion>,
        mut query: Query<&mut OrbitCamera>,
    ) {
        if keycode.pressed(KeyCode::ShiftLeft) || !button.pressed(MouseButton::Middle) {
            return;
        }

        for ev in motion_evr.read() {
            for mut orbit_camera in &mut query {
                orbit_camera.yaw -= ev.delta.x * 0.002;
                orbit_camera.pitch -= ev.delta.y * 0.002;
            }
        }
    }

    pub(super) fn orbit_camera_pan_input(
        keycode: Res<Input<KeyCode>>,
        button: Res<Input<MouseButton>>,
        mut motion_evr: EventReader<MouseMotion>,
        mut query: Query<(&GlobalTransform, &mut OrbitCamera)>,
    ) {
        if !keycode.pressed(KeyCode::ShiftLeft) || !button.pressed(MouseButton::Middle) {
            return;
        }

        for ev in motion_evr.read() {
            for (transform, mut orbit_camera) in &mut query {
                let transform = transform.compute_transform();
                orbit_camera.pivot -= transform.local_x() * ev.delta.x * 0.002;
                orbit_camera.pivot += transform.local_y() * ev.delta.y * 0.002;
            }
        }
    }

    pub(super) fn orbit_camera_zoom_input(
        mut scroll_evr: EventReader<MouseWheel>,
        mut query: Query<&mut OrbitCamera>,
    ) {
        for ev in scroll_evr.read() {
            match ev.unit {
                bevy::input::mouse::MouseScrollUnit::Line => {
                    for mut orbit_camera in &mut query {
                        orbit_camera.distance = (orbit_camera.distance.sqrt() - ev.y * 0.1).powi(2);
                    }
                }
                bevy::input::mouse::MouseScrollUnit::Pixel => todo!(),
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use super::*;
    use bevy::input::{
        mouse::{MouseScrollUnit, MouseWheel},
        touchpad::TouchpadMagnify,
    };

    pub(super) fn orbit_camera_rotate_input(
        keycode: Res<Input<KeyCode>>,
        mut scroll_evr: EventReader<MouseWheel>,
        mut query: Query<&mut OrbitCamera>,
    ) {
        if keycode.pressed(KeyCode::ShiftLeft) {
            return;
        }

        for ev in scroll_evr.read() {
            match ev.unit {
                MouseScrollUnit::Line => {}
                MouseScrollUnit::Pixel => {
                    for mut orbit_camera in &mut query {
                        orbit_camera.yaw -= ev.x * 0.001;
                        orbit_camera.pitch -= ev.y * 0.001;
                    }
                }
            }
        }
    }

    pub(super) fn orbit_camera_pan_input(
        keycode: Res<Input<KeyCode>>,
        mut scroll_evr: EventReader<MouseWheel>,
        mut query: Query<(&GlobalTransform, &mut OrbitCamera)>,
    ) {
        if !keycode.pressed(KeyCode::ShiftLeft) {
            return;
        }

        for ev in scroll_evr.read() {
            match ev.unit {
                MouseScrollUnit::Line => {}
                MouseScrollUnit::Pixel => {
                    for (transform, mut orbit_camera) in &mut query {
                        let transform = transform.compute_transform();
                        orbit_camera.pivot -= transform.local_x() * ev.x * 0.001;
                        orbit_camera.pivot += transform.local_y() * ev.y * 0.001;
                    }
                }
            }
        }
    }

    pub(super) fn orbit_camera_zoom_input(
        mut magnify_evr: EventReader<TouchpadMagnify>,
        mut query: Query<&mut OrbitCamera>,
    ) {
        for ev in magnify_evr.read() {
            for mut orbit_camera in &mut query {
                orbit_camera.distance = (orbit_camera.distance.sqrt() - ev.0).powi(2);
            }
        }
    }
}

pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_os = "macos"))]
        app.add_systems(
            PreUpdate,
            (
                default::orbit_camera_rotate_input,
                default::orbit_camera_pan_input,
                default::orbit_camera_zoom_input,
            ),
        );
        #[cfg(target_os = "macos")]
        app.add_systems(
            PreUpdate,
            (
                macos::orbit_camera_rotate_input,
                macos::orbit_camera_pan_input,
                macos::orbit_camera_zoom_input,
            ),
        );
        app.add_systems(Update, orbit_camera);
    }
}
