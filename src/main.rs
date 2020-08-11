use ggez::*;
use ggez::nalgebra::Point2;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::graphics::DrawParam;
use std::time::Instant;
use std::path;
use std::env;

mod menu;
mod pearl;
mod target;
mod states;
mod sprites;

struct GameState {
	title_screen: menu::Menu,
	player: pearl::Pearl,
	enemies: Vec<target::Target>,
	current_state: states::States,
    sprites: sprites::Sprites,

    points: usize,
    font: graphics::Font,
    points_text: graphics::Text,
    fps_text: graphics::Text,
    time_text: graphics::Text,

    time_since_hit: Instant,
    time: u128,
    pause_time: bool,
    target_time: f32,
    average_target_time: f32,
    slowest_target_time: f32,
    fastest_target_time: f32,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
    	let title_screen = menu::Menu::new();
    	let player = pearl::Pearl::new();
    	let enemies = vec![target::Target::new()];
    	let current_state = states::States::new();
        let sprites = sprites::Sprites::new(ctx);
        let points = 0;
        let time = 0;
        let time_since_hit = Instant::now();
        let font = graphics::Font::new(ctx, "/My_Pixel_Font.ttf").unwrap();
        let points_text = graphics::Text::new((points.to_string(), font, 22.0));
        let time_text = graphics::Text::new(("0.000", font, 22.0));
        let fps_text = graphics::Text::new((ggez::timer::fps(ctx).to_string(), font, 22.0));
        let pause_time = false;
        let target_time = 0.0;
        let average_target_time = 0.0;
        let slowest_target_time = 0.0;
        let fastest_target_time = 100.5;

    	let s = GameState {
    		title_screen, player, enemies, current_state, sprites, 
            points, font, points_text, fps_text, 
            time_text, time_since_hit, time, pause_time, 
            target_time, average_target_time, slowest_target_time, fastest_target_time,
    	};
    	Ok(s)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
    	self.current_state.update_frames(ctx, &mut self.title_screen, &mut self.player, &mut self.enemies);

        //Reset stats on title screen
        if self.title_screen.reset_stats {
            self.points = 0;
            self.average_target_time = 0.0;
            self.slowest_target_time = 0.0;
            self.fastest_target_time = 100.5;
            self.title_screen.reset_stats = false;
            self.player.physics.position = Point2::new(35.0, 205.0);

            //Reset targets
            for x in 0..self.enemies.len() {
                self.enemies.remove(x);
                self.enemies.push(target::Target::new());
            }
        }
    	
        //update object states
        if states::PlayState::StartScreen == self.current_state.playstate {
            self.title_screen.update_menu(ctx, &mut self.current_state);
            self.pause_time = true;
        }
    	else if states::PlayState::GameModeSelect == self.current_state.playstate || states::PlayState::Pause == self.current_state.playstate {
    		self.title_screen.update_menu(ctx, &mut self.current_state);
    	}
        else if states::PlayState::Dead == self.current_state.playstate {
            self.title_screen.update_menu(ctx, &mut self.current_state);
            self.pause_time = true;
        }
    	else if states::PlayState::Play == self.current_state.playstate { 
            if self.pause_time {
                self.time_since_hit = Instant::now();
                self.pause_time = false;
            }
    		self.player.update_player(ctx, &mut self.current_state);
            self.points_text = graphics::Text::new((self.points.to_string(), self.font, 22.0));
            if self.current_state.frames % 100 == 0 {
                self.fps_text = graphics::Text::new((((ggez::timer::fps(ctx)*100.0).round()/100.0).to_string(), self.font, 22.0));
            }

            self.time = self.time_since_hit.elapsed().as_millis();
            self.time_text = graphics::Text::new((((self.time as f32)/1000.0).to_string(), self.font, 22.0));

            //if target destroyed, remove from vector, add new target and add point
            for x in 0..self.enemies.len() {
                self.enemies[x].update_target(&mut self.player);
                if self.enemies[x].destroy_target {
                    self.enemies.remove(x);
                    self.points += 1;
                    self.enemies.push(target::Target::new());
                    self.time_since_hit = Instant::now();
                    self.enemies[x].stop_timer = false;
                }

                if self.enemies[x].stop_timer {
                    if self.time_since_hit.elapsed().as_millis() > 50 {
                        self.target_time = (self.time_since_hit.elapsed().as_millis() as f32)/1000.0;
                        self.average_target_time += self.target_time;
                        if self.slowest_target_time < self.target_time {
                            self.slowest_target_time = self.target_time;
                        }
                        if self.fastest_target_time > self.target_time {
                            self.fastest_target_time = self.target_time;
                        }
                    }
                    self.time_since_hit = Instant::now();
                    self.time_text = graphics::Text::new((self.target_time.to_string(), self.font, 22.0));
                }
            }
    	}

