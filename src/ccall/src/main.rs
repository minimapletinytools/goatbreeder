mod goat;

use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, Mesh, VertexAttributeValues},
    pipeline::PrimitiveTopology,
};
use goat::*;

fn main() -> Result<(), String> {
    rs_goat_init();

    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_goats.system())
        .run();

    rs_goat_exit();

    Ok(())
}

fn add_goats(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("generating goat");
    let (p, n, tc, f) = Goat::random().mesh().buffers();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let pos_vec = p
        .to_vec()
        .chunks(3)
        .into_iter()
        .map(|x| [x[0] as f32, x[1] as f32, x[2] as f32])
        .collect::<Vec<_>>();

    let norm_vec = n
        .to_vec()
        .chunks(3)
        .into_iter()
        .map(|x| [x[0] as f32, x[1] as f32, x[2] as f32])
        .collect::<Vec<_>>();

    let text_vec = tc
        .to_vec()
        .chunks(2)
        .into_iter()
        .map(|x| [x[0] as f32, x[1] as f32])
        .collect::<Vec<_>>();

    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float3(pos_vec),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float3(norm_vec),
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::Float2(text_vec),
    );
    mesh.set_indices(Some(Indices::U32(f.to_vec())));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    commands
        .spawn(PbrComponents {
            mesh: mesh_handle,
            material: material_handle,
            // transform: Transform::from_translation(Vec3::new(-3.0, 0.0, 0.0)),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 3.0, 10.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}
