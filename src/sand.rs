use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::*;

pub fn spawn_sand(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    adding_sand: Res<AddingSand>,
    world_pos: Res<MyWorldCoords>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !adding_sand.0 {
        return;
    }

    let mut rng = rand::thread_rng();

    for _ in 0..4 {
        let color = Color::hsl(200.0, 0.95, 0.7);
        let pos_x = rng.gen_range(world_pos.0.x - 10.0..world_pos.0.x + 10.0);
        let pos_y = rng.gen_range(world_pos.0.y - 10.0..world_pos.0.y + 10.0);
        let position = Vec2::new(pos_x, pos_y);
        let velocity = Vec2::ZERO;
        let radius = 5.0;
        let shape = Mesh2dHandle(meshes.add(Circle { radius }));

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(color),
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(radius))
            .insert(GravityScale(1.0))
            .insert(ColliderMassProperties::Density(3.0))
            .insert(Velocity {
                linvel: velocity,
                angvel: 0.0,
            })
            .insert(Restitution::coefficient(0.4))
            .insert(Sand);
    }
}
