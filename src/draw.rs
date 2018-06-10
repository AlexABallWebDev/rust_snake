use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

/// factor to multiply blocks by. Also the side length of a block.
const BLOCK_SIZE: f64 = 25.0;

/// returns the given coordinate as a f64 multiplied by BLOCK_SIZE.
pub fn to_coord(game_coord: i32) -> f64 {
  (game_coord as f64) * BLOCK_SIZE
}

/// returns the given coordinate as a u32 multiplied by BLOCK_SIZE.
pub fn to_coord_u32(game_coord: i32) -> u32 {
  to_coord(game_coord) as u32
}

/// Draw a block at the given coordinates.
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
  let gui_x = to_coord(x);
  let gui_y = to_coord(y);

  rectangle(
    color,
    // coordinates of the rectangle, along with width and height.
    [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
    con.transform,
    g,
  );
}

/// Draw a rectangle at the given coordinates.
pub fn draw_rectangle(
  color: Color,
  x: i32,
  y: i32,
  width: i32,
  height: i32,
  con: &Context,
  g: &mut G2d,
) {
  let gui_x = to_coord(x);
  let gui_y = to_coord(y);

  rectangle(
    color,
    // coordinates of the rectangle, along with width and height.
    [
      gui_x,
      gui_y,
      BLOCK_SIZE * (width as f64),
      BLOCK_SIZE * (height as f64),
    ],
    con.transform,
    g,
  );
}
