use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::utils::scene::BasicScenePrefab;
use amethyst::{
    assets::{Handle, Loader, PrefabLoader, ProgressCounter, RonFormat},
    core::Transform,
    prelude::*,
    renderer::{
        camera::Camera,
        loaders,
        mtl::{Material, MaterialDefaults},
        palette::rgb::LinSrgba,
        rendy::mesh::{MeshBuilder, Normal, PosNormTex, Position, TexCoord},
        types::{Mesh, MeshData},
    },
};

use crate::goat::*;

pub type MyPrefabData = BasicScenePrefab<Vec<PosNormTex>>;

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
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    world
        .create_entity()
        .with(Camera::standard_3d(100.0, 100.0))
        .with(transform)
        .build();
}

fn initialize_model(world: &mut World) {
    let mesh: Handle<Mesh> = {
        let loader = world.read_resource::<Loader>();
        let mut progress = ProgressCounter::default();
        let mesh_storage = world.read_resource();
        println!("generating goat");
        let g = Goat::random();
        println!("printing before mesh");
        let m = g.mesh();
        let (v, f) = m.buffers();

        let pos_vec = v
            .to_vec()
            .chunks(3)
            .into_iter()
            .map(|x| Position([x[0] as f32, x[1] as f32, x[2] as f32]))
            .collect::<Vec<_>>();

        let norm_vec = v
            .to_vec()
            .chunks(3)
            .into_iter()
            .map(|x| Normal([x[0] as f32, x[1] as f32, x[2] as f32]))
            .collect::<Vec<_>>();

        let text_vec = v
            .to_vec()
            .chunks(3)
            .into_iter()
            .map(|_x| TexCoord([0.0, 0.0]))
            .collect::<Vec<_>>();

        let mesh_data: MeshData = MeshBuilder::new()
            .with_indices(f.to_vec())
            .with_vertices(pos_vec)
            .with_vertices(norm_vec)
            .with_vertices(text_vec)
            .into();

        loader.load_from_data(mesh_data, &mut progress, &mesh_storage)
    };

    let default_mat = world.read_resource::<MaterialDefaults>().0.clone();

    let albedo = {
        let loader = world.read_resource::<Loader>();
        let mut progress = ProgressCounter::default();
        let mesh_storage = world.read_resource();
        loader.load_from_data(
            loaders::load_from_linear_rgba(LinSrgba::new(0.0, 1.0, 1.0, 1.0)).into(),
            &mut progress,
            &mesh_storage,
        )
    };

    let mat: Handle<Material> = {
        let loader = world.read_resource::<Loader>();
        let mut progress = ProgressCounter::default();
        let mesh_storage = world.read_resource();
        loader.load_from_data(
            Material {
                albedo,
                ..default_mat.clone()
            },
            &mut progress,
            &mesh_storage,
        )
    };

    let prefab_handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        loader.load("resources/prefab.ron", RonFormat, ())
    });

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.0);

    world.create_entity().with(prefab_handle).build();
    world
        .create_entity()
        .with(mesh)
        .with(mat)
        .with(transform)
        .build();
}
