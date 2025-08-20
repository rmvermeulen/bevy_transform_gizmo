use bevy::prelude::*;

use crate::*;

pub fn transform_gizmo_picking_1(
    windows: Single<&Window>,
    q_camera: Single<(Entity, &Camera), With<GizmoPickSource>>,
    q_transform: Query<&GlobalTransform>,
) -> Option<Ray3d> {
    let (camera_entity, camera) = *q_camera;
    let camera_transform = q_transform.get(camera_entity).unwrap();
    let cursor_position = windows.cursor_position()?;
    camera
        .viewport_to_world(camera_transform, cursor_position)
        .ok()
}

pub fn transform_gizmo_picking_2(
    In(ray): In<Option<Ray3d>>,
    mut ray_cast: MeshRayCast,
    gizmo_resource: Res<TransformGizmoResource>,
    q_tagged: Query<(), With<GizmoTransformable>>,
    q_gizmo_parts: Query<(), Without<TransformGizmoPart>>,
) -> Option<Entity> {
    let filter_gizmo_parts = |entity| q_gizmo_parts.contains(entity);

    let filter = |entity| q_tagged.contains(entity);

    // Never early-exit. Note that you can change behavior per-entity.
    let early_exit_test = |_entity| false;

    // Ignore the visibility of entities. This allows ray casting hidden entities.
    let visibility = RayCastVisibility::Any;

    let mut settings = MeshRayCastSettings::default()
        .with_filter(&filter_gizmo_parts)
        .with_early_exit_test(&early_exit_test)
        .with_visibility(visibility);

    // Allow only tagged Components to be found;
    if gizmo_resource.use_tag_filter {
        settings = settings.with_filter(&filter);
    }

    ray_cast
        .cast_ray(ray?, &settings)
        .first()
        .map(|(hit_entity, _)| hit_entity)
        .copied()
}

pub fn transform_gizmo_picking_3(
    In(hit_entity): In<Option<Entity>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut materials_3d: Query<&mut MeshMaterial3d<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmo_resource: ResMut<TransformGizmoResource>,
    mut q_gizmo: Single<&mut Transform, With<TransformGizmo>>,
    q_transform: Query<&GlobalTransform>,
) {
    let Some(hit_entity) = hit_entity else {
        return;
    };
    if mouse_input.just_released(gizmo_resource.selection_button) {
        if let Some(last_selection) = gizmo_resource.entity {
            // Reset Last Selection

            let mut material = materials_3d.get_mut(last_selection).unwrap();
            material.0 = gizmo_resource.original_color.clone().unwrap();

            gizmo_resource.origin = None;
            gizmo_resource.entity = None;
            gizmo_resource.original_color = None;
        }

        let mut material = materials_3d.get_mut(hit_entity).unwrap();
        // Store the active Entity

        gizmo_resource.entity = Some(hit_entity);
        gizmo_resource.original_color = Some(material.0.clone());
        gizmo_resource.origin = Some(*q_transform.get(hit_entity).unwrap());

        let pressed_matl = materials.add(gizmo_resource.selection_color);
        material.0 = pressed_matl;

        // Attach the TransformGizmo to it
        let sel_tranform = *q_transform.get(hit_entity).unwrap();

        **q_gizmo = Transform::from_translation(sel_tranform.translation())
            .with_rotation(sel_tranform.rotation());
    }
}
