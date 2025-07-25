use bevy::{
    asset::weak_handle, pbr::{MaterialPipeline, MaterialPipelineKey}, prelude::*, reflect::TypePath, render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    }
};

pub const GIZMO_SHADER_HANDLE: Handle<Shader> = weak_handle!("1347c9b7-c46a-48e7-b7b8-023a354b7cbc");


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GizmoMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl From<Color> for GizmoMaterial {
    fn from(color: Color) -> Self {
        GizmoMaterial { color: color.into() }
    }
}

impl Material for GizmoMaterial {
    fn vertex_shader() -> ShaderRef {
        GIZMO_SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        GIZMO_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
