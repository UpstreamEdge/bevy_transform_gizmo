#![forbid(unsafe_code)]
#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
)]
#![deny(
    clippy::todo,
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
)]

use bevy::prelude::*;
use bevy::asset::load_internal_asset;

pub mod mesh;
use mesh::*;

pub mod transformations;
use transformations::*;

pub mod gizmo_component;
use gizmo_component::*;
mod gizmo_material;
use gizmo_material::GizmoMaterial;


#[derive(Component)]
pub struct TransformGizmo;

#[derive(Component)]
pub struct GizmoPickSource;

#[derive(Component)]
pub struct GizmoTransformable;

#[derive(Component)]
pub struct TransformGizmoPart;

#[derive(Resource, Debug, Default)]
pub struct TransformGizmoSettings {
    pub(crate) active_entity: Option<Entity>,
    pub(crate) is_dragging: bool,
    pub(crate) origin: Option<GlobalTransform>,
}

impl TransformGizmoSettings {
    pub fn is_active(&self) -> bool {
        self.active_entity.is_some()
    }
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
    }
    pub fn select(&mut self, entity: Entity, origin: GlobalTransform) {
        self.active_entity = Some(entity);
        self.origin = Some(origin);
    }
    pub fn deselect(&mut self) {
        self.active_entity = None;
        self.is_dragging = false;
        self.origin = None;
    }
}

pub struct TransformGizmoPlugin;
impl Plugin for TransformGizmoPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            gizmo_material::GIZMO_SHADER_HANDLE,
            "./gizmo_material.wgsl",
            Shader::from_wgsl
        );

        app.insert_resource(TransformGizmoSettings::default());

        app.add_plugins(MeshPickingPlugin);
        app.add_plugins(MaterialPlugin::<GizmoMaterial>::default());

        app.add_systems(PostStartup, build_gizmo);

        app.add_systems(Update, debug_print_settings);

        app.add_systems(Update, (
            update_gizmo_position,
            update_gizmo_visibility,
            deactivate_gizmo_if_entity_does_not_exist
        ).chain());
    }
}

fn debug_print_settings(
    gizmo_settings: Res<TransformGizmoSettings>,
) {
    if gizmo_settings.is_active() {
        log::warn!("Active Entity: {:?}", gizmo_settings.active_entity);
        log::info!("Is Dragging: {}", gizmo_settings.is_dragging);
        if let Some(origin) = &gizmo_settings.origin {
            log::info!("Origin: {:?}", origin.translation());
        } else {
            log::info!("No Origin Set");
        }
        println!();
    } else {
        log::info!("Gizmo is not active.");
    }
}


fn update_gizmo_visibility(
    gizmo_settings: Res<TransformGizmoSettings>,
    mut gizmo_query: Query<&mut Visibility, With<TransformGizmo>>,
) {
    match gizmo_query.single_mut() {
        Ok(mut visibility) => {
            *visibility = if gizmo_settings.is_active() {
                log::info!("Gizmo is active and should be visible.");
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
        Err(err) => {
            log::error!("Failed to update transform gizmo visibility: {err}");
        }
    }
}

/// Deactivates the gizmo if the active entity does not exist in the world.
fn deactivate_gizmo_if_entity_does_not_exist(
    entity_query: Query<Entity>,
    mut gizmo_settings: ResMut<TransformGizmoSettings>,
) {
    if let Some(active_entity) = gizmo_settings.active_entity {
        if entity_query.get(active_entity).is_err() {
            // If the active entity does not exist, deactivate the gizmo
            gizmo_settings.deselect();
        }
    } else {
        // If no active entity, ensure the gizmo is deselected
        gizmo_settings.deselect();
    }
}

/// Updates the position of the gizmo to match the associated/active entity's position.
fn update_gizmo_position(
    gizmo_settings: Res<TransformGizmoSettings>,
    mut q_gizmo: Query<&mut Transform, With<TransformGizmo>>,
    q_transform: Query<&GlobalTransform>,
) {
    if let Some(active_entity) = gizmo_settings.active_entity {
        if let Ok(active_transform) = q_transform.get(active_entity) {
            if let Ok(mut gizmo_transform) = q_gizmo.single_mut() {
                *gizmo_transform = Transform::from_translation(active_transform.translation());
            }
        }
    }
}
