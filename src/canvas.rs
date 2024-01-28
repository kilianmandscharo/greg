use chrono::Timelike;
use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io;

use crate::{block::Block, time_of_day::TimeOfDay};

pub struct Canvas {
    width: u16,
    _height: u16,
    y_offset: u16,
    x_offset: u16,
    start_time: TimeOfDay,
    end_time: TimeOfDay,
}

impl Canvas {
    pub fn new(
        start_time: TimeOfDay,
        end_time: TimeOfDay,
        dimensions: (u16, u16),
        offset: (u16, u16),
    ) -> Self {
        Self {
            start_time,
            end_time,
            width: dimensions.0,
            _height: dimensions.1,
            x_offset: offset.0,
            y_offset: offset.1,
        }
    }

    fn total_minutes(&self) -> u16 {
        self.end_time.total_minutes() - self.start_time.total_minutes()
    }

    fn get_x_position_by_tod(&self, time_of_day: &TimeOfDay) -> u16 {
        let minutes_per_col = self.total_minutes() / self.width;

        (time_of_day.total_minutes() - self.start_time.total_minutes()) / minutes_per_col
    }

    fn print(&self, x: u16, y: u16, content: &str, fg: Color, bg: Color) -> io::Result<()> {
        io::stdout()
            .execute(MoveTo(x + self.x_offset, y + self.y_offset))?
            .execute(SetForegroundColor(fg))?
            .execute(SetBackgroundColor(bg))?
            .execute(Print(content))?
            .execute(ResetColor)?;

        Ok(())
    }

    pub fn render_grid(&self) -> io::Result<()> {
        for hour in self.start_time.hour..=self.end_time.hour {
            let time = TimeOfDay::new(hour, 0);
            let x = self.get_x_position_by_tod(&time);

            for i in 0..3 {
                self.print(x, 1 + i, "|", Color::White, Color::Reset)?;
            }

            self.print(x, 0, &time.to_string(), Color::White, Color::Reset)?;
        }

        // let now = chrono::offset::Local::now();
        // let now = TimeOfDay::new(now.hour() as u16, now.minute() as u16);

        let now = TimeOfDay::new(15, 45);

        let x = self.get_x_position_by_tod(&now);

        for i in 0..4 {
            self.print(x, 2 + i, "|", Color::Green, Color::Reset)?;
        }

        self.print(x, 5, &now.to_string(), Color::Green, Color::Reset)?;

        Ok(())
    }

    pub fn move_cursor_to_end(&self) -> io::Result<()> {
        io::stdout().execute(MoveTo(0, 6 + self.y_offset))?;
        Ok(())
    }

    pub fn render_block(&self, block: &Block) -> io::Result<()> {
        let x = self.get_x_position_by_tod(&block.start);

        self.print(x, 1, &block.title, Color::Black, Color::Green)?;

        for i in 0..4 {
            self.print(x, 2 + i, "|", Color::Green, Color::Reset)?;
        }

        self.print(x, 5, &block.start.to_string(), Color::Green, Color::Reset)?;

        Ok(())
    }
}
