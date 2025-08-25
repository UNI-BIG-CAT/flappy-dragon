use crate::mode::GameMode;
use bracket_lib::prelude::*;
pub struct State {
    mode: GameMode,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // ctx 是游戏上下文，可以用来绘制游戏界面
        // ctx.cls(); // 清空显示窗口
        // ctx.print(1, 1, "Hello, world!");
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: GameMode::Menu,
        }
    }

    pub fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    pub fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Playing;

        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "Press P to Play Again");
        ctx.print_centered(9, "Press Q to Quit");

        self.dead(ctx);
    }

    pub fn main_menu(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Menu;

        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "Press P to Play");
        ctx.print_centered(9, "Press Q to Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => {
                    self.play(ctx);
                }
                VirtualKeyCode::Q => {
                    ctx.quitting = true;
                }
                _ => {}
            }
        }
    }

    pub fn dead(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::End;

        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "Press P to Play Again");
        ctx.print_centered(9, "Press Q to Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => {
                    self.restart();
                }
                VirtualKeyCode::Q => {
                    ctx.quitting = true;
                }
                _ => {}
            }
        }
    }
}
