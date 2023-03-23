#![allow(unused)]

use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use boid::BoidPlugin;

mod boid;

const SCREEN_WIDTH: f32 = 1000.;
const SCREEN_HEIGHT: f32 = 750.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                title: "Boids".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(BoidPlugin)
        //.add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
