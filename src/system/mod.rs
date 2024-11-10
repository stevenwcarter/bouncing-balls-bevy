use crate::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    commands
        .spawn(Collider::cuboid(640.0, 15.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -360.0, 0.0)));
    commands
        .spawn(Collider::cuboid(640.0, 15.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 360.0, 0.0)));
    commands
        .spawn(Collider::cuboid(15.0, 360.0))
        .insert(TransformBundle::from(Transform::from_xyz(-640.0, 0.0, 0.0)));
    commands
        .spawn(Collider::cuboid(15.0, 360.0))
        .insert(TransformBundle::from(Transform::from_xyz(640.0, 0.0, 0.0)));
}

pub fn track_cursor_position(
    mut mycoords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();

    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }
}

pub fn watch_mouse_buttons(
    mut adding_sand: ResMut<AddingSand>,
    mut char_input_events: EventReader<MouseButtonInput>,
) {
    for event in char_input_events.read() {
        match (event.button, event.state) {
            (MouseButton::Left, bevy::input::ButtonState::Pressed) => {
                adding_sand.0 = true;
            }
            (MouseButton::Left, bevy::input::ButtonState::Released) => {
                adding_sand.0 = false;
            }
            _ => {}
        }
    }
}
