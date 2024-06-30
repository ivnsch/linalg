//! This example demonstrates Bevy's immediate mode drawing API intended for visual debugging.

mod alg;
mod camera_controller;
mod curves_2d;
mod curves_3d;
mod functions;
mod gui;
mod lines_2d;
mod rotator;
mod scratchpad_3d;
mod system_2d;
mod system_3d;
mod vectors_2d_system;

use bevy::app::App;
#[allow(unused_imports)]
use curves_3d::add_curves_3d_system;
use lines_2d::add_lines_2d_system;
#[allow(unused_imports)]
use scratchpad_3d::add_3d_scratch;
#[allow(unused_imports)]
use system_2d::add_2d_space;
#[allow(unused_imports)]
use system_3d::add_3d_space;
#[allow(unused_imports)]
use vectors_2d_system::add_vectors_2d_system;

fn main() {
    let app = &mut App::new();
    create_2d(app);
    // create_3d(app);
    app.run();
}

#[allow(dead_code)]
fn create_2d(app: &mut App) {
    add_2d_space(app);
    // add_vectors_2d_system(app);
    add_lines_2d_system(app);
    // add_curves_2d_system(app);
}

#[allow(dead_code)]
fn create_3d(app: &mut App) {
    add_3d_space(app);
    // add_3d_scratch(app);
    add_curves_3d_system(app);
}
