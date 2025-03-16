
use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};
use bevy::asset::load_internal_asset;

pub mod mesh;
use mesh::*;

pub mod  picking;
use picking::*;

pub mod  transformations;
use transformations::*;

pub mod  gizmo_component;
use gizmo_component::*;
use crate::gizmo_material::GizmoMaterial;

pub mod normalization;
use crate::normalization::*;


#[derive(Component)]
pub struct TransformGizmo;

#[derive(Component)]
pub struct GizmoPickSource;

#[derive(Component)]
pub struct GizmoTransformable;

#[derive(Component)]
pub struct TransformGizmoPart;

#[derive(Resource)]
pub struct TransformGizmoRessource{
    pub entity: Option<Entity>,
    pub original_color: Option<Handle<StandardMaterial>>,
    pub origin: Option<GlobalTransform>,
    pub use_tag_filter: bool,
    pub selection_color: Color,
    pub selection_button: MouseButton,
    pub drag_button: PointerButton,
}

impl Default for TransformGizmoRessource {
    fn default() -> Self { 
        Self {
            entity: None,
            original_color: None,
            origin: None,
            use_tag_filter: true,
            selection_color: Color::from(YELLOW_300).clone(),
            selection_button: MouseButton::Left,
            drag_button: PointerButton::Primary,
        }
    }
}

pub struct TransformGizmoPlugin{
    use_tag_filter: bool,
    selection_color: Color,
    selection_button: MouseButton,
    drag_button: PointerButton,
}

impl Default for TransformGizmoPlugin {
    fn default() -> Self { 
        Self {
            use_tag_filter: false,
            selection_color: Color::from(YELLOW_300).clone(),
            selection_button: MouseButton::Right,
            drag_button: PointerButton::Primary,
        }
    }
}

impl Plugin for TransformGizmoPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            gizmo_material::GIZMO_SHADER_HANDLE,
            "./mesh/gizmo_material.wgsl",
            Shader::from_wgsl
        );

        let resource = TransformGizmoRessource{
            use_tag_filter: self.use_tag_filter,
            selection_color:  self.selection_color,
            selection_button: self.selection_button,
            ..Default::default()
        };

        app.insert_resource(resource);
        app.add_plugins(MaterialPlugin::<GizmoMaterial>::default());
        app.add_systems(PostStartup, build_gizmo);
        app.add_systems(Update, transform_gizmo_picking);
        app.add_systems(PostUpdate,normalize);

    }
}

