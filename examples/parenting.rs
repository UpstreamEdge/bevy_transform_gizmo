use bevy::{prelude::*, window::PresentMode};
use bevy_transform_gizmo::TransformGizmoPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Immediate,
                    ..default()
                }),
                ..default()
            }),
            TransformGizmoPlugin{
                use_tag_filter: false,
                ..Default::default()},
        ))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        // bevy_transform_gizmo::GizmoTransformable, // Not Mandatory because of  "use_tag_filter= false"
    ));

    let tan = Color::srgb_u8(204, 178, 153);
    let red = Color::srgb_u8(127, 26, 26);

    // cube
    commands
        .spawn((
                Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.0)))),
                MeshMaterial3d(materials.add(Color::srgb(0.4, 0.4, 0.4))),
                Transform::from_xyz(-1.0, 0.0, 0.0),
            ))
        .with_children(|commands| {
            commands.spawn((
                    Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.0)))),
                    MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
                    Transform::from_xyz(1.0, 0.0, 0.0),
                    // bevy_transform_gizmo::GizmoTransformable, // Not Mandatory because of  "use_tag_filter= false"
                ));
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::from_size(Vec3::splat(1.0)))),
                    MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0,1.0))),
                    Transform::from_xyz(1.0, 1.0, 0.0),
                    // bevy_transform_gizmo::GizmoTransformable, // Not Mandatory because of  "use_tag_filter= false"
                ));
        });

    // Light
    commands.spawn((
        Transform::from_xyz(4.0, 8.0, 4.0),
        PointLight::default(),
    ));
    // Camera
    commands.spawn((
        Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        bevy_transform_gizmo::GizmoPickSource,
    ));
}
