use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct MyWorldCoords(pub Vec2);

#[derive(Resource, Default, Debug)]
pub struct AddingSand(pub bool);
