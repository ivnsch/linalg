use bevy::prelude::*;

pub fn run_2d() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_scene, setup_light))
        .add_systems(
            Update,
            (setup_axes, setup_line, listen_received_character_events),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Add a vector:".to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                ..default()
            },
        ),
        transform: Transform {
            translation: Vec3 {
                x: 400.0,
                y: 320.0,
                z: 0.0,
            },
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        },
        ..default()
    });

    commands.spawn((
        TextInput,
        Text2dBundle {
            text: Text::from_section(
                "".to_string(),
                TextStyle {
                    font,
                    font_size: 20.0,
                    ..default()
                },
            ),
            transform: Transform {
                translation: Vec3 {
                    x: 400.0,
                    y: 290.0,
                    z: 0.0,
                },
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            ..default()
        },
    ));
}

/// marker component for text input
#[derive(Component, Default)]
pub struct TextInput;

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

fn listen_received_character_events(
    mut events: EventReader<ReceivedCharacter>,
    mut edit_text: Query<&mut Text, With<TextInput>>,
) {
    for event in events.read() {
        println!("received text: {:?}", event);
        edit_text.single_mut().sections[0]
            .value
            .push_str(&event.char);
    }
}
