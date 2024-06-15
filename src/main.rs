//! Create a custom material to draw basic lines in 3D

mod camera_controller;
mod rotator;
use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{MeshVertexBufferLayout, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::{
            AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};
use camera_controller::{CameraController, CameraControllerPlugin};
use rotator::{Rotator, RotatorPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<LineMaterial>::default(),
            CameraControllerPlugin,
            RotatorPlugin,
        ))
        .add_systems(
            Startup,
            (setup_axes, setup_plane, setup_camera, setup_light),
        )
        .run();
}

fn setup_axes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    let size = 2.0;
    // x axis
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(LineList {
            lines: vec![(Vec3::new(-size, 0.0, 0.0), Vec3::new(size, 0.0, 0.0))],
        }),
        material: materials.add(LineMaterial {
            color: Color::GREEN,
        }),
        ..default()
    });
    // y axis
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(LineList {
            lines: vec![(Vec3::new(0.0, -size, 0.0), Vec3::new(0.0, size, 0.0))],
        }),
        material: materials.add(LineMaterial { color: Color::RED }),
        ..default()
    });
    // z axis
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(LineList {
            lines: vec![(Vec3::new(0.0, 0.0, -size), Vec3::new(0.0, 0.0, size))],
        }),
        material: materials.add(LineMaterial { color: Color::BLUE }),
        ..default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
        Rotator::default(),
    ));
}

fn setup_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}

fn setup_plane(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane = Plane3d {
        // normal: Direction3d::Z,
        normal: Direction3d::new(Vec3 {
            x: 1.0,
            y: 1.0,
            z: -1.0,
        })
        .unwrap(),
    };
    draw_plane(plane, commands, meshes, materials);
}

fn draw_plane(
    plane: Plane3d,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(plane.mesh().size(2.0, 2.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });
}

#[derive(Asset, TypePath, Default, AsBindGroup, Debug, Clone)]
struct LineMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/line_material.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // This is the important part to tell bevy to render this material as a line between vertices
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}

/// A list of lines with a start and end position
#[derive(Debug, Clone)]
struct LineList {
    lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();

        Mesh::new(
            // This tells wgpu that the positions are list of lines
            // where every pair is a start and end point
            PrimitiveTopology::LineList,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the vertices positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
struct LineStrip {
    points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        Mesh::new(
            // This tells wgpu that the positions are a list of points
            // where a line will be drawn between each consecutive point
            PrimitiveTopology::LineStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        // Add the point positions as an attribute
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, line.points)
    }
}
