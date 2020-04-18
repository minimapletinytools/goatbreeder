use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::utils::scene::BasicScenePrefab;
use amethyst::{
    assets::{Handle, Loader, PrefabLoader, ProgressCounter, RonFormat},
    core::{math::Vector3, Transform},
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

pub const GOAT_NUMBER: u32 = 6;
pub const GOAT_Y: f32 = 2.0;

pub type MyPrefabData = BasicScenePrefab<Vec<PosNormTex>>;

pub struct GoatGame;
pub struct GoatStruct {
    pub index: u32,
    pub hovering: bool,
    pub selected: bool,
}

#[derive(Default)]
pub struct GameState {
    pub hovering_index: i32,
    // due to how fast actions are input there will be 8 inputs
    // needed to trigger a switch per index
    pub eight_count: i32,
}

impl GoatStruct {
    fn new(input_index: u32) -> GoatStruct {
        GoatStruct {
            index: input_index,
            hovering: false,
            selected: false,
        }
    }
}

impl GameState {
    fn new() -> GameState {
        GameState {
            hovering_index: 0,
            eight_count: 0,
        }
    }
}

impl Component for GoatStruct {
    type Storage = DenseVecStorage<Self>;
}

impl Component for GameState {
    type Storage = DenseVecStorage<Self>;
}

impl SimpleState for GoatGame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        initialise_camera(world);
        initialise_game_state(world);
        initialize_prefab(world);
        initialize_goats(world);
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(3.5, 0.0, 7.0);

    world
        .create_entity()
        .with(Camera::standard_3d(100.0, 100.0))
        .with(transform)
        .build();
}

fn initialise_game_state(world: &mut World) {
    world.insert(GameState::new());
}

fn initialize_prefab(world: &mut World) {
    let prefab_handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        loader.load("resources/prefab.ron", RonFormat, ())
    });

    world.create_entity().with(prefab_handle).build();
}

fn initialize_goats(world: &mut World) {
    for iter in 0..GOAT_NUMBER {
        let (mesh, mat) = make_mesh_and_mat(world);
        let mut transform = Transform::default();
        transform.set_translation_xyz(1.3 * iter as f32, GOAT_Y, 0.0);
        transform.set_scale(Vector3::new(1.0, 1.0, 1.0));

        world
            .create_entity()
            .with(mesh.clone())
            .with(mat)
            .with(GoatStruct::new(iter))
            .with(transform)
            .build();
    }
}

pub fn make_mesh_and_mat(world: &mut World) -> (Handle<Mesh>, Handle<Material>) {
    let mesh: Handle<Mesh> = {
        let loader = world.read_resource::<Loader>();
        let mut progress = ProgressCounter::default();
        let mesh_storage = world.read_resource();
        println!("generating goat");
        let g = Goat::random();
        println!("printing before mesh");
        let m = g.mesh();
        let (p, n, tc, f) = m.buffers();
        let _ = write_obj_from_buffers(p, n, tc, f);

        let pos_vec = p
            .to_vec()
            .chunks(3)
            .into_iter()
            .map(|x| Position([x[0] as f32, x[1] as f32, x[2] as f32]))
            .collect::<Vec<_>>();

        let norm_vec = n
            .to_vec()
            .chunks(3)
            .into_iter()
            .map(|x| Normal([x[0] as f32, x[1] as f32, x[2] as f32]))
            .collect::<Vec<_>>();

        let text_vec = tc
            .to_vec()
            .chunks(2)
            .into_iter()
            .map(|x| TexCoord([x[0] as f32, x[1] as f32]))
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
            loaders::load_from_linear_rgba(LinSrgba::new(1.0, 1.0, 1.0, 0.5)).into(),
            &mut progress,
            &mesh_storage,
        )
    };

    let metallic_roughness = {
        let loader = world.read_resource::<Loader>();
        let mut progress = ProgressCounter::default();
        let mesh_storage = world.read_resource();
        let roughness = 1f32 / 4.0f32;
        let metallic = 1f32 / 4.0f32;
        loader.load_from_data(
            loaders::load_from_linear_rgba(LinSrgba::new(0.0, roughness, metallic, 0.0)).into(),
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
                albedo: albedo.clone(),
                metallic_roughness,
                ..default_mat.clone()
            },
            &mut progress,
            &mesh_storage,
        )
    };

    (mesh, mat)
}
