
// use tetra::graphics::{self, Color, Texture};
// use tetra::input::{self, MouseButton, Key};
// use tetra::math::Vec2;
// use tetra::{Context, ContextBuilder, State};

// const PADDLE_SPEED: f32 = 4.0;
// const WINDOW_WIDTH: f32 = 1000.0;
// const WINDOW_HEIGHT: f32 = 1000.0;

// fn main() -> tetra::Result {
//     ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
//         .quit_on_escape(true)
//         .show_mouse(true)
//         .build()?
//         .run(GameState::new)
// }

// struct GameState {
//     player: Entity,
//     twentyblue: Entity,
//     fourtyblue: Entity,
//     twentyred: Entity,
//     fourtyred: Entity,
//     can_move_twentyred: bool,
// }

// impl GameState {
//     fn new(ctx: &mut Context) -> tetra::Result<GameState> {
//         let player_texture = Texture::new(ctx, "./resources/object.png")?;
//         let player_position = Vec2::new(
//             (WINDOW_WIDTH - player_texture.width() as f32) / 2.0,
//             (WINDOW_HEIGHT - player_texture.height() as f32) / 2.0,
//         );

//         let twentyblue_texture = Texture::new(ctx, "./resources/20Nblue.png")?;
//         let twentyblue_position = Vec2::new(
//             WINDOW_WIDTH - twentyblue_texture.width() as f32 - 500.0,
//             (WINDOW_HEIGHT - twentyblue_texture.height() as f32) / 8.0,
//         );

//         let twentyred_texture = Texture::new(ctx, "./resources/20NRed.png")?;
//         let twentyred_position = Vec2::new(
//             WINDOW_WIDTH - twentyred_texture.width() as f32 - 400.0,
//             (WINDOW_HEIGHT - twentyred_texture.height() as f32) / 8.0,
//         );
//         let fourtyblue_texture = Texture::new(ctx, "./resources/40Nblue.png")?;
//         let fourtyblue_position = Vec2::new(
//             WINDOW_WIDTH - fourtyblue_texture.width() as f32 - 300.0,
//             (WINDOW_HEIGHT - fourtyblue_texture.height() as f32) / 8.0,
//         );
//         let fourtyred_texture = Texture::new(ctx, "./resources/40NRed.png")?;
//         let fourtyred_position = Vec2::new(
//             WINDOW_WIDTH - fourtyred_texture.width() as f32 - 200.0,
//             (WINDOW_HEIGHT - fourtyred_texture.height() as f32) / 8.0,
//         );

//         Ok(GameState {
//             player: Entity::new(player_texture, player_position),
//             twentyblue: Entity::new(twentyblue_texture, twentyblue_position),
//             twentyred: Entity::new(twentyred_texture, twentyred_position),
//             fourtyblue: Entity::new(fourtyblue_texture, fourtyblue_position),
//             fourtyred: Entity::new(fourtyred_texture, fourtyred_position),
//             can_move_twentyred: true,
//         })
//     }
// }

// impl State for GameState {
//     fn update(&mut self, ctx: &mut Context) -> tetra::Result {
//         let cursor_position = input::get_mouse_position(ctx);
//         // Check for mouse input events (left click)
//         if cursor_position.x >= self.twentyred.position.x
//         && cursor_position.x <= self.twentyred.position.x + self.twentyred.texture.width() as f32
//         && cursor_position.y >= self.twentyred.position.y
//         && cursor_position.y <= self.twentyred.position.y + self.twentyred.texture.height() as f32
//     {
//         // Move player2 100.0 units to the right
//         if input::is_key_down(ctx, Key::Left) {
//             self.player.position.x -= PADDLE_SPEED;
//         }

//         // Clamp the player's paddle position to the screen bounds
//         self.player.position.x = self.player.position.x.max(0.0).min(WINDOW_WIDTH - self.player.texture.width() as f32);

//         // Disable further movement of player2
        
//     }
//     else  if cursor_position.x >= self.twentyblue.position.x
//     && cursor_position.x <= self.twentyblue.position.x + self.twentyblue.texture.width() as f32
//     && cursor_position.y >= self.twentyblue.position.y
//     && cursor_position.y <= self.twentyblue.position.y + self.twentyblue.texture.height() as f32
// {
//     // Move player2 100.0 units to the right
//     if input::is_key_down(ctx, Key::Right) {
//         self.player.position.x += PADDLE_SPEED;
//     }

//     // Clamp the player's paddle position to the screen bounds
//     self.player.position.x = self.player.position.x.max(0.0).min(WINDOW_WIDTH - self.player.texture.width() as f32);

//     // Disable further movement of player2
// }


//         //------------------------------------------
     
//         // Update the player's paddle position based on the movement direction
     

//         Ok(())
//     }

//     fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
//         graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

//         self.player.texture.draw(ctx, self.player.position);
//         self.twentyblue.texture.draw(ctx, self.twentyblue.position);
//         self.twentyred.texture.draw(ctx, self.twentyred.position);
//         self.fourtyblue.texture.draw(ctx, self.fourtyblue.position);
//         self.fourtyred.texture.draw(ctx, self.fourtyred.position);

