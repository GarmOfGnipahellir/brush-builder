use bevy::{
    input::{
        mouse::{MouseScrollUnit, MouseWheel},
        touchpad::TouchpadMagnify,
    },
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Brush Builder".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            PreUpdate,
            (
                orbit_camera_rotate_input,
                orbit_camera_pan_input,
                orbit_camera_zoom_input,
            ),
        )
        .add_systems(Update, orbit_camera)
        .add_systems(PostUpdate, (gizmo_origin, gizmo_grid))
        .run();
}

#[derive(Component)]
struct OrbitCamera {
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

fn setup(mut commands: Commands) {
    commands.spawn((OrbitCamera::default(), Camera3dBundle::default()));
}

fn orbit_camera_rotate_input(
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

fn orbit_camera_pan_input(
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

fn orbit_camera_zoom_input(
    mut magnify_evr: EventReader<TouchpadMagnify>,
    mut query: Query<&mut OrbitCamera>,
) {
    for ev in magnify_evr.read() {
        for mut orbit_camera in &mut query {
            orbit_camera.distance = (orbit_camera.distance.sqrt() - ev.0).powi(2);
        }
    }
}

fn orbit_camera(mut query: Query<(&mut Transform, &OrbitCamera)>) {
    for (mut transform, orbit_camera) in &mut query {
        orbit_camera.update_transform(&mut transform);
    }
}

fn gizmo_origin(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::X, Color::RED);
    gizmos.line(Vec3::ZERO, Vec3::Y, Color::GREEN);
    gizmos.line(Vec3::ZERO, Vec3::Z, Color::BLUE);
}

fn gizmo_grid(mut gizmos: Gizmos) {
    let num = 32;
    let step = 0.1;
    let size = step * num as f32;

    for i in 0..=num {
        let x = step * i as f32 - size * 0.5;

        gizmos.line(
            Vec3::new(x, 0.0, -size * 0.5),
            Vec3::new(x, 0.0, size * 0.5),
            Color::WHITE.with_a(0.1),
        );

        gizmos.line(
            Vec3::new(-size * 0.5, 0.0, x),
            Vec3::new(size * 0.5, 0.0, x),
            Color::WHITE.with_a(0.1),
        );
    }
}
