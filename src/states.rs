use ggez::*;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::Context;
use std::time::Duration;
use crate::menu;
use crate::pearl;
use crate::target;

#[derive(PartialEq)]
pub enum PlayState {
    StartScreen,
    GameModeSelect,
    ShoeSelect,
    Play,
    Pause,
    Dead,
}

pub struct States {
	pub frames: usize,
	pub playstate: PlayState,
}

impl States {
	pub fn new() -> Self {
		let frames = 0;
		let playstate = PlayState::StartScreen;

		Self {
            frames, playstate,
        }
	}

	pub fn update_frames(&mut self, ctx: &mut Context, title_screen: &mut menu::Menu, player: &mut pearl::Pearl, enemies: &mut Vec<target::Target>) {
		while timer::check_update_time(ctx, 18) {
			if PlayState::StartScreen == self.playstate || PlayState::GameModeSelect == self.playstate{
				title_screen.current_menu_sprite += 1;
	            if title_screen.current_menu_sprite > 9 {
	                title_screen.current_menu_sprite = 1;
	            }
	            if title_screen.current_menu_sprite == 2 {
	                if self.frames % 200 > 30 {
	                    title_screen.current_menu_sprite -= 1;
	                }
	            }

	            title_screen.current_sit_sprite += 1;
	            if title_screen.current_sit_sprite > 6 {
	                title_screen.current_sit_sprite = 0;
	            }
	            if title_screen.current_sit_sprite == 1 {
	                if self.frames % 250 > 40 {
	                    title_screen.current_sit_sprite -= 1;
	                }
	            }
	        }
	        else if PlayState::Play == self.playstate {
	            if player.current_back_sprite > 0 {
	                player.current_back_sprite += 1;
	            }

	            if player.now.elapsed() > Duration::from_millis(400) {
	            	player.is_dashing = false;
	            	player.can_dash = true;
	            }
	            else {
	            	player.can_dash = false;
	            }

	            if player.current_back_sprite > 5 {
	                player.current_back_sprite = 0;
	            }

	            if self.frames % 100 > 96 {
	                player.current_idle_left_sprite += 1;
	                player.current_idle_right_sprite += 1;
	            }

	            if player.current_idle_left_sprite > 1 {
	                player.current_idle_left_sprite = 0;
	            }
	            if player.current_idle_right_sprite > 3 {
	                player.current_idle_right_sprite = 2;
	            }

	            player.current_run_left_sprite += 1;
	            if player.current_run_left_sprite > 9 {
	                player.current_run_left_sprite = 0;
	            }

	            player.current_run_right_sprite += 1;
	            if player.current_run_right_sprite > 19 {
	                player.current_run_right_sprite = 10;
	            }

	            for x in 0..enemies.len() {
		        	enemies[x].current_target_sprite += 1;

		        	if self.frames % 80 > 40 {
		        		enemies[x].position.y += 0.7;
		        	}
		        	else {
		        		enemies[x].position.y -= 0.7;
		        	}

			        if enemies[x].current_target_sprite > 13 && !enemies[x].target_hit {
			        	enemies[x].current_target_sprite = 6;
			        }

			        if enemies[x].animate_hit {
			        	enemies[x].current_animate_hit_sprite += 1;
			        }

			        if enemies[x].current_target_sprite == 6 {
		                if self.frames % 200 > 20 {
		                    enemies[x].current_target_sprite -= 1;
		                }
	            	}
	            }
	        }
		}

		if PlayState::GameModeSelect == self.playstate {
        	if title_screen.current_option == "controls" {
        		if title_screen.current_tp_sprite == 1 {
        			title_screen.current_tp_sprite = 3
        		}

        		if self.frames % 10 == 0 {
        			title_screen.current_tp_sprite += 1;
        		}

        		if title_screen.current_tp_sprite > 18 {
        			title_screen.current_tp_sprite = 1;
        		}
        	}
        }

	    self.frames += 1;
	}

	pub fn draw_sprite(ctx: &mut Context, x: f32, y: f32, spritesheet_cut: graphics::Rect, sprite: &mut graphics::spritebatch::SpriteBatch) {
	    sprite.clear();
	    let params1 = DrawParam::new()
	    	.dest([x, y]);
	    let params2 = DrawParam::new()
	        .src(spritesheet_cut)
	        .scale([2.0, 2.0]);
	    sprite.add(params2);
	    graphics::draw(ctx, sprite, params1).unwrap();
	}
}