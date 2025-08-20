use bevy::asset::load_internal_asset;
use bevy::{color::palettes::tailwind::*, prelude::*};

pub mod mesh;
use bevy_log::error;
use mesh::*;

pub mod picking;
use picking::*;

pub mod transformations;
use transformations::*;

pub mod gizmo_component;
use crate::gizmo_material::GizmoMaterial;
use gizmo_component::*;

pub mod normalization;
use crate::normalization::*;

#[derive(Clone, Component, Debug, Default)]
pub struct InternalGizmoCamera;

#[derive(Component)]
pub struct TransformGizmo;

#[derive(Component)]
pub struct GizmoPickSource;

#[derive(Component)]
pub struct GizmoTransformable;

#[derive(Component)]
pub struct TransformGizmoPart;

#[derive(Resource)]
pub struct TransformGizmoResource {
    pub entity: Option<Entity>,
    pub original_color: Option<Handle<StandardMaterial>>,
    pub origin: Option<GlobalTransform>,
    pub use_tag_filter: bool,
    pub selection_color: Color,
    pub selection_button: MouseButton,
    pub drag_button: PointerButton,
}

impl Default for TransformGizmoResource {
    fn default() -> Self {
        Self {
            entity: None,
            original_color: None,
            origin: None,
            use_tag_filter: true,
            selection_color: Color::from(YELLOW_300),
            selection_button: MouseButton::Left,
            drag_button: PointerButton::Primary,
        }
    }
}

pub struct TransformGizmoPlugin {
    pub use_tag_filter: bool,
    pub selection_color: Color,
    pub selection_button: MouseButton,
    pub drag_button: PointerButton,
}

impl Default for TransformGizmoPlugin {
    fn default() -> Self {
        Self {
            use_tag_filter: false,
            selection_color: Color::from(YELLOW_300),
            selection_button: MouseButton::Left,
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

        let resource = TransformGizmoResource {
            use_tag_filter: self.use_tag_filter,
            selection_color: self.selection_color,
            selection_button: self.selection_button,
            ..Default::default()
        };

        app.insert_resource(resource);

        app.add_plugins(MeshPickingPlugin);
        app.add_plugins(MaterialPlugin::<GizmoMaterial>::default());

        app.add_systems(PostStartup, build_gizmo);
        app.add_systems(
            Update,
            transform_gizmo_picking_1
                .pipe(transform_gizmo_picking_2)
                .pipe(transform_gizmo_picking_3),
        );
        app.add_systems(PostUpdate, normalize);
        app.add_systems(PostUpdate, gizmo_cam_copy_settings);
    }
}

#[allow(clippy::type_complexity)]
fn gizmo_cam_copy_settings(
    main_cam: Query<(Ref<Camera>, Ref<GlobalTransform>, Ref<Projection>), With<GizmoPickSource>>,
    gizmo_cam: Single<
        (&mut Camera, &mut GlobalTransform, &mut Projection),
        (With<InternalGizmoCamera>, Without<GizmoPickSource>),
    >,
) {
    let (main_cam, main_cam_pos, main_proj) = if let Ok(x) = main_cam.single() {
        x
    } else {
        error!("No `GizmoPickSource` found! Insert the `GizmoPickSource` component onto your primary 3d camera");
        return;
    };
    let (mut gizmo_cam, mut gizmo_cam_pos, mut proj) = gizmo_cam.into_inner();
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
