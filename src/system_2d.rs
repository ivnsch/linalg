use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn run_2d() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_scene, setup_light))
        .add_systems(
            Update,
            (
                setup_axes,
                setup_line,
                listen_received_character_events,
                button_system,
                draw_arrows,
            ),
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
                    font: font.clone(),
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

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(50.0),
                // align_items: AlignItems::Center,
                // justify_content: JustifyContent::Center,
                ..default()
            },
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
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Add",
                        TextStyle {
                            font,
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
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

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    edit_text: Query<&mut Text, With<TextInput>>,
) {
    match interaction_query.get_single_mut() {
        Ok((interaction, mut color, mut border_color, _)) => {
            match *interaction {
                Interaction::Pressed => {
                    println!("pressed add!");
                    match edit_text.get_single() {
                        Ok(text) => {
                            match process_vector_str(&text.sections[0].value) {
                                Ok(vec) => {
                                    commands.spawn(MyArrow { target: vec });
                                }
                                Err(err) => println!("error: {}", err), // TODO error handling
                            }
                        }
                        Err(err) => panic!("error: {}", err),
                    }
                    *color = PRESSED_BUTTON.into();
                    border_color.0 = Color::RED;
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                    border_color.0 = Color::WHITE;
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                    border_color.0 = Color::BLACK;
                }
            }
        }
        Err(_) => {
            // TODO use iter_mut()
        }
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
