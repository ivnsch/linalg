//! Create a custom material to draw basic lines in 3D

mod camera_controller;
mod rotator;
use std::f32::consts::PI;

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::{
        camera::RenderTarget,
        mesh::{MeshVertexBufferLayout, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::{
            AsBindGroup, Extent3d, PolygonMode, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
    },
};
use camera_controller::{CameraController, CameraControllerPlugin};
use rotator::{Rotator, RotatorPlugin};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        MaterialPlugin::<LineMaterial>::default(),
        CameraControllerPlugin,
        RotatorPlugin,
    ));

    app.add_systems(
        Startup,
        (
            setup_axes,
            setup_plane,
            setup_camera,
            setup_light,
            setup_x_axis,
            setup_y_axis,
            setup_z_axis,
        ),
    );

    // PostStartup since we need the cameras to exist
    app.add_systems(PostStartup, setup_text);

    app.run();
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
        material: materials.add(StandardMaterial {
            double_sided: true,
            cull_mode: None,
            base_color: Color::rgb(0.3, 0.5, 0.3),
            ..Default::default()
        }),
        ..default()
    });
}

/// Marker component for header node
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct HeaderNode;

/// Marker component for header text
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct HeaderText;

fn setup_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cameras: Query<(Entity, &Camera)>,
) {
    let active_camera = cameras
        .iter()
        .find_map(|(entity, camera)| camera.is_active.then_some(entity))
        .expect("run condition ensures existence");

    let font_size = 24.0;
    let font: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    let style = TextStyle {
        font,
        font_size,
        color: Color::WHITE,
    };
    let text = [TextSection::new("Primitive: ", style.clone())];

    commands
        .spawn((
            HeaderNode,
            NodeBundle {
                style: Style {
                    justify_self: JustifySelf::Center,
                    // top: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            TargetCamera(active_camera),
        ))
        .with_children(|parent| {
            parent.spawn((HeaderText, TextBundle::from_sections(text)));
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

// Marks the cube, to which the UI texture is applied.
#[derive(Component)]
struct Cube;

fn generate_axis_label(
    label: &str,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) -> CubeWithMaterial {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // Light
    commands.spawn(DirectionalLightBundle::default());

    let texture_camera = commands
        .spawn(Camera2dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        })
        .id();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    // Cover the whole image
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            },
            TargetCamera(texture_camera),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 320.0,
                    color: Color::BLACK,
                    ..default()
                },
            ));
        });

    let cube_size = 0.05;
    let cube_handle = meshes.add(Cuboid::new(cube_size, cube_size, cube_size));

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,

        ..default()
    });

    CubeWithMaterial {
        cube: cube_handle,
        material: material_handle,
    }
}

fn setup_x_axis(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    images: ResMut<Assets<Image>>,
) {
    let cube = generate_axis_label("x", &mut commands, meshes, materials, images);
    commands.spawn((
        to_pbr_bundle(
            cube,
            Transform::from_xyz(2.0, 0.0, 0.0).with_rotation(Quat::from_rotation_x(-PI)),
        ),
        Cube,
    ));
}

fn setup_y_axis(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    images: ResMut<Assets<Image>>,
) {
    let cube = generate_axis_label("y", &mut commands, meshes, materials, images);
    commands.spawn((
        to_pbr_bundle(
            cube,
            Transform::from_xyz(0.0, 2.0, 0.0).with_rotation(Quat::from_rotation_x(PI / 2.0)),
        ),
        Cube,
    ));
}

fn setup_z_axis(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    images: ResMut<Assets<Image>>,
) {
    let cube = generate_axis_label("z", &mut commands, meshes, materials, images);
    commands.spawn((
        to_pbr_bundle(
            cube,
            Transform::from_xyz(0.0, 0.0, 2.0).with_rotation(Quat::from_rotation_x(-PI)),
        ),
        Cube,
    ));
}

fn to_pbr_bundle(cube: CubeWithMaterial, transform: Transform) -> PbrBundle {
    PbrBundle {
        mesh: cube.cube,
        material: cube.material,
        transform,
        ..default()
    }
}

struct CubeWithMaterial {
    cube: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}
