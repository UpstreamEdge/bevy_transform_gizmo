use bevy::prelude::*;

use crate::*;

/// Marker struct that marks entities with meshes that should be scaled relative to the camera.
#[derive(Component)]
pub struct Normalize3d {
    /// Length of the object in world space units
    pub size_in_world: f32,
    /// Desired length of the object in pixels
    pub desired_pixel_size: f32,
}


// This Function Scales the Gizmo always to the correct Size
#[allow(clippy::type_complexity)]
pub fn normalize(
    q_camera: Single<(Entity, &Camera), With<GizmoPickSource>>,
    mut q_transform: Query<(Entity, &mut Transform, &Normalize3d), With <Normalize3d>>,
    q_global_transform: Query<&GlobalTransform>,
){
    for (entity, mut transform, normalize) in q_transform.iter_mut() {
        let (camera_entity, camera) = *q_camera;

        let Ok(camera_transform) = q_global_transform.get(camera_entity) else {
            warn!("Camera entity not found for normalization. Ensure the camera is spawned before the normalization system runs.");
            continue;
        };
        let view = camera_transform.compute_matrix().inverse();

        let Ok(global_transform) = q_global_transform.get(entity) else {
            warn!("Entity {:?} not found for normalization. Ensure the entity exists in the world.", entity);
            continue;
        };

        let distance = view.transform_point3(global_transform.translation()).z;
        let gt = global_transform.compute_transform();

        let Ok(pixel_root) = camera.world_to_viewport(
            &GlobalTransform::default(),
            Vec3::new(normalize.size_in_world * gt.scale.x, 0.0, distance),
        ) else {
            continue;
        };

        let Ok(pixel_end) = camera.world_to_viewport(
            &GlobalTransform::default(),
            Vec3::new(0.0, 0.0, distance),
        ) else {
            continue;
        };

        let actual_pixel_size = pixel_root.distance(pixel_end);
        let required_scale = normalize.desired_pixel_size / actual_pixel_size;
        transform.scale = gt.scale * Vec3::splat(required_scale);
    }
}
