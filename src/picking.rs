use bevy::prelude::*;

use crate::*;


pub fn transform_gizmo_picking(
    mut ray_cast: MeshRayCast,
    windows: Single<&Window>,
    q_camera: Single<(Entity, &Camera), With<GizmoPickSource>>,
    q_transform: Query<&GlobalTransform>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut materials_3d: Query<&mut MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_ressource: ResMut<TransformGizmoRessource>,
    mut q_gizmo: Single<&mut Transform, With<TransformGizmo>>,
    q_tagged: Query<(), With<GizmoTransformable>>,
    q_gizmo_parts: Query<(), Without<TransformGizmoPart>>,
) {
    let (camera_entity, camera) = *q_camera;
    let camera_transform =  q_transform.get(camera_entity).unwrap();
    let Some(cursor_position) = windows.cursor_position() else {
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

    let mut settings = RayCastSettings::default()
        .with_filter(&filter_gizmo_parts)
        .with_early_exit_test(&early_exit_test)
        .with_visibility(visibility);

    // Allow only tagged Components to be found;
    if gizmo_ressource.use_tag_filter {

        settings = settings.with_filter(&filter);
    }

    let Some((hit_entity, _hit)) = ray_cast.cast_ray(ray, &settings).first() else {
        return;
    };

    if mouse_input.just_released(gizmo_ressource.selection_button){
        if let Some(last_selection) = gizmo_ressource.entity {
            // Reset Last Selection

            let Ok(mut material) = materials_3d.get_mut(last_selection) else {
                warn!("TransformGizmo: Could not get Material of last selected Entity: {:?}", last_selection);
                return;
            };
            if let Some(original_color) = gizmo_ressource.original_color.clone() {
                material.0 = original_color;
            } else {
                warn!("TransformGizmo: No original color found for last selected Entity: {:?}", last_selection);
            }

            gizmo_ressource.origin = None;
            gizmo_ressource.entity = None;
            gizmo_ressource.original_color = None;
        }

        let Ok(mut material) = materials_3d.get_mut(*hit_entity) else {
            warn!("TransformGizmo: Could not get Material of selected Entity: {:?}", hit_entity);
            return;
        };
        let Ok(hit_entity_transform) = q_transform.get(*hit_entity) else {
            warn!("TransformGizmo: Could not get Transform of selected Entity: {:?}", hit_entity);
            return;
        };

        // Store the active Entity
        gizmo_ressource.entity = Some(*hit_entity);
        gizmo_ressource.original_color = Some(material.0.clone());
        gizmo_ressource.origin = Some(*hit_entity_transform);

        let pressed_matl = materials.add(gizmo_ressource.selection_color);
        material.0 = pressed_matl;

        // Attach the TransformGizmo to it
        **q_gizmo = Transform::from_translation(hit_entity_transform.translation()).with_rotation(hit_entity_transform.rotation());
    }
}
