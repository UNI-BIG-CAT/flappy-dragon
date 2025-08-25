#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")] // #![...] 这种 crate 级别属性 只能放在 文件的最开头，在任何 mod、use 之前

mod constants;
mod mode;
mod obstacle;
mod player;
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
    let _ = main_loop(context, State::new());
    Ok(())
}
