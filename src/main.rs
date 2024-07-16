//! This example demonstrates Bevy's immediate mode drawing API intended for visual debugging.

mod alg;
mod balance_chem_eq;
mod balance_diet;
mod electrical_network;
mod functions;
mod grid_2d;
mod gui;
mod lines_2d;
mod system_2d;
mod vectors_2d_system;
use bevy::app::App;
use grid_2d::add_grid_2d_system;
#[allow(unused_imports)]
use lines_2d::add_lines_2d_system;
use system_2d::add_2d_axes;
#[allow(unused_imports)]
use system_2d::add_2d_space;
#[allow(unused_imports)]
use vectors_2d_system::add_vectors_2d_system;

fn main() {
    let app = &mut App::new();
    create_2d(app);
    app.run();
}

#[allow(dead_code)]
fn create_2d(app: &mut App) {
    add_2d_space(app);
    // add_vectors_2d_system(app);
    add_grid_2d_system(app);
    // grid completely hiding axes so draw axes on top.
    add_2d_axes(app);
    add_lines_2d_system(app);
}
