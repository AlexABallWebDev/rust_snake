use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use draw::draw_block;

/// Snake's color. A Color is defined as an array of RGB + opacity f64 values.
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

/// Represents the directions the snake can move.
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

/// Represents Direction's methods
impl Direction {
  /// Returns the opposite Direction of this Direction.
  pub fn opposite(&self) -> Direction {
    match *self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }
}

/// Represents a block of the snake.
#[derive(Clone, Debug)]
struct Block {
  x: i32,
  y: i32,
}

/// Represents the snake.
pub struct Snake {
  direction: Direction,
  body: LinkedList<Block>,
  tail: Option<Block>,
}

/// Represents Snake's methods
impl Snake {
  /// Create a new Snake at the given coordinates. The snake will face to the right.
  pub fn new(x: i32, y: i32) -> Snake {
    // create the snake's inital body based on the given coordinates.
    let mut body: LinkedList<Block> = LinkedList::new();
    body.push_back(Block { x: x + 2, y });
    body.push_back(Block { x: x + 1, y });
    body.push_back(Block { x: x, y });

    // return the Snake.
    Snake {
      direction: Direction::Right,
      body,
      tail: None,
    }
  }

  /// Draw the snake.
  pub fn draw(&self, con: &Context, g: &mut G2d) {
    for block in &self.body {
      draw_block(SNAKE_COLOR, block.x, block.y, con, g);
    }
  }

  /// Returns the position (x, y) of the snake's head.
  pub fn head_position(&self) -> (i32, i32) {
    // unwrap will get the front element out of the Option.
    let head_block = self.body.front().unwrap();
    (head_block.x, head_block.y)
  }

  /// Move the snake forward.
  pub fn move_forward(&mut self, dir: Option<Direction>) {
    match dir {
      Some(d) => self.direction = d,
      None => (),
    }

    let (last_x, last_y): (i32, i32) = self.head_position();

    // new head block
    let new_block = match self.direction {
      Direction::Up => Block {
        x: last_x,
        y: last_y - 1,
      },
      Direction::Down => Block {
        x: last_x,
        y: last_y + 1,
      },
      Direction::Left => Block {
        x: last_x - 1,
        y: last_y,
      },
      Direction::Right => Block {
        x: last_x + 1,
        y: last_y,
      },
    };
    self.body.push_front(new_block);

    // remove the tail block and store it in self.tail
    let removed_block = self.body.pop_back().unwrap();
    self.tail = Some(removed_block);
  }

  /// Returns a copy of the snake's direction
  pub fn head_direction(&self) -> Direction {
    // return a copy of the snake's Direction.
    self.direction
  }

  /// Returns the coordinates of the next head of the snake. This is the
  /// location that a new head will appear.
  pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
    let (head_x, head_y): (i32, i32) = self.head_position();

    // If no direction is given, then the snake will not change direction.
    let mut moving_dir = self.direction;
    match dir {
      Some(d) => moving_dir = d,
      None => {}
    }

    match moving_dir {
      Direction::Up => (head_x, head_y - 1),
      Direction::Down => (head_x, head_y + 1),
      Direction::Left => (head_x - 1, head_y),
      Direction::Right => (head_x + 1, head_y),
    }
  }

  /// Adds the saved tail back onto the end of the Snake. Used to extend
  /// the snake when it eats a red dot.
  pub fn restore_tail(&mut self) {
    let tail_block = self.tail.clone().unwrap();
    self.body.push_back(tail_block)
  }

  /// Check if the snake's body is overlapping with the given location.
  pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
    let mut ch = 0;
    for block in &self.body {
      // return true if the given location overlaps with part of the snake.
      if x == block.x && y == block.y {
        return true;
      }

      // resolves issue where snake moves into its tail at the same time 
      // the tail moves away from the head. This should not result in a game over.
      ch += 1;
      if ch == self.body.len() - 1 {
        break;
      }
    }
    return false;
  }
}
