use ggez::graphics;
use ggez::Context;
use ggez::event::{KeyCode};
use ggez::input::keyboard;
use crate::states;

pub struct Pause {
	pause_display: graphics::spritebatch::SpriteBatch,
	pause_sprite: graphics::Rect,

	pause_menu_display: graphics::spritebatch::SpriteBatch,
	pause_menu_sprite: Vec<graphics::Rect>,
	current_pause_menu_sprite: usize,

	current_option: &'static str,
    index: usize,
    w_pressed: bool,
    s_pressed: bool,
}

impl Pause {
	pub fn new(ctx: &mut Context) -> Self {
		let options = ["continue", "controls", "exit"];
		let current_option = "continue";
		let index = 0;
		current_pause_menu_sprite = 0;

		let mut pause_display_image = graphics::Image::new(ctx, "/Options.png").unwrap();
        pause_display_image.set_filter(graphics::FilterMode::Nearest);
        let pause_display = graphics::spritebatch::SpriteBatch::new(options_image);
        let pause_sprite = graphics::Rect::new(0.75, 0.75, 0.25, 0.25),

		let mut pause_menu_image = graphics::Image::new(ctx, "/Options.png").unwrap();
        pause_menu_image.set_filter(graphics::FilterMode::Nearest);
        let pause_menu_display = graphics::spritebatch::SpriteBatch::new(options_image);
        let pause_menu_sprite = vec![
            graphics::Rect::new(0.0, 0.5, 0.5, 0.25),
            graphics::Rect::new(0.5, 0.5, 0.5, 0.25),
            graphics::Rect::new(0.0, 0.75, 0.5, 0.25),
            graphics::Rect::new(0.5, 0.75, 0.5, 0.25),
        ];
	}
}