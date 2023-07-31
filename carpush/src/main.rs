
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, MouseButton, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const PADDLE_SPEED: f32 = 8.0;
const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

fn main() -> tetra::Result {
    ContextBuilder::new("CarPush", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}

struct GameState {
    player1: Entity,
    player2: Entity,
    one: Entity,
    two:Entity,
        three:Entity,
        four:Entity,
        five:Entity,
        reload:Entity,

    // New flag to track if player2 can move or not
    player2_can_move: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./resources/man.png")?;
        let player1_position = Vec2::new(
            60.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );

        let player2_texture = Texture::new(ctx, "./resources/truck.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 500.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );
        let reload_texture = Texture::new(ctx, "./resources/reload.png")?;
        let reload_position = Vec2::new(
            WINDOW_WIDTH - reload_texture.width() as f32 - 500.0,
            (WINDOW_HEIGHT - reload_texture.height() as f32) 
        );

        // ... (other entities)

        let one_texture = Texture::new(ctx, "./resources/100n.png")?;
        let one_position = Vec2::new(
            WINDOW_WIDTH - one_texture.width() as f32 - 500.0,
            (WINDOW_HEIGHT - one_texture.height() as f32) / 8.0,
        );

        let two_texture = Texture::new(ctx, "./resources/200n.png")?;
                let two_position = Vec2::new(
                    WINDOW_WIDTH - two_texture.width() as f32 - 400.0,
                    (WINDOW_HEIGHT - two_texture.height() as f32) / 8.0,
                );
                let three_texture = Texture::new(ctx, "./resources/300n.png")?;
                let three_position = Vec2::new(
                    WINDOW_WIDTH - three_texture.width() as f32 - 300.0,
                    (WINDOW_HEIGHT - three_texture.height() as f32) / 8.0,
                );
                let four_texture = Texture::new(ctx, "./resources/400n.png")?;
                let four_position = Vec2::new(
                    WINDOW_WIDTH - four_texture.width() as f32 - 200.0,
                    (WINDOW_HEIGHT - four_texture.height() as f32) / 8.0,
                );
                let five_texture = Texture::new(ctx, "./resources/500n.png")?;
                let five_position = Vec2::new(
                    WINDOW_WIDTH - five_texture.width() as f32 - 20.0,
                    (WINDOW_HEIGHT - five_texture.height() as f32) / 8.0,
                );

        // ... (other entities)

        Ok(GameState {
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            one: Entity::new(one_texture, one_position),
            // ... (other entities)
            two: Entity::new(two_texture, two_position),
                        three: Entity::new(three_texture, three_position),
                        four: Entity::new(four_texture, four_position),
                        five: Entity::new(five_texture, five_position),
                        reload:Entity::new(reload_texture, reload_position),
            
            player2_can_move: true, // Initialize the flag to allow initial movement
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
       
        if input::is_mouse_button_down(ctx, MouseButton::Left) && self.player2_can_move {
            let cursor_position = input::get_mouse_position(ctx);
           
           
           
            if cursor_position.x >= self.one.position.x
                && cursor_position.x <= self.one.position.x + self.one.texture.width() as f32
                && cursor_position.y >= self.one.position.y
                && cursor_position.y <= self.one.position.y + self.one.texture.height() as f32
            {
                // Move player2 100.0 units to the right
                self.player2.position.x += 50.0;

                // Disable further movement of player2
                self.player2_can_move = false;
            }
            // Check for tapping on entity "two" to move player2 150.0 units to the right
            else if cursor_position.x >= self.two.position.x
                && cursor_position.x <= self.two.position.x + self.two.texture.width() as f32
                && cursor_position.y >= self.two.position.y
                && cursor_position.y <= self.two.position.y + self.two.texture.height() as f32
            {
                self.player2.position.x += 150.0;
                self.player2_can_move = false;
            }
            // Check for tapping on entity "three" to move player2 200.0 units to the right
            else if cursor_position.x >= self.three.position.x
                && cursor_position.x <= self.three.position.x + self.three.texture.width() as f32
                && cursor_position.y >= self.three.position.y
                && cursor_position.y <= self.three.position.y + self.three.texture.height() as f32
            {
                self.player2.position.x += 250.0;
                self.player2_can_move = false;
            }
            // Check for tapping on entity "four" to move player2 250.0 units to the right
            else if cursor_position.x >= self.four.position.x
                && cursor_position.x <= self.four.position.x + self.four.texture.width() as f32
                && cursor_position.y >= self.four.position.y
                && cursor_position.y <= self.four.position.y + self.four.texture.height() as f32
            {
                self.player2.position.x += 350.0;
                self.player2_can_move = false;
            }
            // Check for tapping on entity "five" to move player2 300.0 units to the right
            else if cursor_position.x >= self.five.position.x
                && cursor_position.x <= self.five.position.x + self.five.texture.width() as f32
                && cursor_position.y >= self.five.position.y
                && cursor_position.y <= self.five.position.y + self.five.texture.height() as f32
            {
                self.player2.position.x += 450.0;
                self.player2_can_move = false;
            }
            
            
        }
        

        // Move player2 (right paddle) up and down with the Up and Down arrow keys
        if input::is_key_down(ctx, Key::Up) {
            self.player2.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down) {
            self.player2.position.y += PADDLE_SPEED;
        }

        // ... (other updates)

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);
        self.one.texture.draw(ctx, self.one.position);
        // ... (draw other entities)
        self.two.texture.draw(ctx,self.two.position );
        self.three.texture.draw(ctx,self.three.position );
        self.four.texture.draw(ctx,self.four.position );
        self.five.texture.draw(ctx,self.five.position );
        self.reload.texture.draw(ctx, self.reload.position);

        Ok(())
    }
}

struct Entity {
    texture: Texture,
    position: Vec2<f32>,
}

impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity { texture, position }
    }
}
//-----khgsDLHglDGSHljgslcnbCB ---

