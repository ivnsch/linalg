//! This example demonstrates Bevy's immediate mode drawing API intended for visual debugging.

mod alg;
mod camera_controller;
mod rotator;
mod system_2d;
mod system_3d;

use system_2d::run_2d;

fn main() {
    // run_3d()
    run_2d()
}
