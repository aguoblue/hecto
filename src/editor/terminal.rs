use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Error, Write};

#[derive(Copy, Clone)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { height, width })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}


// queue! 按行缓冲 Queues one or more command(s) for further execution.
// When flush is called manually on the given type implementing io::Write.
// The terminal will flush automatically if the buffer is full.
// Each line is flushed in case of stdout, because it is line buffered.

// flush 后 终端解析这些ANSI序列，包括\r\n