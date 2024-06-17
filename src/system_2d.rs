use bevy::prelude::*;

use crate::gui::{button_system, listen_received_character_events, setup_gui, GuiInput};

pub fn run_2d() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_gui, setup_light))
        .add_systems(
            Update,
            (
                setup_axes,
                setup_line,
                listen_received_character_events,
                button_system,
                draw_arrows,
                listen_inputs_from_gui,
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct MyArrow {
    target: Vec2,
}

fn draw_arrows(arrows: Query<&MyArrow>, mut gizmos: Gizmos) {
    for arrow in arrows.iter() {
        gizmos.arrow_2d(Vec2::ZERO, arrow.target, Color::YELLOW);
    }
}

fn process_vector_str(str: &str) -> Result<Vec2, String> {
    let values: Result<Vec<f32>, _> = str.split_whitespace().map(|s| s.parse::<f32>()).collect();
    match values {
        Ok(vec) if vec.len() == 2 => {
            let vec2 = Vec2 {
                x: vec[0],
                y: vec[1],
            };
            Ok(vec2)
        }
        Ok(_) => Err("Input must contain exactly two numbers.".to_owned()),
        Err(e) => Err(format!("Failed to parse input: {}", e)),
    }
}

fn setup_line(mut gizmos: Gizmos) {
    gizmos.line_2d(
        Vec2 { x: -1.0, y: -1.0 },
        Vec2 { x: 2.0, y: 1.5 },
        Color::BLUE,
    );
}

fn setup_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

fn setup_axes(mut gizmos: Gizmos) {
    let size = 300.0;
    let zero = 0.0;
    // x
    gizmos.line_2d(
        Vec2 { x: -size, y: zero },
        Vec2 { x: size, y: zero },
        Color::GREEN,
    );
    // y
    gizmos.line_2d(
        Vec2 { x: zero, y: -size },
        Vec2 { x: zero, y: size },
        Color::RED,
    );
}

fn listen_inputs_from_gui(mut commands: Commands, input: Query<&mut GuiInput>) {
    for input in input.into_iter() {
        match process_vector_str(&input.text) {
            Ok(vec) => {
                commands.spawn(MyArrow { target: vec });
            }
            Err(err) => println!("error: {}", err), // TODO error handling
        }
    }
}
