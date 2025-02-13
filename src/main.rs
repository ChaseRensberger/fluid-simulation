use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

const WINDOW_DIMENSIONS: Vec2 = Vec2::new(1400., 800.);
const BOUNDING_BOX_THICKNESS: f32 = 5.;
const BOUNDING_BOX_COLOR: Color = Color::srgb(0., 1., 0.);

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
    #[inspector(min = 0.0, max = 100.0)]
    particle_size: f32,
    bounding_box_dimensions: Vec2,
}

enum BoundingBoxLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl BoundingBoxLocation {
    fn position(&self, config: &Res<Configuration>) -> Vec2 {
        match self {
            BoundingBoxLocation::Left => Vec2::new(-config.bounding_box_dimensions.x / 2., 0.),
            BoundingBoxLocation::Right => Vec2::new(config.bounding_box_dimensions.x / 2., 0.),
            BoundingBoxLocation::Bottom => Vec2::new(0., -config.bounding_box_dimensions.y / 2.),
            BoundingBoxLocation::Top => Vec2::new(0., config.bounding_box_dimensions.y / 2.),
        }
    }

    fn size(&self, config: &Res<Configuration>) -> Vec2 {
        match self {
            BoundingBoxLocation::Left | BoundingBoxLocation::Right => Vec2::new(
                BOUNDING_BOX_THICKNESS,
                config.bounding_box_dimensions.y + BOUNDING_BOX_THICKNESS,
            ),

            BoundingBoxLocation::Bottom | BoundingBoxLocation::Top => Vec2::new(
                BOUNDING_BOX_THICKNESS + config.bounding_box_dimensions.x,
                BOUNDING_BOX_THICKNESS,
            ),
        }
    }
}

impl BoundingBoxBundle {
    fn new(location: BoundingBoxLocation, config: &Res<Configuration>) -> BoundingBoxBundle {
        BoundingBoxBundle {
            sprite: Sprite::from_color(BOUNDING_BOX_COLOR, Vec2::ONE),
            transform: Transform {
                translation: location.position(config).extend(0.0),
                scale: location.size(config).extend(1.0),
                ..default()
            },
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<Configuration>,
) {
    commands.spawn(Camera2d);

    // Particle
    commands.spawn((
        Particle,
        Velocity(Vec3::ZERO),
        Mesh2d(meshes.add(Circle::new(config.particle_size))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 255.))),
        Transform::from_xyz(0., 0., 0.),
    ));

    // Bounding Box
    commands.spawn(BoundingBoxBundle::new(BoundingBoxLocation::Left, &config));
    commands.spawn(BoundingBoxBundle::new(BoundingBoxLocation::Right, &config));
    commands.spawn(BoundingBoxBundle::new(BoundingBoxLocation::Bottom, &config));
    commands.spawn(BoundingBoxBundle::new(BoundingBoxLocation::Top, &config));
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

fn resolve_collisions(
    mut query: Query<(&mut Transform, &mut Velocity)>,
    config: Res<Configuration>,
) {
    for (transform, mut velocity) in &mut query {
        if transform.translation.y < -((WINDOW_DIMENSIONS.y / 2.) - config.particle_size / 2.) {
            velocity.0.y *= -1.;
        }
    }
}

fn update_particle_size(
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut Mesh2d, With<Particle>>,
    config: Res<Configuration>,
) {
    if config.is_changed() {
        for mut mesh in &mut query {
            *mesh = Mesh2d(meshes.add(Circle::new(config.particle_size)));
        }
    }
}

fn update_bounding_box_dimensions(
    mut query: Query<(&mut Transform, &Sprite)>,
    config: Res<Configuration>,
) {
    if config.is_changed() {
        for (mut transform, sprite) in &mut query {
            // Only update bounding boxes (identified by their color)
            if sprite.color == BOUNDING_BOX_COLOR {
                // Determine which wall this is based on its current position
                let location = if transform.translation.x < 0. {
                    BoundingBoxLocation::Left
                } else if transform.translation.x > 0. {
                    BoundingBoxLocation::Right
                } else if transform.translation.y < 0. {
                    BoundingBoxLocation::Bottom
                } else {
                    BoundingBoxLocation::Top
                };

                // Update position and scale
                transform.translation = location.position(&config).extend(0.0);
                transform.scale = location.size(&config).extend(1.0);
            }
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
        .init_resource::<Configuration>()
        .insert_resource(Configuration {
            gravity: 0.0,
            particle_size: 10.,
            bounding_box_dimensions: Vec2::new(WINDOW_DIMENSIONS.x / 2., WINDOW_DIMENSIONS.y / 2.),
            ..default()
        })
        .register_type::<Configuration>()
        .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_particle_size, update_bounding_box_dimensions).chain(),
        )
        .add_systems(
            FixedUpdate,
            (apply_velocity, apply_gravity, resolve_collisions).chain(),
        )
        .run();
}