        if self.points > 9 {
            if self.points == 10 {
                self.average_target_time = (self.average_target_time/10.0 * 1000.0).round()/1000.0;
                self.current_state.playstate = states::PlayState::Dead;
            }
            self.points += 1;
        }

    	Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
    	graphics::clear(ctx, graphics::Color::from((0.0, 0.0, 0.0, 0.0)));

		if states::PlayState::StartScreen == self.current_state.playstate {
			self.title_screen.draw_menu(ctx, &mut self.sprites);
		}
        else if states::PlayState::GameModeSelect == self.current_state.playstate {
            self.title_screen.draw_gamemode_menu(ctx, &mut self.sprites);
        }
		else if states::PlayState::Play == self.current_state.playstate {
            self.player.draw_background(ctx, &mut self.sprites);
            self.player.draw_top_bar(ctx, &mut self.sprites);
    		
            for x in 0..self.enemies.len() {
                self.enemies[x].draw_target(ctx, &mut self.sprites);
            }

            self.player.draw_player(ctx, &mut self.sprites);

            draw_stats(ctx, &mut self.points_text, &mut self.fps_text, &mut self.time_text);

    		/*SHOW HITBOXES

    		let rect = graphics::Rect::new(self.player.physics.position.x, self.player.physics.position.y, 32.0, 58.0);
	        let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(1.0), rect, graphics::WHITE).unwrap();
	        graphics::draw(ctx, &r1, DrawParam::default()).unwrap();
			let c1 = graphics::Mesh::new_circle(ctx, graphics::DrawMode::stroke(1.0), 
                Point2::new(self.enemies[self.enemies.len() - 1].position.x + 22.0, 
                self.enemies[self.enemies.len() - 1].position.y + 22.0), 
                22.0, 0.1, graphics::WHITE).unwrap();
	        graphics::draw(ctx, &c1, DrawParam::default()).unwrap();
            */
		}
        else if states::PlayState::Pause == self.current_state.playstate {
            self.title_screen.draw_pause_menu(ctx, &mut self.sprites);
            self.player.draw_top_bar(ctx, &mut self.sprites);
            self.player.draw_player(ctx, &mut self.sprites);
            for x in 0..self.enemies.len() {
                self.enemies[x].draw_target(ctx, &mut self.sprites);
            }
            self.title_screen.draw_pause_menu_options(ctx, &mut self.sprites);
            draw_stats(ctx, &mut self.points_text, &mut self.fps_text, &mut self.time_text);
        }
        else if states::PlayState::Dead == self.current_state.playstate {
            self.player.draw_background(ctx, &mut self.sprites);
            self.player.draw_top_bar(ctx, &mut self.sprites);
            self.player.draw_player(ctx, &mut self.sprites);
            for x in 0..self.enemies.len() {
                self.enemies[x].draw_target(ctx, &mut self.sprites);
            }
            self.title_screen.draw_dead_screen(ctx, &mut self.sprites);
            self.time_text = graphics::Text::new(("0.000", self.font, 22.0));
            draw_stats(ctx, &mut self.points_text, &mut self.fps_text, &mut self.time_text);

            graphics::draw(ctx, &graphics::Text::new((self.average_target_time.to_string(), self.font, 22.0)), 
                (Point2::new(360.0, 170.0), 0.0, graphics::WHITE)).unwrap();
            graphics::draw(ctx, &graphics::Text::new((self.slowest_target_time.to_string(), self.font, 22.0)), 
                (Point2::new(360.0, 184.0), 0.0, graphics::WHITE)).unwrap();
            graphics::draw(ctx, &graphics::Text::new((self.fastest_target_time.to_string(), self.font, 22.0)), 
                (Point2::new(360.0, 198.0), 0.0, graphics::WHITE)).unwrap();
        }

        self.title_screen.draw_cover(ctx, &mut self.sprites);

    	graphics::present(ctx)?;

    	Ok(())
    }
}

fn main() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("shoefull", "Manraj Gill")
        .window_setup(conf::WindowSetup::default().title("Shoefull"))
        .window_mode(conf::WindowMode::default().dimensions(640.0, 360.0));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new(ctx)?;
    event::run(ctx, event_loop, state)
}

pub fn draw_stats(ctx: &mut Context, points_text: &mut graphics::Text, fps_text: &mut graphics::Text, time_text: &mut graphics::Text) {
    graphics::draw(ctx, points_text, (Point2::new(600.0, 50.0), 0.0, graphics::WHITE)).unwrap();
    graphics::draw(ctx, fps_text, (Point2::new(582.0, 301.0), 0.0, graphics::WHITE)).unwrap();
    graphics::draw(ctx, time_text, (Point2::new(305.0, 301.0), 0.0, graphics::WHITE)).unwrap();
}
