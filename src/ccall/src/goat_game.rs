use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::{
    assets::{Handle, Loader, ProgressCounter},
    core::{math::Vector3, Transform},
    prelude::*,
    renderer::{
        camera::{Camera},
        rendy::{
            mesh::{MeshBuilder, Position}
        },
        types::{Mesh, MeshData},
    },
};

use crate::goat::{*};

pub struct GoatGame;
pub struct Model {}

impl Component for Model {
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for GoatGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        initialise_camera(world);
        initialize_model(world);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.0);

    world
        .create_entity()
        .with(Camera::standard_3d(100.0, 100.0))
        .with(transform)
        .build();
}

fn initialize_model(world: &mut World) {
    let asset : Handle<Mesh> = {
        let loader = world.read_resource::<Loader>();
        let mut progress = ProgressCounter::default();
        let mesh_storage = world.read_resource();
        rs_goat_init();
        println!("generating goat");
        let g = Goat::random();
        println!("printing before mesh");
        let m = g.mesh();
        let (v, f) = m.buffers();

        let pos_slice = v
            .to_vec()
            .chunks(3)
            .into_iter()
            .map(|x| Position([x[0] as f32, x[1] as f32, x[2] as f32]))
            .collect::<Vec<_>>();

        let mesh_data : MeshData = MeshBuilder::new()
        .with_vertices(pos_slice)
        .with_indices(f.to_vec())
        .into();

        loader.load_from_data(
            mesh_data,
            &mut progress,
            &mesh_storage,
        )
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.0);
    transform.set_scale(Vector3::new(12.0, 12.0, 12.0));

    world
        .create_entity()
        .with(asset)
        .with(transform)
        .build();
}
