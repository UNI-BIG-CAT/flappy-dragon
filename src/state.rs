use crate::constants::{FRAME_DURATION, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::mode::GameMode;
use crate::obstacle::Obstacle;
use crate::player::Player;
use bracket_lib::prelude::*;
pub struct State {
    player: Player,
    obstacle: Obstacle,
    frame_time: f32,
    mode: GameMode,
    score: i32,
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
            player: Player::new(15, 25),
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            frame_time: 0.0,
            mode: GameMode::Menu,
            score: 0,
        }
    }

    pub fn restart(&mut self) {
        self.player = Player::new(15, 25);
        self.frame_time = 0.0;
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
    }

    pub fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Playing;

        ctx.cls_bg(NAVY_BLUE);

        ctx.print(1, 1, "Press SPACE to flap");
        ctx.print(1, 0, &format!("Score: {}", self.score));

        self.obstacle.render(ctx);

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
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
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(7, &format!("You earned {} points", self.score));
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
