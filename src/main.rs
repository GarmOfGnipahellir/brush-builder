mod orbit_camera;

use bevy::prelude::*;
use orbit_camera::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Brush Builder".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            OrbitCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, (gizmo_origin, gizmo_grid))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((OrbitCamera::default(), Camera3dBundle::default()));
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
