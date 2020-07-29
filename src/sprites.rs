use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::graphics::spritebatch::SpriteBatch;

pub struct MenuSprites {
	pub back_display: SpriteBatch,
    pub back_sprite: Vec<graphics::Rect>,
    pub options_display: SpriteBatch,
    pub options_sprite: Vec<graphics::Rect>,

    pub controls_display: SpriteBatch,
    pub controls_sprite: graphics::Rect,

    pub p_sit: SpriteBatch,
    pub sit_sprite: Vec<graphics::Rect>,

    pub pause_menu_display: SpriteBatch,
    pub pause_menu_sprite: Vec<graphics::Rect>,

    pub gamemode_menu_display: SpriteBatch,
    pub gamemode_menu_sprite: Vec<graphics::Rect>,

    pub dead_screen_display: SpriteBatch,
    pub dead_screen_sprite: Vec<graphics::Rect>,

    pub cover_display: SpriteBatch,
    pub cover_sprite: graphics::Rect,
}

impl MenuSprites {
	pub fn new (ctx: &mut Context) -> GameResult<MenuSprites> {
		let mut back_image = graphics::Image::new(ctx, "/Back.png").unwrap();
        back_image.set_filter(graphics::FilterMode::Nearest);
        let back_display = SpriteBatch::new(back_image);
        let back_sprite = vec![
            graphics::Rect::new(0.0, 0.0, 0.25, 0.25),
            graphics::Rect::new(0.25, 0.0, 0.25, 0.25),
            graphics::Rect::new(0.5, 0.0, 0.25, 0.25),
            graphics::Rect::new(0.75, 0.0, 0.25, 0.25),
            graphics::Rect::new(0.0, 0.25, 0.25, 0.25),
            graphics::Rect::new(0.25, 0.25, 0.25, 0.25),
            graphics::Rect::new(0.5, 0.25, 0.25, 0.25),
            graphics::Rect::new(0.75, 0.25, 0.25, 0.25),
            graphics::Rect::new(0.0, 0.5, 0.25, 0.25),
            graphics::Rect::new(0.25, 0.5, 0.25, 0.25),
            graphics::Rect::new(0.75, 0.75, 0.25, 0.25),
        ];

        let mut options_image = graphics::Image::new(ctx, "/Options.png").unwrap();
        options_image.set_filter(graphics::FilterMode::Nearest);
        let options_display = SpriteBatch::new(options_image);
        let options_sprite = vec![
            graphics::Rect::new(0.0, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.125, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.25, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.375, 0.0, 0.125, 0.25),
        ];

        let mut controls_image = graphics::Image::new(ctx, "/Controls.png").unwrap();
        controls_image.set_filter(graphics::FilterMode::Nearest);
        let controls_display = SpriteBatch::new(controls_image);
        let controls_sprite = graphics::Rect::new(0.0, 0.0, 1.0, 1.0);

        let mut p_sit_image = graphics::Image::new(ctx, "/Pearl_Sit.png").unwrap();
        p_sit_image.set_filter(graphics::FilterMode::Nearest);
        let p_sit = SpriteBatch::new(p_sit_image);
        let sit_sprite = vec![
            graphics::Rect::new(0.0, 0.0, 0.5, 0.25),
            graphics::Rect::new(0.5, 0.0, 0.5, 0.25),
            graphics::Rect::new(0.0, 0.25, 0.5, 0.25),
            graphics::Rect::new(0.5, 0.25, 0.5, 0.25),
            graphics::Rect::new(0.0, 0.5, 0.5, 0.25),
            graphics::Rect::new(0.5, 0.5, 0.5, 0.25),
            graphics::Rect::new(0.0, 0.75, 0.5, 0.25),
            graphics::Rect::new(0.5, 0.75, 0.5, 0.25),
        ];

        let mut pause_menu_image = graphics::Image::new(ctx, "/Options.png").unwrap();
        pause_menu_image.set_filter(graphics::FilterMode::Nearest);
        let pause_menu_display = SpriteBatch::new(pause_menu_image);
        let pause_menu_sprite = vec![
            graphics::Rect::new(0.5, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.625, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.75, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.875, 0.0, 0.125, 0.25),
        ];

        let mut gamemode_menu_image = graphics::Image::new(ctx, "/Options.png").unwrap();
        gamemode_menu_image.set_filter(graphics::FilterMode::Nearest);
        let gamemode_menu_display = SpriteBatch::new(gamemode_menu_image);
        let gamemode_menu_sprite = vec![
            graphics::Rect::new(0.125, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.25, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.25, 0.75, 0.125, 0.25),
            graphics::Rect::new(0.375, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.5, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.625, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.75, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.875, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.0, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.125, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.25, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.375, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.5, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.625, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.75, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.875, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.0, 0.75, 0.125, 0.25),
            graphics::Rect::new(0.125, 0.75, 0.125, 0.25),
            graphics::Rect::new(0.25, 0.25, 0.125, 0.25),
        ];

        let mut dead_screen_image = graphics::Image::new(ctx, "/Dead_Screen.png").unwrap();
        dead_screen_image.set_filter(graphics::FilterMode::Nearest);
        let dead_screen_display = SpriteBatch::new(dead_screen_image);
        let dead_screen_sprite = vec![
        	graphics::Rect::new(0.0, 0.0, 0.5, 0.5),
            graphics::Rect::new(0.5, 0.0, 0.5, 0.5),
            graphics::Rect::new(0.0, 0.5, 0.5, 0.5),
        ];


        let mut cover_image = graphics::Image::new(ctx, "/Cover.png").unwrap();
        cover_image.set_filter(graphics::FilterMode::Nearest);
        let cover_display = SpriteBatch::new(cover_image);
        let cover_sprite = graphics::Rect::new(0.0, 0.0, 1.0, 1.0);

        Ok(MenuSprites {
        	back_display, back_sprite, options_display, options_sprite,
        	controls_display, controls_sprite, p_sit, sit_sprite,
        	pause_menu_display, pause_menu_sprite, 
        	gamemode_menu_display, gamemode_menu_sprite,
        	dead_screen_display, dead_screen_sprite,
        	cover_display, cover_sprite,
        })
	}
}

