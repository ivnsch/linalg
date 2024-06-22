use bevy::prelude::*;

use crate::gui::{button_system, listen_received_character_events, setup_gui, GuiInput};

#[allow(dead_code)]
pub fn add_vectors_2d_system(app: &mut App) {
    app.add_systems(Startup, setup_gui).add_systems(
        Update,
        (
            listen_received_character_events,
            button_system,
            draw_arrows,
            listen_inputs_from_gui,
        ),
    );
}

#[derive(Component)]
struct MyArrow {
    start: Vec2,
    end: Vec2,
}

fn draw_arrows(arrows: Query<&MyArrow>, mut gizmos: Gizmos) {
    for arrow in arrows.iter() {
        gizmos.arrow_2d(arrow.start, arrow.end, Color::YELLOW);
    }
}

fn process_arrow_str(str: &str) -> Result<MyArrow, String> {
    let values: Result<Vec<f32>, _> = str.split_whitespace().map(|s| s.parse::<f32>()).collect();
    match values {
        Ok(vec) if vec.len() == 4 => Ok(MyArrow {
            start: Vec2 {
                x: vec[0],
                y: vec[1],
            },
            end: Vec2 {
                x: vec[2],
                y: vec[3],
            },
        }),
        Ok(_) => Err("Input must contain exactly two numbers.".to_owned()),
        Err(e) => Err(format!("Failed to parse input: {}", e)),
    }
}

fn listen_inputs_from_gui(mut commands: Commands, input: Query<&mut GuiInput>) {
    for input in input.into_iter() {
        match process_arrow_str(&input.text) {
            Ok(arrow) => {
                commands.spawn(arrow);
            }
            Err(err) => println!("error: {}", err), // TODO error handling
        }
    }
}
