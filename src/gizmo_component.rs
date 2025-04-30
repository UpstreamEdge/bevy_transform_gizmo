use bevy::{
    core_pipeline::core_3d::Camera3dDepthLoadOp,
    pbr::NotShadowCaster,
    prelude::*,
    render::view::RenderLayers,
};

use crate::*;
// use crate::normalization::*;


/// Startup system that builds the procedural mesh and materials of the gizmo.
pub fn build_gizmo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GizmoMaterial>>,
) {
    let axis_length = 1.3;
    let arc_radius = 1.;
    let plane_size = axis_length * 0.25;
    let plane_offset = plane_size / 2. + axis_length * 0.2;
    // Define gizmo meshes
    let arrow_tail_mesh = meshes.add(Capsule3d {
        radius: 0.04,
        half_length: axis_length * 0.5f32,
    });
    let arrow_tail_mesh_y = meshes.add(Capsule3d {
        radius: 0.04,
        half_length: axis_length * 0.333f32,
    });
    let cone_mesh = meshes.add(cone::Cone {
        height: 0.25,
        radius: 0.10,
        ..Default::default()
    });
    let plane_mesh = meshes.add(Plane3d::default().mesh().size(plane_size, plane_size));
    let sphere_mesh = meshes.add(Sphere { radius: 0.2 });
    let rotation_mesh = meshes.add(Mesh::from(truncated_torus::TruncatedTorus {
        radius: arc_radius,
        ring_radius: 0.04,
        ..Default::default()
    }));

    // Define gizmo materials
    let (s, l) = (0.8, 0.6);
    let gizmo_matl_x = materials.add(GizmoMaterial::from(Color::hsl(0.0, s, l)));
    let gizmo_matl_y = materials.add(GizmoMaterial::from(Color::hsl(120.0, s, l)));
    let gizmo_matl_z = materials.add(GizmoMaterial::from(Color::hsl(240.0, s, l)));
    let gizmo_matl_x_sel = materials.add(GizmoMaterial::from(Color::hsl(0.0, s, l)));
    let gizmo_matl_y_sel = materials.add(GizmoMaterial::from(Color::hsl(120.0, s, l)));
    let gizmo_matl_z_sel = materials.add(GizmoMaterial::from(Color::hsl(240.0, s, l)));
    let gizmo_matl_v_sel = materials.add(GizmoMaterial::from(Color::hsl(0., 0.0, l)));

    // Build the gizmo using the variables above.
    let parent = commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::Hidden,
        RenderLayers::layer(12),
        TransformGizmo,
        // Normalize3d{
        //     size_in_world: 1.5,
        //     desired_pixel_size : 150.0
        // },
        )).id();


    // Translation Axes
    let translation_x_axis = commands.spawn((
        Mesh3d(arrow_tail_mesh.clone()),
        MeshMaterial3d(gizmo_matl_x.clone()),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_z(std::f32::consts::PI / 2.0),
            Vec3::new(axis_length / 2.0, 0.0, 0.0),
        )),
        RenderLayers::layer(12),
        NotShadowCaster,
        TransformGizmoPart,
    )).id();


    commands.entity(parent).add_children(&[translation_x_axis]);
    commands.entity(translation_x_axis).observe(transform_axis);

    let translation_y_axis = commands.spawn((
        Mesh3d(arrow_tail_mesh_y.clone()),
        MeshMaterial3d(gizmo_matl_y.clone()),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_y(std::f32::consts::PI / 2.0),
            Vec3::new(0.0, axis_length / 3.0, 0.0),
        )),
        NotShadowCaster,
        TransformGizmoPart,
        RenderLayers::layer(12),
    )).id();

    commands.entity(parent).add_children(&[translation_y_axis]);
    commands.entity(translation_y_axis).observe(transform_axis);

    let translation_z_axis = commands.spawn((
        Mesh3d(arrow_tail_mesh.clone()),
        MeshMaterial3d(gizmo_matl_z.clone()),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_x(std::f32::consts::PI / 2.0),
            Vec3::new(0.0, 0.0, axis_length / 2.0),
        )),
        NotShadowCaster,
        TransformGizmoPart,
        RenderLayers::layer(12),
    )).id();

    commands.entity(parent).add_children(&[translation_z_axis]);
    commands.entity(translation_z_axis).observe(transform_axis);

    // Translation Handles
    let translation_x_handle = commands.spawn((
        Mesh3d(cone_mesh.clone()),
        MeshMaterial3d(gizmo_matl_x_sel.clone()),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_z(std::f32::consts::PI / -2.0),
            Vec3::new(axis_length, 0.0, 0.0),
        )),
        NotShadowCaster,
        TransformGizmoPart,
        RenderLayers::layer(12),
    )).id();

    commands.entity(parent).add_children(&[translation_x_handle]);
    commands.entity(translation_x_handle).observe(transform_axis);

    let translation_x_plane = commands.spawn((
        Mesh3d(plane_mesh.clone()),
        MeshMaterial3d( gizmo_matl_x_sel.clone()),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_z(std::f32::consts::PI / -2.0),
            Vec3::new(0., plane_offset, plane_offset),
        )),
        NotShadowCaster,
        TransformGizmoPart,
        RenderLayers::layer(12),
    )).id();

    commands.entity(parent).add_children(&[translation_x_plane]);
    commands.entity(translation_x_plane).observe(transform_plane);


    let translation_y_handle = commands.spawn((
        Mesh3d(cone_mesh.clone()),
        MeshMaterial3d(gizmo_matl_y_sel.clone()),
        Transform::from_translation(Vec3::new(0.0, axis_length * 0.667, 0.0)),
        NotShadowCaster,
        TransformGizmoPart,
        RenderLayers::layer(12),

    )).id();

    commands.entity(parent).add_children(&[translation_y_handle]);
    commands.entity(translation_y_handle).observe(transform_axis);


    let translation_y_plane = commands.spawn((
        Mesh3d( plane_mesh.clone()),
        MeshMaterial3d( gizmo_matl_y_sel.clone()),
         Transform::from_translation(Vec3::new(
            plane_offset,
            0.0,
            plane_offset,
        )),
        NotShadowCaster,
        TransformGizmoPart,
        RenderLayers::layer(12),
    )).id();

    commands.entity(parent).add_children(&[translation_y_plane]);
    commands.entity(translation_y_plane).observe(transform_plane);

    let translation_z_handle = commands.spawn((
        Mesh3d( cone_mesh.clone()),
        MeshMaterial3d( gizmo_matl_z_sel.clone()),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_x(std::f32::consts::PI / 2.0),
            Vec3::new(0.0, 0.0, axis_length),
        )),
        NotShadowCaster,
        TransformGizmoPart,
        RenderLayers::layer(12),
    )).id();

    commands.entity(parent).add_children(&[translation_z_handle]);
    commands.entity(translation_z_handle).observe(transform_axis);

    let translation_z_plane = commands.spawn((
        Mesh3d( plane_mesh.clone()),
        MeshMaterial3d( gizmo_matl_z_sel.clone()),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_x(std::f32::consts::PI / 2.0),
            Vec3::new(plane_offset, plane_offset, 0.0),
        )),
        NotShadowCaster,
        TransformGizmoPart,
        RenderLayers::layer(12),
    )).id();

    commands.entity(parent).add_children(&[translation_z_plane]);
    commands.entity(translation_z_plane).observe(transform_plane);

    let handle = commands.spawn((
        Mesh3d( sphere_mesh.clone()),
        MeshMaterial3d( gizmo_matl_v_sel.clone()),
        NotShadowCaster,
        TransformGizmoPart,
        RenderLayers::layer(12),
    )).id();

    commands.entity(parent).add_children(&[handle]);
    commands.entity(handle).observe(transform_camera_plane);


    // Rotation Arcs
    // let rotation_x_arc = commands.spawn((
    //     Mesh3d(rotation_mesh.clone()),
    //     MeshMaterial3d(gizmo_matl_x.clone()),
    //     Transform::from_rotation(Quat::from_axis_angle(
    //         Vec3::Z,
    //         f32::to_radians(90.0),
    //     )),
    //     NotShadowCaster,
    //     TransformGizmoPart,
    //     RenderLayers::layer(12),
    // )).id();

    // commands.entity(parent).add_children(&[rotation_x_arc]);
    // commands.entity(rotation_x_arc).observe(transform_rotation);


    let rotation_y_arc = commands.spawn((
        Mesh3d(rotation_mesh.clone()),
        MeshMaterial3d(gizmo_matl_y.clone()),
        NotShadowCaster,
        TransformGizmoPart,
        RenderLayers::layer(12),
    )).id();

    commands.entity(parent).add_children(&[rotation_y_arc]);
    commands.entity(rotation_y_arc).observe(transform_rotation);

    // let rotation_z_arc = commands.spawn((
    //     Mesh3d(rotation_mesh.clone()),
    //     MeshMaterial3d(gizmo_matl_z.clone()),
    //     Transform::from_rotation(
    //         Quat::from_axis_angle(Vec3::Z, f32::to_radians(90.0))
    //             * Quat::from_axis_angle(Vec3::X, f32::to_radians(90.0)),
    //     ),
    //     NotShadowCaster,
    //     TransformGizmoPart,
    //     RenderLayers::layer(12),
    // )).id();

    // commands.entity(parent).add_children(&[rotation_z_arc]);
    // commands.entity(rotation_z_arc).observe(transform_rotation);

    commands.spawn((
        Camera3d {
            depth_load_op: Camera3dDepthLoadOp::Clear(0.),
            ..default()
        },
        Camera {
            clear_color: ClearColorConfig::None,
            ..default()
        },
        InternalGizmoCamera,
        RenderLayers::layer(12),
    ));
}
