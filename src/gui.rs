use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// text input confirmed via add button
#[derive(Component, Default)]
pub struct GuiInput {
    pub text: String,
}

/// marker component for text input
/// needs to be public to add the component in main, maybe I restructure this later
#[derive(Component, Default)]
pub struct TextInput;

pub fn setup_gui(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn button_system(
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
        Ok((interaction, mut color, mut border_color, _)) => match *interaction {
            Interaction::Pressed => {
                println!("pressed add!");
                match edit_text.get_single() {
                    Ok(text) => {
                        commands.spawn(GuiInput {
                            text: text.sections[0].value.clone(),
                        });
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
        },
        Err(_) => {
            // TODO use iter_mut()
        }
    }
}

pub fn listen_received_character_events(
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
