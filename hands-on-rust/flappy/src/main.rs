use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 90.0;

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32, // the y center of the gap
    size: i32,  // the size of the gap
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        let gap_y = random.range(10, 40);
        let size = i32::max(2, 20 - score);
        Obstacle { x, gap_y, size }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let gap_top = self.gap_top();
        let gap_bottom = self.gap_bottom();
        // moves the wall relative to the player x position
        let x = self.x - player_x;

        for y in 0..gap_top {
            ctx.set(x, y, YELLOW1, BLACK, to_cp437('#'));
        }
        for y in gap_bottom..SCREEN_HEIGHT {
            ctx.set(x, y, YELLOW, BLACK, to_cp437('#'));
        }
    }

    fn hit_obsctale(&self, player: &Player) -> bool {
        let player_at_wall = self.x == player.x;
        let player_too_high = Obstacle::is_y_above(player.y, self.gap_top());
        let player_too_low = Obstacle::is_y_below(player.y, self.gap_bottom());

        player_at_wall && (player_too_high || player_too_low)
    }

    fn gap_top(&self) -> i32 {
        let half_gap = self.size / 2;
        self.gap_y - half_gap
    }

    fn gap_bottom(&self) -> i32 {
        let half_gap = self.size / 2;
        self.gap_y + half_gap
    }

    fn is_y_above(y: i32, y_limit: i32) -> bool {
        // the lower the value of Y, the higher the position on the screen
        y < y_limit
    }
    fn is_y_below(y: i32, y_limit: i32) -> bool {
        y > y_limit
    }
}

struct State {
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _other => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        self.obstacle.render(ctx, self.player.x);

        if self.player.x > self.obstacle.x {
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
            self.score += 1;
        }

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            ctx.print(9, 0, "^-^");
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score {}", self.score));

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obsctale(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _other => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
        self.score = 0;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
