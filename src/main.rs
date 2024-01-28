use block::Block;
use canvas::Canvas;
use crossterm::{
    cursor,
    terminal::{disable_raw_mode, enable_raw_mode, size},
};
use std::io;
use time_of_day::TimeOfDay;

mod block;
mod canvas;
mod time_of_day;

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let start = TimeOfDay::new(8, 0);
    let end = TimeOfDay::new(20, 0);

    let canvas = Canvas::new(start, end, size()?, cursor::position()?);

    canvas.render_grid()?;

    let block = Block::new(
        TimeOfDay::new(13, 0),
        TimeOfDay::new(15, 0),
        "Lunch".to_string(),
        "Lunch at the café".to_string(),
    );

    canvas.render_block(&block)?;

    canvas.move_cursor_to_end()?;

    disable_raw_mode()?;

    Ok(())
}
