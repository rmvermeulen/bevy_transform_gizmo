use bevy::prelude::*;

use crate::*;

/// This Observer Function allows to move in the forward/Back direction of the dragged Entity  
pub fn transform_axis(
    drag: Trigger<Pointer<Drag>>,
    q_parents: Query<&ChildOf>,
    q_transform: Query<&GlobalTransform>,
    mut q_local_transform: Query<&mut Transform>,
    windows: Single<&Window>,
    q_camera: Single<(Entity, &Camera), With<GizmoPickSource>>,
    selection: Res<TransformGizmoResource>,
) {
    // Check if the correct Mouse Button is pressed
    if drag.button != selection.drag_button {
        return;
    }
    let (camera_entity, camera) = *q_camera;

    let handle_entity = drag.target();

    let parent_entity = q_parents.get(handle_entity).unwrap().parent();

    let gizmo_transform = q_transform.get(handle_entity).unwrap();

    let camera_transform = q_transform.get(camera_entity).unwrap();

    let direction = gizmo_transform.up();
    let direction_plane = gizmo_transform.forward();

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the Handle plane.
    let Some(distance) = ray.intersect_plane(
        gizmo_transform.translation(),
        InfinitePlane3d::new(direction_plane),
    ) else {
        return;
    };

    let point = ray.get_point(distance);

    // Get the Point before the Drag
    let Ok(ray_delta) = camera.viewport_to_world(camera_transform, cursor_position - drag.delta)
    else {
        return;
    };

    // Calculate if and where the ray is hitting the Handle plane.
    let Some(distance_delta) = ray_delta.intersect_plane(
        gizmo_transform.translation(),
        InfinitePlane3d::new(direction_plane),
    ) else {
        return;
    };
    let point_delta = ray_delta.get_point(distance_delta);

    // Calculate the drag in the correct direction
    let delta_vector = point - point_delta;
    // Calculate the Effect of the mouse movement in the direction of the Handle
    let result = delta_vector.project_onto(*direction);

    // Set the transformation
    let mut parent_transform_local = q_local_transform.get_mut(parent_entity).unwrap();
    parent_transform_local.translation += result;

    // Set the Transformation to the connected Object
    if let Some(sel_entity) = selection.entity {
        let mut selection_transform_local = q_local_transform.get_mut(sel_entity).unwrap();
        selection_transform_local.translation += result;
    }
}

/// This Observer Function allows to move in the two directions on the Plane created from Forward and Right of the dragged Entity  
pub fn transform_plane(
    drag: Trigger<Pointer<Drag>>,
    q_parents: Query<&ChildOf>,
    q_transform: Query<&GlobalTransform>,
    mut q_local_transform: Query<&mut Transform>,
    windows: Single<&Window>,
    q_camera: Single<(Entity, &Camera), With<GizmoPickSource>>,
    selection: Res<TransformGizmoResource>,
) {
    // Check if the correct Mouse Button is pressed
    if drag.button != selection.drag_button {
        return;
    }

    let (camera_entity, camera) = *q_camera;

    let handle_entity = drag.target();

    let parent_entity = q_parents.get(handle_entity).unwrap().parent();

    let gizmo_transform = q_transform.get(handle_entity).unwrap();

    let camera_transform = q_transform.get(camera_entity).unwrap();

    let axis_1 = Vec3::from(gizmo_transform.forward());
    let axis_2 = Vec3::from(gizmo_transform.right());

    let direction_plane = gizmo_transform.up();

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the Handle plane.
    let Some(distance) = ray.intersect_plane(
        gizmo_transform.translation(),
        InfinitePlane3d::new(direction_plane),
    ) else {
        return;
    };

    let point = ray.get_point(distance);

    // Get the Point before the Drag
    let Ok(ray_delta) = camera.viewport_to_world(camera_transform, cursor_position - drag.delta)
    else {
        return;
    };

    // Calculate if and where the ray is hitting the Handle plane.
    let Some(distance_delta) = ray_delta.intersect_plane(
        gizmo_transform.translation(),
        InfinitePlane3d::new(direction_plane),
    ) else {
        return;
    };
    let point_delta = ray_delta.get_point(distance_delta);

    // Calculate the drag in the correct direction
    let delta_vector = point - point_delta;
    // Calculate the Effect of the mouse movement in the direction of the Handle
    let result = delta_vector.project_onto(axis_1) + delta_vector.project_onto(axis_2);

    // Set the transformation
    let mut parent_transform_local = q_local_transform.get_mut(parent_entity).unwrap();
    parent_transform_local.translation += result;

    // Set the Transformation to the connected Object
    if let Some(sel_entity) = selection.entity {
        let mut selection_transform_local = q_local_transform.get_mut(sel_entity).unwrap();
        selection_transform_local.translation += result;
    }
}

