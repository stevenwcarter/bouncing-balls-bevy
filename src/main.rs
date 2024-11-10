use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use sand_sim::*;

fn main() {
    App::new()
        .init_resource::<MyWorldCoords>()
        .init_resource::<AddingSand>()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            (watch_mouse_buttons, track_cursor_position, spawn_sand),
        )
        .run();
}
