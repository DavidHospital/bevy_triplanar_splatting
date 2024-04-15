use crate::triplanar_material::TriplanarMaterial;
use bevy::asset::load_internal_asset;
use bevy::prelude::*;

const TRIPLANAR_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(2631398565563939187);
const BIPLANAR_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(1945949403120376729);
pub(crate) const TRIPLANAR_MATERIAL_VERT_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(15253114830703633252);
pub(crate) const TRIPLANAR_MATERIAL_FRAG_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(6091875363090994480);

pub struct TriplanarMaterialPlugin;

impl Plugin for TriplanarMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<TriplanarMaterial>::default());

        load_internal_asset!(
            app,
            TRIPLANAR_SHADER_HANDLE,
            "shaders/triplanar.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            BIPLANAR_SHADER_HANDLE,
            "shaders/biplanar.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            TRIPLANAR_MATERIAL_VERT_SHADER_HANDLE,
            "shaders/triplanar_material_vert.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            TRIPLANAR_MATERIAL_FRAG_SHADER_HANDLE,
            "shaders/triplanar_material_frag.wgsl",
            Shader::from_wgsl
        );
    }
}
