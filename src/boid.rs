use crate::{ResourceInspectorPlugin, SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::sprite::*;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, transform};
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use rand::prelude::*;

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        // create resourse of mesh to display boids
        // add systems to create, update, and display boids
        app.insert_resource(MaxBoids { 0: 50 })
            .insert_resource(Speed { 0: 50.0 })
            .register_type::<Speed>()
            .add_plugin(ResourceInspectorPlugin::<Speed>::default())
            .add_startup_system(setup_boids)
            .add_system(move_boids)
            .add_system(wrap_borders_system)
            .add_system(rotate_transform_with_direction);
    }
}

#[derive(Debug, Resource)]
pub struct MaxBoids(u32);

#[derive(Component)]
pub struct Boid;

#[derive(Resource, Debug, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Speed(f32);

#[derive(Component, Default, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Direction(Vec2);

impl From<Direction> for Vec2 {
    fn from(d: Direction) -> Vec2 {
        d.0
    }
}

pub fn spawn_boid(
    commands: &mut Commands,
    position: Vec2,
    direction: Direction,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Boid)
        .insert(direction)
        .insert(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(shape::RegularPolygon::new(1.0, 3).into())),
            material: colors.add(ColorMaterial {
                color: Color::RED,
                texture: None,
            }),
            ..default()
        })
        .insert(Transform::from_xyz(position.x, position.y, 0.0));
}

pub fn move_boids(
    mut boids: Query<(&mut Transform, &Direction), With<Boid>>,
    time: Res<Time>,
    speed: Res<Speed>,
) {
    for (mut position, direction) in boids.iter_mut() {
        let delta = time.delta_seconds();
        let mut change = direction.0.normalize();
        change *= speed.0 * delta;
        position.translation.x += change.x;
        position.translation.y += change.y;
    }
}

pub fn rotate_transform_with_direction(mut boids: Query<(&mut Transform, &Direction), With<Boid>>) {
    for (mut transform, direction) in boids.iter_mut() {
        transform.rotation = Quat::from_rotation_z(-direction.0.x.atan2(direction.0.y));
    }
}

pub fn setup_boids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
    max: Res<MaxBoids>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..max.0 {
        let position = Vec3 {
            x: rng.gen_range(0.0..SCREEN_HEIGHT),
            y: rng.gen_range(0.0..SCREEN_WIDTH),
            z: 0.0,
        };

        let direction = Vec2 {
            x: rng.gen_range(-1.0..=1.0),
            y: rng.gen_range(-1.0..=1.0),
        };
        commands
            .spawn(Boid)
            .insert(Direction { 0: direction })
            .insert(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::RegularPolygon::new(25.0, 3).into())),
                material: colors.add(ColorMaterial {
                    color: Color::RED,
                    texture: None,
                }),
                ..default()
            })
            .insert(Transform::from_translation(position));
    }
}

fn wrap_borders_system(
    mut query: Query<&mut Transform>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = window.iter_mut().next().unwrap();
    let width = window.width();
    let height = window.height();
    for mut transform in query.iter_mut() {
        if transform.translation.x >= width / 2.0 {
            transform.translation.x = -width / 2.0 + 1.0;
        } else if transform.translation.x <= -width / 2.0 {
            transform.translation.x = width / 2.0 - 1.0;
        }
        if transform.translation.y >= height / 2.0 {
            transform.translation.y = -height / 2.0 + 1.0;
        } else if transform.translation.y <= -height / 2.0 {
            transform.translation.y = height / 2.0 - 1.0;
        }
    }
}
