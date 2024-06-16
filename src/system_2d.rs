use bevy::prelude::*;

pub fn run_2d() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_light))
        .add_systems(Update, (setup_axes, setup_line))
        .run();
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

fn setup_camera(mut commands: Commands) {
    let projection = OrthographicProjection {
        scale: 0.008,
        ..Default::default()
    };
    commands.spawn(Camera2dBundle {
        projection,
        ..default()
    });
}

fn setup_axes(mut gizmos: Gizmos) {
    let size = 2.0;
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
