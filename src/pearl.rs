use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use ggez::event::KeyCode;
use ggez::input::keyboard;
use std::time::Instant;
use crate::states;
use crate::sprites;

pub struct Physics {
    pub position: Point2<f32>,
    acceleration_x: Vector2<f32>,
    acceleration_y: f32,
}

impl Physics {
    pub fn new() -> GameResult<Physics> { 
        Ok(Physics {
            position: Point2::new(35.0, 205.0),
            acceleration_x: Vector2::new(0.0, 0.0),
            acceleration_y: 5.0,
        })
    }
}

pub struct Pearl {
    is_facing_right: bool,
    is_crouching: bool,

    pub current_idle_left_sprite: usize,
    pub current_idle_right_sprite: usize,
    pub current_run_left_sprite: usize,
    pub current_run_right_sprite: usize,
    pub current_back_sprite: usize,
    pub is_dashing: bool,
    pub can_dash: bool,
    pub can_double_jump: bool,
    pub now: Instant,
    pub physics: Physics,
}

impl Pearl {
    pub fn new() -> Self {
        let current_idle_left_sprite = 0;
        let current_idle_right_sprite = 2;
        let current_run_left_sprite = 0;
        let current_run_right_sprite = 10;
        let current_back_sprite = 1;
        let is_facing_right = true;
        let is_crouching = false;
        let is_dashing = false;
        let can_dash = true;
        let can_double_jump = true;
        let physics = Physics::new().unwrap();
        let now = Instant::now();

        Self {
            current_idle_left_sprite, current_idle_right_sprite,
            current_run_left_sprite, current_run_right_sprite,
            current_back_sprite, now,
            is_facing_right, is_crouching, is_dashing, can_dash, can_double_jump,
            physics,
        }
    }