pub struct PearlSprites {
	pub p_idle: SpriteBatch,
    pub idle_sprite: Vec<graphics::Rect>,

    pub p_run: SpriteBatch,
    pub run_sprite: Vec<graphics::Rect>,

    pub p_jump: SpriteBatch,
    pub jump_sprite: Vec<graphics::Rect>,

    pub p_fall: SpriteBatch,
    pub fall_sprite: Vec<graphics::Rect>,

    pub p_crouch: SpriteBatch,
    pub crouch_sprite: Vec<graphics::Rect>,

    pub p_dash: SpriteBatch,
    pub dash_sprite: Vec<graphics::Rect>,

    pub back_display: SpriteBatch,
    pub back_sprite: Vec<graphics::Rect>,

    pub top_bar_display: SpriteBatch,
    pub top_bar_sprite: Vec<graphics::Rect>,
}

impl PearlSprites {
	pub fn new (ctx: &mut Context) -> GameResult<PearlSprites> {
		let mut p_idle_image = graphics::Image::new(ctx, "/Pearl_Idle.png").unwrap();
        p_idle_image.set_filter(graphics::FilterMode::Nearest);
        let p_idle = SpriteBatch::new(p_idle_image);
        let idle_sprite = vec![
            graphics::Rect::new(0.0, 0.0, 0.25, 1.0),
            graphics::Rect::new(0.25, 0.0, 0.25, 1.0),
            graphics::Rect::new(0.5, 0.0, 0.25, 1.0),
            graphics::Rect::new(0.75, 0.0, 0.25, 1.0),
        ];

        let mut p_run_image = graphics::Image::new(ctx, "/Pearl_Run.png").unwrap();
        p_run_image.set_filter(graphics::FilterMode::Nearest);
        let p_run = SpriteBatch::new(p_run_image);
        let run_sprite = vec![
            graphics::Rect::new(0.0, 0.0, 0.2, 0.25),
            graphics::Rect::new(0.2, 0.0, 0.2, 0.25),
            graphics::Rect::new(0.4, 0.0, 0.2, 0.25),
            graphics::Rect::new(0.6, 0.0, 0.2, 0.25),
            graphics::Rect::new(0.8, 0.0, 0.2, 0.25),
            graphics::Rect::new(0.0, 0.25, 0.2, 0.25),
            graphics::Rect::new(0.2, 0.25, 0.2, 0.25),
            graphics::Rect::new(0.4, 0.25, 0.2, 0.25),
            graphics::Rect::new(0.6, 0.25, 0.2, 0.25),
            graphics::Rect::new(0.8, 0.25, 0.2, 0.25),
            graphics::Rect::new(0.0, 0.5, 0.2, 0.25),
            graphics::Rect::new(0.2, 0.5, 0.2, 0.25),
            graphics::Rect::new(0.4, 0.5, 0.2, 0.25),
            graphics::Rect::new(0.6, 0.5, 0.2, 0.25),
            graphics::Rect::new(0.8, 0.5, 0.2, 0.25),
            graphics::Rect::new(0.0, 0.75, 0.2, 0.25),
            graphics::Rect::new(0.2, 0.75, 0.2, 0.25),
            graphics::Rect::new(0.4, 0.75, 0.2, 0.25),
            graphics::Rect::new(0.6, 0.75, 0.2, 0.25),
            graphics::Rect::new(0.8, 0.75, 0.2, 0.25),
        ];

        let mut p_jump_image = graphics::Image::new(ctx, "/Pearl_Jump.png").unwrap();
        p_jump_image.set_filter(graphics::FilterMode::Nearest);
        let p_jump = SpriteBatch::new(p_jump_image);
        let jump_sprite = vec![
            graphics::Rect::new(0.0, 0.0, 0.5, 1.0),
            graphics::Rect::new(0.5, 0.0, 0.5, 1.0),
        ];

        let mut p_fall_image = graphics::Image::new(ctx, "/Pearl_Fall.png").unwrap();
        p_fall_image.set_filter(graphics::FilterMode::Nearest);
        let p_fall = SpriteBatch::new(p_fall_image);
        let fall_sprite = vec![
            graphics::Rect::new(0.0, 0.0, 0.5, 1.0),
            graphics::Rect::new(0.5, 0.0, 0.5, 1.0),
        ];

        let mut p_crouch_image = graphics::Image::new(ctx, "/Pearl_Crouch.png").unwrap();
        p_crouch_image.set_filter(graphics::FilterMode::Nearest);
        let p_crouch = SpriteBatch::new(p_crouch_image);
        let crouch_sprite = vec![
            graphics::Rect::new(0.0, 0.0, 0.5, 1.0),
            graphics::Rect::new(0.5, 0.0, 0.5, 1.0),
        ];

        let mut p_dash_image = graphics::Image::new(ctx, "/Pearl_Dash.png").unwrap();
        p_dash_image.set_filter(graphics::FilterMode::Nearest);
        let p_dash = SpriteBatch::new(p_dash_image);
        let dash_sprite = vec![
            graphics::Rect::new(0.0, 0.0, 0.5, 1.0),
            graphics::Rect::new(0.5, 0.0, 0.5, 1.0),
        ];

        let mut back_image = graphics::Image::new(ctx, "/Back.png").unwrap();
        back_image.set_filter(graphics::FilterMode::Nearest);
        let back_display = SpriteBatch::new(back_image);
        let back_sprite = vec![
            graphics::Rect::new(0.0, 0.0, 0.25, 0.25),
            graphics::Rect::new(0.5, 0.5, 0.25, 0.25),
            graphics::Rect::new(0.75, 0.5, 0.25, 0.25),
            graphics::Rect::new(0.0, 0.75, 0.25, 0.25),
            graphics::Rect::new(0.25, 0.75, 0.25, 0.25),
            graphics::Rect::new(0.5, 0.75, 0.25, 0.25),
        ];

        let mut top_bar_image = graphics::Image::new(ctx, "/Top_Bar.png").unwrap();
        top_bar_image.set_filter(graphics::FilterMode::Nearest);
        let top_bar_display = SpriteBatch::new(top_bar_image);
        let top_bar_sprite = vec![
            graphics::Rect::new(0.0, 0.0, 1.0, 0.25),
            graphics::Rect::new(0.0, 0.25, 1.0, 0.25),
            graphics::Rect::new(0.0, 0.5, 1.0, 0.25),
            graphics::Rect::new(0.0, 0.75, 1.0, 0.25),
        ];

        Ok(PearlSprites {
        	p_idle, idle_sprite, p_run, run_sprite, p_jump, jump_sprite,
        	p_fall, fall_sprite, p_crouch, crouch_sprite, p_dash, dash_sprite,
        	back_display, back_sprite, top_bar_display, top_bar_sprite,
        })
	}
}

