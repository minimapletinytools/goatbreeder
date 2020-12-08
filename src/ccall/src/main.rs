mod goat;

use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, Mesh, VertexAttributeValues},
    pipeline::PrimitiveTopology,
};
use bevy_mod_picking::*;
use goat::*;
use std::sync::Mutex;

struct SelectedGoatParent {
    maybe_parent: Option<GoatEntity>,
}

struct GoatEntity {
    goat: Mutex<Goat>,
}

fn main() -> Result<(), String> {
    rs_goat_init();

    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        // .add_plugin(DebugPickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_resource(SelectedGoatParent { maybe_parent: None })
        .add_startup_system(init_game)
        .add_system(selection_handler)
        .add_system(rotate_goats)
        .run();

    rs_goat_exit();

    Ok(())
}

fn breed_goats(
    parent1: &Goat,
    parent2: &Goat,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("generating goat");
    for x in -2..3 {
        for y in -1..2 {
            loop {
                let new_goat = breed(parent1, parent2);
                let (p, n, tc, f) = new_goat.mesh().buffers();
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
                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

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
                let faces_vec = f.to_vec();
                // below is a hack to work around bad goat generation causing panics with mod_picking
                if faces_vec.iter().all(|x| *x <= 900_000_000) {
                    mesh.set_indices(Some(Indices::U32(faces_vec)));

                    let mesh_handle = meshes.add(mesh);
                    let material_handle = materials.add(StandardMaterial {
                        albedo: Color::rgb(0.8, 0.7, 0.6),
                        ..Default::default()
                    });
                    let mesh_transform = Vec3::new(2.0 * x as f32, 2.0 * y as f32, 0.0);

                    commands
                        .spawn(PbrBundle {
                            mesh: mesh_handle,
                            material: material_handle,
                            transform: Transform::from_translation(mesh_transform),
                            ..Default::default()
                        })
                        .with(GoatEntity {
                            goat: Mutex::new(new_goat),
                        })
                        .with(PickableMesh::default())
                        .with(InteractableMesh::default())
                        .with(HighlightablePickMesh::default())
                        .with(SelectablePickMesh::default());

                    break;
                } else {
                    eprintln!("there is a bug that seems to be occuring more as you breed");
                    eprintln!("retrying");
                };
            }
        }
    }
}

fn rotate_goats(mut query: Query<(&GoatEntity, &mut Transform)>) {
    for (_goat_entity, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 140.0));
    }
}

fn selection_handler(
    commands: &mut Commands,
    mut selected_goat_parent: ResMut<SelectedGoatParent>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    select_query: Query<(&SelectablePickMesh, &GoatEntity)>,
    entity_query: Query<(Entity, &GoatEntity)>,
) {
    for (selectable, goat_entity) in select_query.iter() {
        if selectable.selected(&Group::default()) {
            match &selected_goat_parent.maybe_parent {
                None => {
                    let usable_goat = goat_entity.goat.lock().unwrap();
                    selected_goat_parent.maybe_parent = Some(GoatEntity {
                        goat: Mutex::new(usable_goat.clone()),
                    });
                    break;
                }
                Some(selected_goat) => {
                    let s_goat = selected_goat.goat.lock().unwrap();
                    let usable_goat = goat_entity.goat.lock().unwrap();
                    // re-init game
                    if s_goat.id != usable_goat.id {
                        for (entity, _goat) in entity_query.iter() {
                            commands.despawn(entity);
                        }
                        breed_goats(&s_goat, &usable_goat, commands, meshes, materials);
                        drop(s_goat);
                        selected_goat_parent.maybe_parent = None;
                        println!("You bred a New Round of Goats!");
                        break;
                    }
                }
            }
        }
    }
}

fn init_game(
    commands: &mut Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    breed_goats(
        &Goat::random(),
        &Goat::random(),
        commands,
        meshes,
        materials,
    );

    commands
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 3.0, 10.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(PickSource::default());
}
