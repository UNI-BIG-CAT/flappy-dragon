mod mode;
mod state;
//
use bracket_lib::prelude::*;
use state::State;
use std::boxed::Box;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new());
    Ok(())
}
