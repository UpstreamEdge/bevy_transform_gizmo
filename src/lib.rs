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

pub mod  picking;
use picking::*;

pub mod transformations;
use transformations::*;

pub mod gizmo_component;
use gizmo_component::*;
mod gizmo_material;
use gizmo_material::GizmoMaterial;

// pub mod normalization;
// use crate::normalization::*;

#[derive(Component, Default, Clone, Debug)]
pub struct InternalGizmoCamera;


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
    pub active_entity: Option<Entity>,
    pub is_dragging: bool,
    pub(crate) origin: Option<GlobalTransform>,
}

impl TransformGizmoSettings {
    pub fn is_active(&self) -> bool {
        self.active_entity.is_some()
    }
    pub fn is_dragging(&self) -> bool {
        self.is_dragging
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

        app.add_systems(Update, transform_gizmo_picking);
        // app.add_systems(PostUpdate,normalize);
        app.add_systems(PostUpdate,gizmo_cam_copy_settings);

    }
}


fn gizmo_cam_copy_settings(
    main_cam: Query<(Ref<Camera>, Ref<GlobalTransform>, Ref<Projection>), With<GizmoPickSource>>,
    mut gizmo_cam: Query<
        (&mut Camera, &mut GlobalTransform, &mut Projection),
        (With<InternalGizmoCamera>, Without<GizmoPickSource>),
    >,
) {
    let (main_cam, main_cam_pos, main_proj) = if let Ok(x) = main_cam.get_single() {
        x
    } else {
        error!("No `GizmoPickSource` found! Insert the `GizmoPickSource` component onto your primary 3d camera");
        return;
    };
    let (mut gizmo_cam, mut gizmo_cam_pos, mut proj) = gizmo_cam.single_mut();
    if main_cam_pos.is_changed() {
        *gizmo_cam_pos = *main_cam_pos;
    }
    if main_cam.is_changed() {
        *gizmo_cam = main_cam.clone();
        gizmo_cam.order += 10;
    }
    if main_proj.is_changed() {
        *proj = main_proj.clone();
    }
}
