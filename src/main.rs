use bevy::{prelude::*, window::WindowResolution};

const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 800.;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn draw_circle(
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Position { x, y },
        Mesh2d(meshes.add(Circle::new(size))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(x, y, 0.),
    ));
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    draw_circle(
        0.,
        0.,
        40.,
        Color::srgb(0., 0., 255.),
        commands,
        meshes,
        materials,
    );
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}
