use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::graphics::text::{Font, Text};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const WINDOW_WIDTH:  f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

const PADDLE_SPEED: f32 = 8.0;
const BALL_SPEED:   f32 = 5.0;

const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC:    f32 = 0.05;

// Create GameState with default behaviour 
struct GameState { 
    player1: Entity,
    player2: Entity,
    ball: Entity,
    score1: (Text, i32),
    score2: (Text, i32),
}

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Self {
        Self::with_velocity(texture, position, Vec2::zero())
    }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Self {
        Self {
            texture,
            position,
            velocity,
        }
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }
    
    fn height(&self) -> f32 {
        self.texture.height() as f32
    }
    
    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        // Load texture
        let paddle_texture = Texture::new(ctx, "./assets/player.png")?;
        
        // Set paddles positions
        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - paddle_texture.height() as f32) / 2.0,
        );
        let player2_position = Vec2::new(
            WINDOW_WIDTH - paddle_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - paddle_texture.height() as f32) / 2.0,
        );

        // Set up Ball
        let ball_texture = Texture::new(ctx, "./assets/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);

        let text = Text::new("Score: 0", Font::vector(ctx, "/usr/share/fonts/TTF/DejaVuSans.ttf", 12.0)?);
        // Init GameState
        Ok(GameState {
            player1: Entity::new(paddle_texture.clone(), player1_position),
            player2: Entity::new(paddle_texture, player2_position),
            ball:    Entity::with_velocity(ball_texture, ball_position, ball_velocity),
            score1: (text.clone(), 0),
            score2: (text, 0),
        })
    }

    fn reset_ball(&mut self, ctx: &mut Context) -> tetra::Result {
        let ball_texture = Texture::new(ctx, "./assets/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );

        self.ball.position = ball_position;
        self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);
        Ok(())
    }
}

impl State for GameState {    
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        // Check input
        if input::is_key_down(ctx, Key::W) && self.player1.position.y > 0.0 {
            self.player1.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) && self.player1.position.y < WINDOW_HEIGHT - self.player1.height() {
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up) && self.player2.position.y > 0.0 {
            self.player2.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down) && self.player2.position.y < WINDOW_HEIGHT - self.player1.height() {
            self.player2.position.y += PADDLE_SPEED;
        }

        // Moving ball
        self.ball.position += self.ball.velocity;

        // Collision detection
        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };


        // Vary ball height
        if let Some(paddle) = paddle_hit {
            // Increase the ball's velocity, then flip it.
            self.ball.velocity.x =
                -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));
        
            // Calculate the offset between the paddle and the ball, as a number between
            // -1.0 and 1.0.
            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();
        
            // Apply the spin to the ball.
            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        // Collide with top and bottom
        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // Decide winner 
        if self.ball.position.x < 0.0 {
            self.score2.1 += 1;
            self.reset_ball(ctx)?;
            self.score2.0.set_content(format!("Score: {}", self.score2.1));
        }
        
        if self.ball.position.x > WINDOW_WIDTH {
            self.score1.1 += 1;
            self.reset_ball(ctx)?;
            self.score1.0.set_content(format!("Score: {}", self.score1.1));
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Paint background with gray 
        graphics::clear(ctx, Color::rgb(0.2, 0.2, 0.2));
        // Draw paddle texture
        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.ball.texture.draw(ctx, self.ball.position);
        self.score1.0.draw(ctx, Vec2::new(16.0, 16.0));
        self.score2.0.draw(ctx, Vec2::new(WINDOW_WIDTH - 66.0, 16.0));
        Ok(())
    }
}


fn main() -> tetra::Result {
    // Create Window and quit on escape
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