/// This Observer Function allows to move in the two directions on the Plane created from the Camera View of the dragged Entity
pub fn transform_camera_plane(
    drag: Trigger<Pointer<Drag>>,
    q_parents: Query<&ChildOf>,
    q_transform: Query<&GlobalTransform>,
    mut q_local_transform: Query<&mut Transform>,
    windows: Single<&Window>,
    q_camera: Single<(Entity, &Camera), With<GizmoPickSource>>,
    selection: Res<TransformGizmoResource>,
) {
    // Check if the correct Mouse Button is pressed
    if drag.button != selection.drag_button {
        return;
    }
    let (camera_entity, camera) = *q_camera;

    let handle_entity = drag.target();

    let parent_entity = q_parents.get(handle_entity).unwrap().parent();

    let gizmo_transform = q_transform.get(handle_entity).unwrap();

    let camera_transform = q_transform.get(camera_entity).unwrap();

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let direction_plane = camera_transform.back();
    // Calculate if and where the ray is hitting the Handle plane.
    let Some(distance) = ray.intersect_plane(
        gizmo_transform.translation(),
        InfinitePlane3d::new(direction_plane),
    ) else {
        return;
    };

    let point = ray.get_point(distance);

    // Get the Point before the Drag
    let Ok(ray_delta) = camera.viewport_to_world(camera_transform, cursor_position - drag.delta)
    else {
        return;
    };

    // Calculate if and where the ray is hitting the Handle plane.
    let Some(distance_delta) = ray_delta.intersect_plane(
        gizmo_transform.translation(),
        InfinitePlane3d::new(direction_plane),
    ) else {
        return;
    };
    let point_delta = ray_delta.get_point(distance_delta);

    // Calculate the drag in the correct direction

    let delta_vector = point - point_delta;

    let axis_1 = Vec3::from(camera_transform.up());
    let axis_2 = Vec3::from(camera_transform.right());

    // Calculate the Effect of the mouse movement in the direction of the Handle
    let result = delta_vector.project_onto(axis_1) + delta_vector.project_onto(axis_2);

    // Set the transformation
    let mut parent_transform_local = q_local_transform.get_mut(parent_entity).unwrap();
    parent_transform_local.translation += result;

    // Set the Transformation to the connected Object
    if let Some(sel_entity) = selection.entity {
        let mut selection_transform_local = q_local_transform.get_mut(sel_entity).unwrap();
        selection_transform_local.translation += result;
    }
}

/// This Observer Function allows to rotate the dragged Entity
pub fn transform_rotation(
    drag: Trigger<Pointer<Drag>>,
    q_parents: Query<&ChildOf>,
    q_transform: Query<&GlobalTransform>,
    mut q_local_transform: Query<&mut Transform>,
    windows: Single<&Window>,
    q_camera: Single<(Entity, &Camera), With<GizmoPickSource>>,
    selection: Res<TransformGizmoResource>,
) {
    // Check if the correct Mouse Button is pressed
    if drag.button != selection.drag_button {
        return;
    }

    let (camera_entity, camera) = *q_camera;

    let handle_entity = drag.target();

    let parent_entity = q_parents.get(handle_entity).unwrap().parent();

    let gizmo_transform = q_transform.get(handle_entity).unwrap();

    let camera_transform = q_transform.get(camera_entity).unwrap();

    let axis_1 = Vec3::from(gizmo_transform.up());

    let direction_plane = gizmo_transform.up();

    let Some(cursor_position) = windows.cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the Handle plane.
    let Some(distance) = ray.intersect_plane(
        gizmo_transform.translation(),
        InfinitePlane3d::new(direction_plane),
    ) else {
        return;
    };

    let point = ray.get_point(distance);

    // Get the Point before the Drag
    let Ok(ray_delta) = camera.viewport_to_world(camera_transform, cursor_position - drag.delta)
    else {
        return;
    };

    // Calculate if and where the ray is hitting the Handle plane.
    let Some(distance_delta) = ray_delta.intersect_plane(
        gizmo_transform.translation(),
        InfinitePlane3d::new(direction_plane),
    ) else {
        return;
    };
    let point_delta = ray_delta.get_point(distance_delta);

    // Calculate the drag in the correct direction
    // Calculate the Effect of the mouse movement in the direction of the Handle
    let origin = gizmo_transform.translation();
    let origin_dir = gizmo_transform.back();

    let dir1 = (point - origin).normalize();
    let dir2 = (point_delta - origin).normalize();

    let angle_side = origin_dir.angle_between(dir1);
    let angle_side_2 = origin_dir.angle_between(dir2);

    let angle_diff = angle_side - angle_side_2;

    // Set the transformation
    let mut parent_transform_local = q_local_transform.get_mut(parent_entity).unwrap();
    parent_transform_local.rotate(Quat::from_axis_angle(axis_1, angle_diff));

    // Set the Transformation to the connected Object
    if let Some(sel_entity) = selection.entity {
        let mut selection_transform_local = q_local_transform.get_mut(sel_entity).unwrap();
        selection_transform_local.rotate(Quat::from_axis_angle(axis_1, angle_diff));
    }
}