//         Ok(())
//     }
// }

// struct Entity {
//     texture: Texture,
//     position: Vec2<f32>,
// }

// impl Entity {
//     fn new(texture: Texture, position: Vec2<f32>) -> Entity {
//         Entity { texture, position }
//     }
// }


use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, MouseButton, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};


const PADDLE_SPEED: f32 = 4.0;
const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 1000.0;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?
        .run(GameState::new)
}

struct GameState {
    player: Entity,
    twentyblue: Entity,
    twentyred: Entity,
    fourtyblue: Entity,
    fourtyred: Entity,
    can_move_twentyred: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player_texture = Texture::new(ctx, "./resources/object.png")?;
        let player_position = Vec2::new(
            (WINDOW_WIDTH - player_texture.width() as f32) / 2.0,
            (WINDOW_HEIGHT - player_texture.height() as f32) / 2.0,
        );

        let twentyblue_texture = Texture::new(ctx, "./resources/20Nblue.png")?;
        let twentyblue_position = Vec2::new(
            WINDOW_WIDTH - twentyblue_texture.width() as f32 - 500.0,
            (WINDOW_HEIGHT - twentyblue_texture.height() as f32) / 8.0,
        );

        let twentyred_texture = Texture::new(ctx, "./resources/20NRed.png")?;
        let twentyred_position = Vec2::new(
            WINDOW_WIDTH - twentyred_texture.width() as f32 - 400.0,
            (WINDOW_HEIGHT - twentyred_texture.height() as f32) / 8.0,
        );
        let fourtyblue_texture = Texture::new(ctx, "./resources/40Nblue.png")?;
        let fourtyblue_position = Vec2::new(
            WINDOW_WIDTH - fourtyblue_texture.width() as f32 - 300.0,
            (WINDOW_HEIGHT - fourtyblue_texture.height() as f32) / 8.0,
        );
        let fourtyred_texture = Texture::new(ctx, "./resources/40NRed.png")?;
        let fourtyred_position = Vec2::new(
            WINDOW_WIDTH - fourtyred_texture.width() as f32 - 200.0,
            (WINDOW_HEIGHT - fourtyred_texture.height() as f32) / 8.0,
        );

        Ok(GameState {
            player: Entity::new(player_texture, player_position),
            twentyblue: Entity::new(twentyblue_texture, twentyblue_position),
            twentyred: Entity::new(twentyred_texture, twentyred_position),
            fourtyblue: Entity::new(fourtyblue_texture, fourtyblue_position),
            fourtyred: Entity::new(fourtyred_texture, fourtyred_position),
            can_move_twentyred: true,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let cursor_position = input::get_mouse_position(ctx);

        // Check for mouse input events (left click)
        if cursor_position.x >= self.twentyred.position.x
            && cursor_position.x <= self.twentyred.position.x + self.twentyred.texture.width() as f32
            && cursor_position.y >= self.twentyred.position.y
            && cursor_position.y <= self.twentyred.position.y + self.twentyred.texture.height() as f32
        {
            // Move player2 100.0 units to the left
            if input::is_key_down(ctx, Key::Left) {
                self.player.position.x -= PADDLE_SPEED;
            }

            // Clamp the player's paddle position to the left half of the screen
            self.player.position.x = self.player.position.x.max(0.0).min(WINDOW_WIDTH / 2.0);

            // Disable further movement of player2
            self.can_move_twentyred = false;
        } else if cursor_position.x >= self.twentyblue.position.x
            && cursor_position.x <= self.twentyblue.position.x + self.twentyblue.texture.width() as f32
            && cursor_position.y >= self.twentyblue.position.y
            && cursor_position.y <= self.twentyblue.position.y + self.twentyblue.texture.height() as f32
        {
            // Move player2 100.0 units to the right
            if input::is_key_down(ctx, Key::Right) {
                self.player.position.x += PADDLE_SPEED;
            }

            // Clamp the player's paddle position to the right half of the screen
            self.player.position.x = self.player.position.x.max(WINDOW_WIDTH / 2.0).min(WINDOW_WIDTH - self.player.texture.width() as f32);

            // Disable further movement of player2
            self.can_move_twentyred = true;
        }

        // Calculate the center of the player's paddle
        let player_center_x = self.player.position.x + self.player.texture.width() as f32 / 2.0;

        // Check if the player is on the left half of its length
        if player_center_x < WINDOW_WIDTH / 5.0 {
            println!("Red player win!!!!!!!!!!");
        }
        // Check if the player is on the right half of its length
     else if player_center_x > WINDOW_WIDTH / 9.0{
           println!("Blue player win!!!!!!!!")
       }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.player.texture.draw(ctx, self.player.position);
        self.twentyblue.texture.draw(ctx, self.twentyblue.position);
        self.twentyred.texture.draw(ctx, self.twentyred.position);
        self.fourtyblue.texture.draw(ctx, self.fourtyblue.position);
        self.fourtyred.texture.draw(ctx, self.fourtyred.position);

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
