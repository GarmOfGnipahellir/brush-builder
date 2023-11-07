mod brush;
mod orbit_camera;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
};
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
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            WireframePlugin::default(),
            OrbitCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, (gizmo_origin, gizmo_grid, gizmo_brush))
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands
        .spawn((OrbitCamera::default(), Camera3dBundle::default()))
        .with_children(|parent| {
            parent.spawn(DirectionalLightBundle::default());
        });

    let brush = brush::Brush::from_planes(&[
        brush::Plane::new(Vec3::X, 0.5),
        brush::Plane::new(Vec3::Y, 0.5),
        brush::Plane::new(Vec3::Z, 0.5),
        brush::Plane::new(Vec3::NEG_X, 0.5),
        brush::Plane::new(Vec3::NEG_Y, 0.5),
        brush::Plane::new(Vec3::NEG_Z, 0.5),
        brush::Plane::new(Vec3::ONE.normalize(), 0.5),
    ]);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(brush.to_mesh()),
            ..Default::default()
        },
        Wireframe,
    ));

    let brush = brush::Brush::from_planes(&[
        brush::Plane::new(Vec3::X, 0.75),
        brush::Plane::new(Vec3::Y, 0.0),
        brush::Plane::new(Vec3::Z, 0.75),
        brush::Plane::new(Vec3::NEG_X, 0.75),
        brush::Plane::new(Vec3::NEG_Y, 1.0),
        brush::Plane::new(Vec3::NEG_Z, 0.75),
    ]);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(brush.to_mesh()),
            ..Default::default()
        },
        Wireframe,
    ));
}

fn gizmo_origin(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::X * 0.1, Color::RED);
    gizmos.line(Vec3::ZERO, Vec3::Y * 0.1, Color::GREEN);
    gizmos.line(Vec3::ZERO, Vec3::Z * 0.1, Color::BLUE);
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

fn gizmo_brush(mut gizmos: Gizmos) {
    let brush = brush::Brush::from_planes(&[
        brush::Plane::new(Vec3::X, 0.5),
        brush::Plane::new(Vec3::Y, 0.5),
        brush::Plane::new(Vec3::Z, 0.5),
        brush::Plane::new(Vec3::NEG_X, 0.5),
        brush::Plane::new(Vec3::NEG_Y, 0.5),
        brush::Plane::new(Vec3::NEG_Z, 0.5),
        brush::Plane::new(Vec3::ONE.normalize(), 0.5),
    ]);

    for p in &brush.polys {
        let pos = p.center();

        gizmos.ray(pos, p.tangent * 0.1, Color::RED);
        gizmos.ray(pos, p.bitangent * 0.1, Color::GREEN);
        gizmos.ray(pos, p.normal * 0.1, Color::BLUE);

        for (i, &v0) in p.verts.iter().enumerate() {
            let v1 = p.verts[(i + 1) % p.verts.len()];

            gizmos.line(v0, v1, Color::WHITE);
        }
    }
}
