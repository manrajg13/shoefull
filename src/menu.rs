use ggez::Context;
use ggez::event::{KeyCode};
use ggez::input::keyboard;
use std::time::Instant;
use crate::states;
use crate::sprites;

pub struct Menu {
    options: [&'static str; 3],
    index: usize,
    w_pressed: bool,
    s_pressed: bool,
    j_pressed: bool,

    pub reset_stats: bool,
    pub now: Instant,
    pub current_option: &'static str,
    pub current_menu_sprite: usize,
    pub current_sit_sprite: usize,
    pub current_tp_sprite: usize,
}

impl Menu {
    pub fn new() -> Self {
        let options = ["play", "controls", "exit"];
        let current_option = "play";
        let index = 0;
        let current_menu_sprite = 1;
        let current_sit_sprite = 0;
        let current_tp_sprite = 0;
        let w_pressed = false;
        let s_pressed = false;
        let j_pressed = false;
        let reset_stats = false;
        let now = Instant::now();

        Self { 
            current_menu_sprite, current_option, options, index,
            current_sit_sprite, current_tp_sprite, now,
            w_pressed, s_pressed, j_pressed, reset_stats,
        }
    }

    pub fn update_menu(&mut self, ctx: &mut Context, current_state: &mut states::States) {
        self.index = self.options.iter().position(|&s| s.to_string() == self.current_option).unwrap();

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            if self.index > 0 {
                if !self.w_pressed {
                    self.index -= 1;
                    self.w_pressed = true;
                }
            }
            self.current_option = self.options[self.index];
        }
        else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            if self.index < 2 {
                if !self.s_pressed {
                    self.index += 1;
                    self.s_pressed = true;
                }
            }
            self.current_option = self.options[self.index];
        }
        else {
            self.w_pressed = false;
            self.s_pressed = false;
        }

        if states::PlayState::StartScreen == current_state.playstate {
            self.reset_stats = true;
            if keyboard::is_key_pressed(ctx, KeyCode::J) && self.current_option == "play" {
                if !self.j_pressed {
                    self.j_pressed = true;
                    current_state.playstate = states::PlayState::GameModeSelect;
                }
            }
            else if keyboard::is_key_pressed(ctx, KeyCode::J) && self.current_option == "exit" {
                if !self.j_pressed {
                    self.j_pressed = true;
                    std::process::abort();
                }
            }
            else {
                self.j_pressed = false;
            }
        }
        if states::PlayState::GameModeSelect == current_state.playstate {
            if keyboard::is_key_pressed(ctx, KeyCode::J) && self.current_option == "controls" {
                if !self.j_pressed {
                    self.j_pressed = true;
                    current_state.playstate = states::PlayState::Play;
                }
            }
            else if keyboard::is_key_pressed(ctx, KeyCode::J) && self.current_option == "exit" {
                if !self.j_pressed {
                    self.j_pressed = true;
                    current_state.playstate = states::PlayState::StartScreen;
                }
            }
            else {
                self.j_pressed = false;
            }
        }
        if states::PlayState::Dead == current_state.playstate {
            if self.index == 0 {
                self.current_option = "controls";
            }

            if keyboard::is_key_pressed(ctx, KeyCode::J) && self.current_option == "controls" {
                self.reset_stats = true;
                if !self.j_pressed {
                    self.j_pressed = true;
                    current_state.playstate = states::PlayState::Play;
                }
            }
            else if keyboard::is_key_pressed(ctx, KeyCode::J) && self.current_option == "exit" {
                self.reset_stats = true;
                if !self.j_pressed {
                    self.j_pressed = true;
                    current_state.playstate = states::PlayState::StartScreen;
                }
            }
            else {
                self.j_pressed = false;
            }
        }
        if states::PlayState::Pause == current_state.playstate {
            if keyboard::is_key_pressed(ctx, KeyCode::J) && self.current_option == "play" {
                if !self.j_pressed {
                    self.j_pressed = true;
                    current_state.playstate = states::PlayState::Play;
                }
            }
            else if keyboard::is_key_pressed(ctx, KeyCode::J) && self.current_option == "exit" {
                if !self.j_pressed {
                    self.j_pressed = true;
                    current_state.playstate = states::PlayState::StartScreen;
                }
            }
            else {
                self.j_pressed = false;
            }
        }
    }

    pub fn draw_menu(&mut self, ctx: &mut Context, sprites: &mut sprites::Sprites) {
        states::States::draw_sprite(ctx, 20.0, 44.0, 
            sprites.menu_sprites.back_sprite[self.current_menu_sprite], &mut sprites.menu_sprites.back_display);

        states::States::draw_sprite(ctx, 35.0, 205.0, 
            sprites.menu_sprites.sit_sprite[self.current_sit_sprite], &mut sprites.menu_sprites.p_sit);

        states::States::draw_sprite(ctx, 528.0, 150.0, 
            sprites.menu_sprites.options_sprite[self.index + 1], &mut sprites.menu_sprites.options_display);

        if self.current_option == "controls" {
            states::States::draw_sprite(ctx, 288.0, 153.0, 
                sprites.menu_sprites.controls_sprite, &mut sprites.menu_sprites.controls_display);
        }
    }

    pub fn draw_pause_menu(&mut self, ctx: &mut Context, sprites: &mut sprites::Sprites) {
        states::States::draw_sprite(ctx, 20.0, 44.0, 
            sprites.menu_sprites.back_sprite[10], &mut sprites.menu_sprites.back_display);
    }

    pub fn draw_pause_menu_options(&mut self, ctx: &mut Context, sprites: &mut sprites::Sprites) {
        states::States::draw_sprite(ctx, 528.0, 150.0,
            sprites.menu_sprites.pause_menu_sprite[self.index + 1], 
            &mut sprites.menu_sprites.pause_menu_display);

        if self.current_option == "controls" {
            states::States::draw_sprite(ctx, 288.0, 153.0, 
                sprites.menu_sprites.controls_sprite, &mut sprites.menu_sprites.controls_display);
        }
    }

    pub fn draw_gamemode_menu(&mut self, ctx: &mut Context, sprites: &mut sprites::Sprites) {
        states::States::draw_sprite(ctx, 20.0, 44.0, 
            sprites.menu_sprites.back_sprite[self.current_menu_sprite], &mut sprites.menu_sprites.back_display);

        states::States::draw_sprite(ctx, 35.0, 205.0, 
            sprites.menu_sprites.sit_sprite[self.current_sit_sprite], &mut sprites.menu_sprites.p_sit);

        if self.current_option == "controls" {
            self.now = Instant::now();
            states::States::draw_sprite(ctx, 528.0, 150.0, 
                sprites.menu_sprites.gamemode_menu_sprite[self.current_tp_sprite], &mut sprites.menu_sprites.gamemode_menu_display);
        }
        else {
            states::States::draw_sprite(ctx, 528.0, 150.0, 
                sprites.menu_sprites.gamemode_menu_sprite[self.index], &mut sprites.menu_sprites.gamemode_menu_display);
        }
    }

    pub fn draw_dead_screen(&mut self, ctx: &mut Context, sprites: &mut sprites::Sprites) {
        states::States::draw_sprite(ctx, 250.0, 170.0,
            sprites.menu_sprites.dead_screen_sprite[self.index], &mut sprites.menu_sprites.dead_screen_display);
    }

    pub fn draw_cover(&mut self, ctx: &mut Context, sprites: &mut sprites::Sprites) {
        states::States::draw_sprite(ctx, 0.0, 0.0, 
            sprites.menu_sprites.cover_sprite, &mut sprites.menu_sprites.cover_display);
    }
}