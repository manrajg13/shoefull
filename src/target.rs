use ggez::nalgebra::Point2;
use ggez::Context;
use rand::Rng;
use crate::states;
use crate::pearl;
use crate::sprites;

pub struct Target {
    pub position: Point2<f32>,
    pub animate_hit: bool,
    pub target_hit: bool,
    pub destroy_target: bool,
    pub stop_timer: bool,
    pub current_target_sprite: usize,
    pub current_animate_hit_sprite: usize,
}

impl Target {
    pub fn new() -> Self {
    	let mut rng = rand::thread_rng();
    	let position = Point2::new(rng.gen_range(20.0, 575.0), rng.gen_range(75.0, 235.0));
    	let animate_hit = false;
    	let target_hit = false;
    	let destroy_target = false;
    	let stop_timer = false;

    	let current_target_sprite = 0;
    	let current_animate_hit_sprite = 14;

        Self {
        	position, animate_hit, target_hit, destroy_target, stop_timer,
        	current_target_sprite, current_animate_hit_sprite,
        }
    }

    pub fn update_target(&mut self, player: &mut pearl::Pearl) {
    	if circle_rect(self.position.x + 22.0, self.position.y + 22.0, 22.0, player.physics.position.x, player.physics.position.y, 32.0, 58.0) && player.is_dashing {
    		self.target_hit = true;
    		self.stop_timer = true;
	    }

    	if self.target_hit && self.current_animate_hit_sprite < 35 {
    		self.position = Point2::new(self.position.x, self.position.y);
    		self.animate_hit = true;
    		self.current_animate_hit_sprite = 14;
            self.current_target_sprite = 0;
            self.target_hit = false;
        }

        if self.current_animate_hit_sprite > 35 {
    		self.destroy_target = true;
    	}
    }

    pub fn draw_target(&mut self, ctx: &mut Context, sprites: &mut sprites::Sprites) {
    	if !self.animate_hit {
    		states::States::draw_sprite(ctx, self.position.x, self.position.y, 
    			sprites.target_sprites.target_sprite[self.current_target_sprite], &mut sprites.target_sprites.target);
    	}
    	if self.animate_hit && self.current_animate_hit_sprite < 35 {
    		states::States::draw_sprite(ctx, self.position.x, self.position.y, 
    			sprites.target_sprites.target_sprite[self.current_animate_hit_sprite], &mut sprites.target_sprites.target);
    	}
    }
}

fn circle_rect(cx: f32, cy: f32, radius: f32, rx: f32, ry: f32, rw: f32, rh: f32) -> bool {
    // temporary variables to set edges for testing
    let mut x: f32 = cx;
    let mut y: f32 = cy;
    // which edge is closest?
    if cx < rx {
        x = rx;      // test left edge
    }
    else if cx > rx+rw {
        x = rx+rw;   // right edge
    }
    if cy < ry {
        y = ry;      // top edge
    }
    else if cy > ry+rh { 
        y = ry+rh;   // bottom edge
    }

    // get distance from closest edges
    let distx: f32 = cx - x;
    let disty: f32 = cy - y;
    let distance: f32 = distx*distx + disty*disty;
    let sqrtdistance: f32 = distance.sqrt();

    // if the distance is less than the radius, then collision
    if sqrtdistance <= radius {
        return true;
    }
    return false;
}