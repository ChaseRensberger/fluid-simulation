use bevy::{prelude::*, window::WindowResolution};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

const WINDOW_DIMENSIONS: Vec2 = Vec2::new(1400., 800.);
const BOUNDING_BOX_DIMENSIONS: Vec2 = Vec2::new(WINDOW_DIMENSIONS.x / 2., WINDOW_DIMENSIONS.y / 2.);

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

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    #[inspector(min = 0.0, max = 1000.0)]
    gravity: f32,
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

fn apply_gravity(mut query: Query<&mut Velocity>, time: Res<Time>, config: Res<Configuration>) {
    for mut velocity in &mut query {
        velocity.0 += Vec3::new(0., -1., 0.) * config.gravity * time.delta_secs();
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
        // .add_plugins(WorldInspectorPlugin::new())
        // .init_resource::<Configuration>()
        .insert_resource(Configuration {
            gravity: 0.0,
            ..default()
        })
        .register_type::<Configuration>() // you need to register your type to display it
        .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
        // also works with built-in resources, as long as they are `Reflect`
        .add_plugins(ResourceInspectorPlugin::<Time>::default())
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (apply_velocity, apply_gravity, resolve_collisions).chain(),
        )
        .run();
}
