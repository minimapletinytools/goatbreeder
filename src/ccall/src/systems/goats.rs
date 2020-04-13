use crate::goat_game::{GameState, GoatStruct, GOAT_NUMBER, GOAT_Y};

use amethyst::{
    core::{math::Vector3, Transform},
    ecs::{Join, Read, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct GoatSystem;

impl<'s> System<'s> for GoatSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, GoatStruct>,
        Write<'s, GameState>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, mut goats, mut game_state, input): Self::SystemData) {
        let movement_left = input.action_is_down("left").unwrap_or(false);
        let movement_right = input.action_is_down("right").unwrap_or(false);
        let mut selected_count = 0;

        if movement_left {
            game_state.hovering_index -= 1;
            if game_state.hovering_index < 0 {
                game_state.hovering_index = GOAT_NUMBER as i32 - 1
            }
            // println!("game_state.hovering_index-{}", game_state.hovering_index);
            println!("going left---------");
        } else if movement_right {
            game_state.hovering_index += 1;
            if game_state.hovering_index > GOAT_NUMBER as i32 - 1 {
                game_state.hovering_index = 0
            }
            // println!("game_state.hovering_index-{}", game_state.hovering_index);
            println!("going right---------");
        }

        for (goat, transform) in (&mut goats, &mut transforms).join() {
            let selected = input.action_is_down("select").unwrap_or(false);

            if !goat.hovering && game_state.hovering_index == goat.index as i32 {
                println!("setting hover {} and goat index {}", game_state.hovering_index, goat.index as i32);
                goat.hovering = true;
                transform.move_down(GOAT_Y);
            } else if goat.hovering && game_state.hovering_index != goat.index as i32 {
                println!(
                    "reseting hover {} and goat index {}",
                    game_state.hovering_index, goat.index as i32
                );
                goat.hovering = false;
                transform.move_up(GOAT_Y);
            } else {
                println!(
                    "nothing matched--------- {} and goat index {}",
                    game_state.hovering_index, goat.index as i32
                );
            }

            if goat.selected { selected_count += 1};

            if selected_count >= 2 {
                // todo win condition stuff
                println!("I Really Should Breed These Goats!");
            } else if selected && goat.selected {
                // double select so undo
                transform.set_scale(Vector3::new(1.0, 1.0, 1.0));
                goat.selected = false;
            } else if selected && !goat.selected && goat.hovering {
                transform.set_scale(Vector3::new(1.5, 1.5, 1.5));
                goat.selected = true;
            }
            // println!("game_state.hovering_index-{}", game_state.hovering_index);
        }
    }
}