    pub fn update_player(&mut self, ctx: &mut Context, current_state: &mut states::States) {
        //Enter Pause state
        if keyboard::is_key_pressed(ctx, KeyCode::P) {
            current_state.playstate = states::PlayState::Pause;
        }

        //Never allow acceleration to reach higher than 20.0
        if self.physics.acceleration_x.x > 20.0 || self.physics.acceleration_x.y > 20.0 {
            self.physics.acceleration_x.x = 20.0;
            self.physics.acceleration_x.y = 20.0;
        }

        //Jump acceleration physics
        if self.physics.acceleration_y < 6.0 || self.physics.position.y < 238.0 {
            self.physics.position.y += self.physics.acceleration_y;
            self.physics.acceleration_y += 0.65;
        }

        //Never allow player to reach lower than ground
        if self.physics.position.y >= 238.0 {
            self.physics.position.y = 238.0;
            self.can_double_jump = true;
        }

        //Vertical acceleration
        //Moving left
        if keyboard::is_key_pressed(ctx, KeyCode::A) && !keyboard::is_key_pressed(ctx, KeyCode::D) && !self.is_crouching {
            self.is_facing_right = false;
            //Dashing acceleration
            if keyboard::is_key_pressed(ctx, KeyCode::J) && self.can_dash && self.can_double_jump{
                self.now = Instant::now();
                self.physics.acceleration_x.x = 20.0;
                //Left acceleration when in midair dash
                if keyboard::is_key_pressed(ctx, KeyCode::W) {
                    self.physics.acceleration_y = 0.0;
                    self.physics.acceleration_y -= 7.0;
                    self.can_double_jump = false;
                }
            }
            //Gain acceleration when not dashing
            if self.physics.acceleration_x.x < 8.0 {
                self.physics.acceleration_x.x += 0.2;
                self.is_dashing = false;
            }
            if self.physics.acceleration_x.x < 8.0 {
                self.physics.acceleration_x.x += 0.2;
            }
            self.physics.position.x -= self.physics.acceleration_x.x;
        }
        //Moving right
        else if keyboard::is_key_pressed(ctx, KeyCode::D) && !keyboard::is_key_pressed(ctx, KeyCode::A) && !self.is_crouching {
            self.is_facing_right = true;
            //Dashing acceleration
            if keyboard::is_key_pressed(ctx, KeyCode::J) && self.can_dash && self.can_double_jump{
                self.now = Instant::now();
                self.physics.acceleration_x.y = 20.0;
                //Right acceleration when in midair dash
                if keyboard::is_key_pressed(ctx, KeyCode::W) {
                    self.physics.acceleration_y = 0.0;
                    self.physics.acceleration_y -= 7.0;
                    self.can_double_jump = false;
                }
            }
            //Gain acceleration when not dashing
            if self.physics.acceleration_x.y < 8.0 {
                self.physics.acceleration_x.y += 0.2;
                self.is_dashing = false;
            }
            self.physics.position.x += self.physics.acceleration_x.y;
        }
        //Stand idle when neither A nor D are pressed
        else {
            if self.physics.acceleration_x.x >= 0.0 || self.physics.acceleration_x.y >= 0.0 {
                self.physics.acceleration_x.x = 0.0;
                self.physics.acceleration_x.y = 0.0;
            }
        }

        //If acceleration goes above default walk acceleration, reduce gradually by 2.0
        if self.physics.acceleration_x.y > 8.2 || self.physics.acceleration_x.x > 8.2 {
            self.physics.acceleration_x.y -= 1.6;
            self.physics.acceleration_x.x -= 1.6;
            self.is_dashing = true;
        }

        //Vertical acceleration
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            if keyboard::is_key_pressed(ctx, KeyCode::J) && self.can_dash && self.can_double_jump{
                self.now = Instant::now();
                self.physics.acceleration_y = 0.0;
                self.physics.acceleration_y -= 10.0;
                self.can_double_jump = false;
                self.is_dashing = true;
            }
            //Only jump when on ground
            if self.physics.position.y == 238.0 {
                self.physics.acceleration_y = -10.0;
            }
        }
        //Crouching only when on ground
        else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            if self.physics.position.y == 238.0 {
                self.is_crouching = true;
            }
        }
        else {
            self.is_crouching = false;
        }

        //Never allow player to go off screen
        if self.physics.position.x < 20.0 {
            self.physics.position.x = 20.0;
            self.physics.acceleration_x.x = 1.0;
        }
        if self.physics.position.x > 588.0 {
            self.physics.position.x = 588.0;
            self.physics.acceleration_x.y = 1.0;
        }
    }

    pub fn draw_background(&mut self, ctx: &mut Context, sprites: &mut sprites::Sprites) {
        states::States::draw_sprite(ctx, 20.0, 44.0, 
            sprites.pearl_sprites.back_sprite[self.current_back_sprite], &mut sprites.pearl_sprites.back_display);
    }

    pub fn draw_top_bar(&mut self, ctx: &mut Context, sprites: &mut sprites::Sprites) {
        states::States::draw_sprite(ctx, 20.0, 44.0, 
            sprites.pearl_sprites.top_bar_sprite[0], &mut sprites.pearl_sprites.top_bar_display);
    }

    pub fn draw_player(&mut self, ctx: &mut Context, sprites: &mut sprites::Sprites) {
        //Air Dash
        if self.is_dashing && self.physics.acceleration_x.x != 0.0 && self.physics.acceleration_x.y != 0.0 {
            if !self.is_facing_right {
                states::States::draw_sprite(ctx, self.physics.position.x, self.physics.position.y + 4.0, 
                    sprites.pearl_sprites.dash_sprite[0], &mut sprites.pearl_sprites.p_dash);
            }
            else {
                states::States::draw_sprite(ctx, self.physics.position.x, self.physics.position.y + 4.0, 
                    sprites.pearl_sprites.dash_sprite[1], &mut sprites.pearl_sprites.p_dash);
            }
        }
        //Jump
        else if self.physics.acceleration_y <= 0.0 {
            if !self.is_facing_right {
                states::States::draw_sprite(ctx, self.physics.position.x, self.physics.position.y, 
                    sprites.pearl_sprites.jump_sprite[0], &mut sprites.pearl_sprites.p_jump);
            }
            else {
                states::States::draw_sprite(ctx, self.physics.position.x, self.physics.position.y, 
                    sprites.pearl_sprites.jump_sprite[1], &mut sprites.pearl_sprites.p_jump);
            }
        }
        //Fall
        else if self.physics.acceleration_y > 0.0 && self.physics.position.y < 238.0 {
            if !self.is_facing_right {
                states::States::draw_sprite(ctx, self.physics.position.x, self.physics.position.y, 
                    sprites.pearl_sprites.fall_sprite[0], &mut sprites.pearl_sprites.p_fall);
            }
            else {
                states::States::draw_sprite(ctx, self.physics.position.x, self.physics.position.y, 
                    sprites.pearl_sprites.fall_sprite[1], &mut sprites.pearl_sprites.p_fall);
            }
        }
        //Crouch
        else if self.is_crouching {
            if !self.is_facing_right {
                states::States::draw_sprite(ctx, self.physics.position.x - 2.0, self.physics.position.y + 14.0, 
                    sprites.pearl_sprites.crouch_sprite[0], &mut sprites.pearl_sprites.p_crouch);
            }
            else {
                states::States::draw_sprite(ctx, self.physics.position.x + 2.0, self.physics.position.y + 14.0, 
                    sprites.pearl_sprites.crouch_sprite[1], &mut sprites.pearl_sprites.p_crouch);
            }
        }
        //Idle
        else if self.physics.acceleration_x.x == 0.0 && self.physics.acceleration_x.y == 0.0 && self.physics.position.y == 238.0 {
            if !self.is_facing_right {
                states::States::draw_sprite(ctx, self.physics.position.x, self.physics.position.y, 
                    sprites.pearl_sprites.idle_sprite[self.current_idle_left_sprite], &mut sprites.pearl_sprites.p_idle);
            }
            else {
                states::States::draw_sprite(ctx, self.physics.position.x, self.physics.position.y, 
                    sprites.pearl_sprites.idle_sprite[self.current_idle_right_sprite], &mut sprites.pearl_sprites.p_idle);
            }
        }
        //Dash
        else if self.is_dashing {
            if !self.is_facing_right {
                states::States::draw_sprite(ctx, self.physics.position.x, self.physics.position.y + 4.0, 
                    sprites.pearl_sprites.dash_sprite[0], &mut sprites.pearl_sprites.p_dash);
            }
            else {
                states::States::draw_sprite(ctx, self.physics.position.x, self.physics.position.y + 4.0, 
                    sprites.pearl_sprites.dash_sprite[1], &mut sprites.pearl_sprites.p_dash);
            }
        }
        //Run
        else if self.physics.position.y == 238.0 {
            if !self.is_facing_right {
                states::States::draw_sprite(ctx, self.physics.position.x - 6.0, self.physics.position.y - 1.6, 
                    sprites.pearl_sprites.run_sprite[self.current_run_left_sprite], &mut sprites.pearl_sprites.p_run);
            }
            else {
                states::States::draw_sprite(ctx, self.physics.position.x - 2.0, self.physics.position.y - 1.6, 
                    sprites.pearl_sprites.run_sprite[self.current_run_right_sprite], &mut sprites.pearl_sprites.p_run);
            }
        }
    }
}