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
            game_state.eight_count -= 1;

            if game_state.eight_count < -10 {
                game_state.eight_count = 0;
                game_state.hovering_index -= 1;
                println!("going left---------");
            }

            if game_state.hovering_index < 0 {
                game_state.hovering_index = GOAT_NUMBER as i32 - 1;
            }
        } else if movement_right {
            game_state.eight_count += 1;

            if game_state.eight_count > 10 {
                game_state.eight_count = 0;
                game_state.hovering_index += 1;
                println!("going right---------");
            }
            if game_state.hovering_index > GOAT_NUMBER as i32 - 1 {
                game_state.hovering_index = 0;
            }
        }

        for (goat, transform) in (&mut goats, &mut transforms).join() {
            let selected = input.action_is_down("select").unwrap_or(false);

            if !goat.hovering && game_state.hovering_index == goat.index as i32 {
                println!("making goat hover with index {}", goat.index);
                goat.hovering = true;
                transform.set_translation_y(0.0);
            } else if goat.hovering && game_state.hovering_index != goat.index as i32 {
                println!("making goat stop hovering with index {}", goat.index);
                goat.hovering = false;
                transform.set_translation_y(GOAT_Y);
            }

            if goat.selected {
                selected_count += 1
            };

            if selected_count >= 2 {
                // todo win condition stuff
                println!("I Really Should Breed These Goats!");
            } else if selected && !goat.selected && goat.hovering {
                println!("selecting on goat index {} with {} selected", goat.index, selected_count);
                transform.set_scale(Vector3::new(1.5, 1.5, 1.5));
                goat.selected = true;
            } else if selected && goat.selected && goat.hovering {
                // double select so undo
                println!("undo selecting on goat index {} with {} selected", goat.index, selected_count);
                transform.set_scale(Vector3::new(1.0, 1.0, 1.0));
                goat.selected = false;
            }
        }
    }
}
