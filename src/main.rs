use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const WINDOW_DIMENSIONS: Vec2 = Vec2::new(1400., 800.);
const BOUNDING_BOX_DIMENSIONS: Vec2 = Vec2::new(WINDOW_DIMENSIONS.x / 2., WINDOW_DIMENSIONS.y / 2.);

const GRAVITY: f32 = 100.;
const PARTICLE_SIZE: f32 = 30.;

#[derive(Component)]
struct Particle;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Bundle)]
struct BoundingBoxBundle {
    sprite: Sprite,
    transform: Transform,
}

enum BoundingBoxLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl BoundingBoxLocation {
    fn position(&self) -> Vec2 {
        match self {
            BoundingBoxLocation::Left => Vec2::new(-BOUNDING_BOX_DIMENSIONS.x / 2., 0.),
            BoundingBoxLocation::Right => Vec2::new(BOUNDING_BOX_DIMENSIONS.x / 2., 0.),
            BoundingBoxLocation::Bottom => Vec2::new(BOUNDING_BOX_DIMENSIONS.y / 2., 0.),
            BoundingBoxLocation::Top => Vec2::new(-BOUNDING_BOX_DIMENSIONS.y / 2., 0.),
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // Particle
    commands.spawn((
        Particle,
        Velocity(Vec3::ZERO),
        Mesh2d(meshes.add(Circle::new(PARTICLE_SIZE))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 255.))),
        Transform::from_xyz(0., 0., 0.),
    ));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * time.delta_secs();
        // println!("{}", transform.translation)
    }
}

fn apply_gravity(mut query: Query<&mut Velocity>, time: Res<Time>) {
    for mut velocity in &mut query {
        velocity.0 += Vec3::new(0., -1., 0.) * GRAVITY * time.delta_secs();
    }
}

fn resolve_collisions(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (transform, mut velocity) in &mut query {
        if transform.translation.y < -((WINDOW_DIMENSIONS.y / 2.) - PARTICLE_SIZE / 2.) {
            velocity.0.y *= -1.;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_DIMENSIONS.x, WINDOW_DIMENSIONS.y),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (apply_velocity, apply_gravity, resolve_collisions).chain(),
        )
        .run();
}
