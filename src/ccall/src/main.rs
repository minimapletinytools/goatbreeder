extern crate amethyst;
extern crate libc;

mod goat_game;

mod goat;
mod systems;
use goat::*;

use std::path::Path;
use std::time::Duration;

use crate::goat_game::GoatGame;
use crate::goat_game::MyPrefabData;
use amethyst::{
    assets::PrefabLoaderSystemDesc,
    core::{frame_limiter::FrameRateLimitStrategy, TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
};

#[test]
fn test_goat() {
    {
        println!("generating goat");
        let g = Goat::random();

        //println!("printing mesh");
        let m = g.mesh();
        let (v, n, tc, f) = m.buffers();

        println!("writing mesh to file");
        let _ = write_obj_from_buffers(v, n, tc, f);

        // print buffers ourselves
        //println!("{:?}", v);
        //println!("vectors above-------------");
        //println!("{:?}", f);

        // print using library
        //g.dump();

        println!("breeding");

        // test breeding
        let g1 = Goat::random();
        let g2 = Goat::random();
        let _g3 = breed(&g1, &g2);

        // print to check results
        //g3.dump();

        println!("done");
    }
}

fn main() -> amethyst::Result<()> {
    println!("Hello GOAT!");
    rs_goat_init();

    let bindings_path = Path::new("./resources/input.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?;

    amethyst::start_logger(Default::default());

    let display_config_path = Path::new("./resources/display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<MyPrefabData>::default(),
            "scene_loader",
            &[],
        )
        .with_bundle(input_bundle)?
        .with(systems::GoatSystem, "goats", &["input_system"])
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?)
                .with_plugin(RenderShaded3D::default()),
        )?;

    let mut game = Application::build("./", GoatGame)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            60,
        )
        .build(game_data)?;

    game.run();

    rs_goat_exit();

    Ok(())
}
