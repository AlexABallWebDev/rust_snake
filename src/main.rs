/// Author: Alex Ball
/// 
/// Snake game written in Rust. Created using a tutorial by 
/// Tensor Programming:
/// https://www.youtube.com/watch?v=DnT_7M7L7vo&t=1s

// get access to our dependencies (cargo crates)
extern crate piston_window;
extern crate rand;

// tell compiler to link our other source files to this file.
mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use draw::to_coord_u32;
use game::Game;

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

/// Main function that runs the game.
pub fn main() {
  let (width, height) = (20, 20);

  // create the window that the game will be displayed in.
  let mut window: PistonWindow = WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
      // tells the window to exit if esc key is pressed
      .exit_on_esc(true)
      // build the window
      .build()
      // unwrap the window from the option returned by build()
      .unwrap();

  let mut game = Game::new(width, height);

  // main game loop. Get the next event from the window repeatedly.
  while let Some(event) = window.next() {
    // check for input from the keyboard and relay the key pressed to the game.
    if let Some(Button::Keyboard(key)) = event.press_args() {
      game.key_pressed(key);
    }

    // update the game display (draw to the window).
    // The |c, g| syntax signifies an anonymous function. In this case,
    // draw_2d() is calling this anonymous function and telling game
    // to draw using the given context c and g2d g.
    window.draw_2d(&event, |c, g| {
      clear(BACKGROUND_COLOR, g);
      game.draw(&c, g);
    });

    // update the game state (update the snake, add food, check for gameover, etc.).
    event.update(|arg| {
      game.update(arg.dt);
    });
  }
}
