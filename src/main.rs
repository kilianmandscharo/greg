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

    let start = TimeOfDay::new(0, 0);
    let end = TimeOfDay::new(23, 59);

    let canvas = Canvas::new(start, end, size()?, cursor::position()?);

    let blocks = vec![
        canvas.create_block(
            TimeOfDay::new(13, 0),
            TimeOfDay::new(15, 0),
            "Lunch".to_string(),
            "Lunch at the café".to_string(),
        ),
        canvas.create_block(
            TimeOfDay::new(14, 0),
            TimeOfDay::new(16, 0),
            "Dentist".to_string(),
            "Dentist appointment".to_string(),
        ),
        canvas.create_block(
            TimeOfDay::new(15, 30),
            TimeOfDay::new(17, 30),
            "Football match".to_string(),
            "Football match at the local club".to_string(),
        ),
        canvas.create_block(
            TimeOfDay::new(15, 00),
            TimeOfDay::new(19, 00),
            "Server backup".to_string(),
            "Server backup is running".to_string(),
        ),
    ];

    canvas.render(blocks)?;

    disable_raw_mode()?;

    Ok(())
}
