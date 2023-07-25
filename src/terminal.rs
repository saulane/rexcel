use crossterm::event::{read, Event};
use crossterm::terminal;
use crossterm::{execute, ErrorKind};

use crate::Position;
use std::io::{stdout, Write};

#[derive(Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub struct Terminal {
    pub size: Size,
}

impl Terminal {
    pub fn new() -> Result<Self, ErrorKind> {
        let size = terminal::size()?;
        Ok(Self {
            size: Size {
                width: size.0 as usize,
                height: size.1.saturating_sub(4) as usize,
            },
        })
    }

    pub fn enter() {
        terminal::enable_raw_mode().unwrap();
        execute!(stdout(), terminal::EnterAlternateScreen).unwrap();
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn update_size(&mut self, width: usize, height: usize) -> Result<(), ErrorKind> {
        let new_size = Size {
            width,
            height: height.saturating_sub(4),
        };
        write!(stdout(), "{}|{}", width, height)?;
        self.size = new_size;
        Ok(())
    }

    pub fn leave() {
        terminal::disable_raw_mode().unwrap();
        execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
        println!("Goodbye!");
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn show_cursor() {
        execute!(stdout(), crossterm::cursor::Show).unwrap();
    }

    pub fn hide_cursor() {
        execute!(stdout(), crossterm::cursor::Hide).unwrap();
    }

    pub fn clear() {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    pub fn clear_line() {
        execute!(stdout(), terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
    }

    pub fn set_fg_color(color: crossterm::style::Color) {
        execute!(stdout(), crossterm::style::SetForegroundColor(color)).unwrap();
    }

    pub fn set_bg_color(color: crossterm::style::Color) {
        execute!(stdout(), crossterm::style::SetBackgroundColor(color)).unwrap();
    }

    pub fn reset_colors() {
        execute!(stdout(), crossterm::style::ResetColor).unwrap();
    }

    pub fn read_event() -> Result<Event, ErrorKind> {
        loop {
            if let Ok(event) = read() {
                return Ok(event);
            }
        }
    }

    pub fn goto(p: &Position) {
        execute!(stdout(), crossterm::cursor::MoveTo(p.x as u16, p.y as u16)).unwrap();
    }
}
