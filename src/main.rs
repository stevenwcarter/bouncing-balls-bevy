use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Ball;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, spawn_balls))
        .add_systems(Update, (bounce, adjust_gravity))
        // .add_systems(Startup, setup)
        // .add_systems(Update, move_ferris)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for i in 0..20 {
        let position = Vec2::new(rng.gen_range(-30.0..30.0), rng.gen_range(-30.0..30.0));
        let velocity = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));

        let radius = rng.gen_range(10.0..80.0);
        let color = Color::hsl(360. * i as f32 / 20.0, 0.95, 0.7);
        let shape = Mesh2dHandle(meshes.add(Circle { radius }));

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(color),
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            })
            // .spawn(SpriteBundle {
            //     texture: texture_handle.clone(),
            //     transform: Transform::from_translation(position.extend(0.0)),
            //     ..default()
            // })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(radius))
            .insert(GravityScale(0.0))
            .insert(ColliderMassProperties::Density(1.0))
            .insert(Velocity {
                linvel: velocity,
                angvel: 0.0,
            })
            .insert(Restitution::coefficient(0.9))
            .insert(Ball);
    }

    commands
        .spawn(Collider::cuboid(600.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -600.0, 0.0)));
    commands
        .spawn(Collider::cuboid(600.0, 10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 600.0, 0.0)));
    commands
        .spawn(Collider::cuboid(10.0, 600.0))
        .insert(TransformBundle::from(Transform::from_xyz(-600.0, 0.0, 0.0)));
    commands
        .spawn(Collider::cuboid(10.0, 600.0))
        .insert(TransformBundle::from(Transform::from_xyz(600.0, 0.0, 0.0)));
}

fn bounce(mut query: Query<(&mut Velocity, &Transform), With<Ball>>) {
    for (mut velocity, transform) in query.iter_mut() {
        let pos = transform.translation;
        // let screen_bounds = 300.0;

        // if pos.x > screen_bounds || pos.x < -screen_bounds {
        //     velocity.linvel.x *= -1.0;
        // }
        // if pos.y > screen_bounds || pos.y < -screen_bounds {
        //     velocity.linvel.y *= -1.0;
        // }
    }
}

fn adjust_gravity(
    mut query: Query<&mut GravityScale, With<Ball>>,
    mut char_input_events: EventReader<KeyboardInput>,
) {
    let mut gravity_scale = None;
    for event in char_input_events.read() {
        match event.key_code {
            KeyCode::KeyU => {
                gravity_scale = Some(-0.5);
            }
            KeyCode::KeyG => {
                gravity_scale = Some(0.5);
            }
            KeyCode::Space => {
                gravity_scale = Some(0.0);
            }
            _ => {}
        }
    }

    if let Some(gravity_scale) = gravity_scale {
        for mut gravity in &mut query {
            gravity.0 = gravity_scale;
        }
    }
}
