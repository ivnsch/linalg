use bevy::prelude::*;

#[allow(dead_code)]
pub fn add_3d_scratch(app: &mut App) {
    app.add_systems(Startup, (setup_plane, setup_sphere));
}

fn setup_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(2.0, 2.0)),
        material: materials.add(StandardMaterial {
            double_sided: true,
            cull_mode: None,
            base_color: Color::rgb(0.3, 0.5, 0.3),
            ..default()
        }),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn setup_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial { ..default() });

    let shape = meshes.add(Sphere::default().mesh().uv(32, 18));
    commands.spawn((
        PbrBundle {
            mesh: shape,
            material: debug_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Shape,
    ));
}

/// A marker component for our shapes so we can query them separately from other things
#[derive(Component)]
struct Shape;
