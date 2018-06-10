use piston_window::*;
use piston_window::types::Color;
use rand::{thread_rng, Rng};

use snake::{Direction, Snake};
use draw::{draw_block, draw_rectangle};

/// Color of food that the player tries to collect with the snake.
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
/// Color of the outer border of the game.
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
/// Gameover color used to overlay the game when the player loses.
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

/// Period between each movement of the snake (in seconds)
const MOVING_PERIOD: f64 = 0.1;
/// Time before game restarts after a gameover (in seconds)
const RESTART_TIME: f64 = 1.0;

pub struct Game {
  snake: Snake,

  food_exists: bool,
  food_x: i32,
  food_y: i32,

  width: i32,
  height: i32,

  game_over: bool,
  waiting_time: f64,
}

impl Game {
  pub fn new(width: i32, height: i32) -> Game {
    Game {
      snake: Snake::new(2, 2),
      waiting_time: 0.0,
      food_exists: true,
      food_x: 6,
      food_y: 4,
      width,
      height,
      game_over: false,
    }
  }

  /// Checks a pressed key and updates the snake to move according to
  /// the pressed key.
  pub fn key_pressed(&mut self, key: Key) {
    // if the game is over, don't let the user control the snake.
    if self.game_over {
      return;
    }

    let dir = match key {
      Key::Up => Some(Direction::Up),
      Key::Down => Some(Direction::Down),
      Key::Left => Some(Direction::Left),
      Key::Right => Some(Direction::Right),
      _ => None
    };

    // prevents the user from telling the snake to go backwards.
    if dir.unwrap() == self.snake.head_direction().opposite() {
      return;
    }

    self.update_snake(dir);
  }

  /// Draw the elements in the game.
  pub fn draw(&self, con: &Context, g: &mut G2d) {
    // draw snake
    self.snake.draw(con, g);

    // draw food
    if self.food_exists {
      draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
    }

    // draw borders
    draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
    draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
    draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
    draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

    // check for gameover and draw the gameover overlay.
    if self.game_over {
      draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
    }
  }

  pub fn update(&mut self, delta_time: f64) {
    self.waiting_time += delta_time;

    // check if game is over. If it is, wait until RESTART_TIME has passed, then restart.
    if self.game_over {
      if self.waiting_time > RESTART_TIME {
        self.restart();
      }
      return;
    }

    // Check if there is food. if not, add food to the game.
    if !self.food_exists {
      self.add_food();
    }

    // if enough time has passed since the last time the snake moved, then update the snake.
    if self.waiting_time > MOVING_PERIOD {
      self.update_snake(None);
    }
  }

  /// Checks if the snake's head is at the same location as the food.
  /// If it is, then the snake grows.
  fn is_eating(&mut self) {
    let (head_x, head_y): (i32, i32) = self.snake.head_position();
    if self.food_exists && self.food_x == head_x && self.food_y == head_y {
      self.food_exists = false;
      self.snake.restore_tail();
    }
  }

  /// Checks if the snake has hit a wall or its tail, returning false if so.
  fn is_snake_alive(&self, dir: Option<Direction>) -> bool {
    let (next_x, next_y) = self.snake.next_head(dir);

    if self.snake.overlap_tail(next_x, next_y) {
      return false;
    }

    // return false if the snake hits a wall or is outside the game area.
    next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
  }

  /// Adds food to the game for the snake to eat.
  fn add_food(&mut self) {
    // randomly generate a position for the food.
    let mut rng = thread_rng();
    let mut new_x = rng.gen_range(1, self.width - 1);
    let mut new_y = rng.gen_range(1, self.height - 1);

    // keep generating new positions until we find one that is not inside the snake.
    while self.snake.overlap_tail(new_x, new_y) {
      new_x = rng.gen_range(1, self.width - 1);
      new_y = rng.gen_range(1, self.height - 1);
    }

    // set the food in the game.
    self.food_x = new_x;
    self.food_y = new_y;
    self.food_exists = true;
  }

  /// Updates the snake in the game.
  fn update_snake(&mut self, dir: Option<Direction>) {
    if self.is_snake_alive(dir) {
      self.snake.move_forward(dir);
      self.is_eating();
    } else {
      self.game_over = true;
    }
    self.waiting_time = 0.0;
  }

  /// Restarts the game.
  fn restart(&mut self) {
    self.snake = Snake::new(2, 2);
    self.waiting_time = 0.0;
    self.food_exists = true;
    self.food_x = 6;
    self.food_y = 4;
    self.game_over = false;
  }
}