pub struct TargetSprites {
	pub target: SpriteBatch,
    pub target_sprite: Vec<graphics::Rect>,
}

impl TargetSprites {
	pub fn new (ctx: &mut Context) -> GameResult<TargetSprites> {
		let mut target_image = graphics::Image::new(ctx, "/Target.png").unwrap();
        target_image.set_filter(graphics::FilterMode::Nearest);
        let target = SpriteBatch::new(target_image);
        let target_sprite = vec![
            graphics::Rect::new(0.0, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.125, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.25, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.375, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.5, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.625, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.75, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.875, 0.0, 0.125, 0.25),
            graphics::Rect::new(0.0, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.125, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.25, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.375, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.5, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.625, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.75, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.875, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.0, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.125, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.25, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.375, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.5, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.625, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.75, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.875, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.0, 0.75, 0.125, 0.25),
            graphics::Rect::new(0.125, 0.75, 0.125, 0.25),
            graphics::Rect::new(0.25, 0.75, 0.125, 0.25),
            graphics::Rect::new(0.375, 0.75, 0.125, 0.25),
            graphics::Rect::new(0.5, 0.75, 0.125, 0.25),
            graphics::Rect::new(0.625, 0.75, 0.125, 0.25),
            graphics::Rect::new(0.375, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.5, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.625, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.75, 0.5, 0.125, 0.25),
            graphics::Rect::new(0.875, 0.25, 0.125, 0.25),
            graphics::Rect::new(0.0, 0.75, 0.125, 0.25),
        ];

        Ok(TargetSprites {
        	target, target_sprite,
        })
	}
}

pub struct Sprites {
	pub menu_sprites: MenuSprites,
	pub pearl_sprites: PearlSprites,
	pub target_sprites: TargetSprites,
}

impl Sprites {
	pub fn new(ctx: &mut Context) -> Self {
		let menu_sprites = MenuSprites::new(ctx).unwrap();
		let pearl_sprites = PearlSprites::new(ctx).unwrap();
		let target_sprites = TargetSprites::new(ctx).unwrap();

		Self {
			menu_sprites, pearl_sprites, target_sprites,
		}
	}
}