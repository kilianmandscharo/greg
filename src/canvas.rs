use chrono::Timelike;
use crossterm::{
    cursor::MoveTo,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io;

use crate::{
    block::Block,
    time_of_day::{StepType, TimeOfDay, TimeOfDayRange},
};

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
            x_offset: offset.0 + 1,
            y_offset: offset.1,
        }
    }

    fn total_minutes(&self) -> u16 {
        self.end_time.total - self.start_time.total
    }

    fn get_x_position_by_tod(&self, time_of_day: &TimeOfDay) -> u16 {
        let minutes_per_col = (self.total_minutes() as f32 / self.width as f32).ceil() as u16;

        (time_of_day.total - self.start_time.total) / minutes_per_col
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

    fn render_grid(&self, max_y: u16) -> io::Result<()> {
        let time_range =
            TimeOfDayRange::new(self.start_time, self.end_time, self.get_step_type()).unwrap();

        for time in time_range {
            let x = self.get_x_position_by_tod(&time);

            for i in 0..max_y + 1 {
                self.print(x, 1 + i, "⎸", Color::White, Color::Reset)?;
            }

            self.print(x, 0, &time.to_string(), Color::White, Color::Reset)?;
        }

        Ok(())
    }

    fn move_cursor_to_end(&self, max_y: u16) -> io::Result<()> {
        io::stdout().execute(MoveTo(0, max_y + self.y_offset + 2))?;
        Ok(())
    }

    pub fn create_block(
        &self,
        start: TimeOfDay,
        end: TimeOfDay,
        title: String,
        description: String,
    ) -> Block {
        let x1 = self.get_x_position_by_tod(&start);
        let x2 = self.get_x_position_by_tod(&end);
        Block::new(start, end, title, description, x1, 0, x2 - x1, 2)
    }

    fn position_blocks(&self, blocks: Vec<Block>) -> Vec<Block> {
        let mut positioned_blocks: Vec<Block> = Vec::new();

        for block in blocks {
            let mut new_block = block.clone();

            loop {
                if new_block.intersects_any(&positioned_blocks) {
                    new_block.y += 1;
                } else {
                    break;
                }
            }

            positioned_blocks.push(new_block);
        }

        positioned_blocks
    }

    fn render_blocks(&self, blocks: Vec<Block>) -> io::Result<()> {
        let positioned_blocks = self.position_blocks(blocks);

        for block in positioned_blocks {
            self.render_block(&block)?;
        }

        Ok(())
    }

    fn render_block(&self, block: &Block) -> io::Result<()> {
        self.print(
            block.x,
            block.y + 2,
            &self.pad_string(&block.start.to_string(), block.width as usize),
            Color::Black,
            Color::Green,
        )?;
        self.print(
            block.x,
            block.y + 3,
            &self.pad_string(&block.title, block.width as usize),
            Color::Black,
            Color::Green,
        )?;

        Ok(())
    }

    fn render_now(&self, max_y: u16) -> io::Result<()> {
        let now = chrono::offset::Local::now();
        let now = TimeOfDay::new(now.hour() as u16, now.minute() as u16);

        let x = self.get_x_position_by_tod(&now);

        for i in 0..max_y + 2 {
            self.print(x, 1 + i, "⎸", Color::Green, Color::Reset)?;
        }

        self.print(x, max_y + 3, &now.to_string(), Color::Green, Color::Reset)?;

        Ok(())
    }

    fn pad_string(&self, s: &str, width: usize) -> String {
        format!("{}{}", s, " ".repeat(width - s.len()))
    }

    pub fn render(&self, blocks: Vec<Block>) -> io::Result<()> {
        let positioned_blocks = self.position_blocks(blocks);

        let max_y = positioned_blocks.iter().map(|block| block.y).max().unwrap()
            + positioned_blocks[0].height
            + 1;

        self.render_grid(max_y)?;
        self.render_now(max_y)?;
        self.render_blocks(positioned_blocks)?;
        self.move_cursor_to_end(max_y)?;

        Ok(())
    }

    fn get_step_type(&self) -> StepType {
        let n_labels = self.width / 6;
        let total = self.total_minutes();

        if total / 15 <= n_labels {
            return StepType::Minute(15);
        }

        if total / 30 <= n_labels {
            return StepType::Minute(30);
        }

        if total / 60 <= n_labels {
            return StepType::Hour(1);
        }

        if total / 120 <= n_labels {
            return StepType::Hour(2);
        }

        StepType::Hour(3)
    }
}
