use bevy::{prelude::*, window::PrimaryWindow};

use crate::*;


pub fn transform_gizmo_picking(
    mut ray_cast: MeshRayCast,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Single<(Entity, &Camera), With<GizmoPickSource>>,
    q_transform: Query<&GlobalTransform>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut gizmo_settings: ResMut<TransformGizmoSettings>,
    mut q_gizmo: Single<&mut Transform, With<TransformGizmo>>,
    q_tagged: Query<(), With<GizmoTransformable>>,
    q_gizmo_parts: Query<(), Without<TransformGizmoPart>>,
) {
    if !mouse_input.pressed(MouseButton::Left) {
        gizmo_settings.is_dragging = false;
    }

    let (camera_entity, camera) = *q_camera;
    let Ok(camera_transform) = q_transform.get(camera_entity) else {
        warn!("TransformGizmo: Camera entity not found for picking. Ensure the camera is spawned before the picking system runs.");
        return;
    };

    let Ok(window) = primary_window.get_single() else {
        debug!("primary_window.get_single() failed in transform_gizmo_picking!");
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let filter_gizmo_parts = |entity| q_gizmo_parts.contains(entity);

    let filter = |entity| q_tagged.contains(entity);

    // Never early-exit. Note that you can change behavior per-entity.
    let early_exit_test = |_entity| false;

    // Ignore the visibility of entities. This allows ray casting hidden entities.
    let visibility = RayCastVisibility::Any;

    let settings = RayCastSettings::default()
        .with_filter(&filter_gizmo_parts)
        .with_early_exit_test(&early_exit_test)
        .with_visibility(visibility)
        .with_filter(&filter);

    let Some((hit_entity, _hit)) = ray_cast.cast_ray(ray, &settings).first() else {
        return;
    };

    // if mouse_input.just_released(gizmo_resource.selection_button) {
    if mouse_input.just_released(MouseButton::Left) {
        gizmo_settings.deselect();

        let Ok(hit_entity_transform) = q_transform.get(*hit_entity) else {
            warn!("TransformGizmo: Could not get Transform of selected Entity: {:?}", hit_entity);
            return;
        };

        // Store the active Entity
        gizmo_settings.active_entity = Some(*hit_entity);
        gizmo_settings.origin = Some(*hit_entity_transform);

        // Attach the TransformGizmo to it
        **q_gizmo = Transform::from_translation(hit_entity_transform.translation()).with_rotation(hit_entity_transform.rotation());
    }
}
