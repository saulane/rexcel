use crossterm::event::{Event, KeyCode, KeyModifiers, KeyEvent};
use crossterm::ErrorKind;
use crossterm::style::Color;

use std::io::{Write, stdout};
use std::env;

use crate::Terminal;
use crate::Document;


pub struct Position{
    pub x: usize,
    pub y: usize
}

pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

impl Position{
    pub fn shift(&mut self, dir: Direction){
        match dir{
            Direction::Up => self.y.saturating_sub(1),
            Direction::Down => self.y.saturating_add(1),
            Direction::Left => self.y.saturating_sub(1),
            Direction::Right => self.y.saturating_add(1),
        };
    }
}

enum EditorMode{
    Edit,
    Move,
}

pub struct Editor{
    pub terminal: Terminal,
    pub cursor_position: Position,
    pub cell_position: Position,
    pub document: Document,
    mode: EditorMode,
    quit: bool
}

impl Editor{
    pub fn new() -> Result<Self, ErrorKind>{
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1{
            let filename = &args[1];
            let doc = Document::open(&filename);
            if doc.is_ok(){
                doc.unwrap()
            }else{
                Document::default()
            }
        }else{
            Document::default()
        };

        Ok(Self{
            terminal: Terminal::new()?,
            cursor_position: Position{x:5,y:3},
            cell_position: Position{x:0,y:0},
            document,
            mode: EditorMode::Move,
            quit: false
        })
    }

    pub fn run(&mut self){
        Terminal::enter();

        while !self.quit {
            if let Err(error) = self.update(){
                die(error);
            }

            if let Err(error) = self.process_input() {
                die(error);
            }
        }

        Terminal::leave();
    }

    fn process_input(&mut self) -> Result<(), ErrorKind>{
        match Terminal::read_key()?{
            Event::Key(KeyEvent{code: KeyCode::Down, modifiers: _}) => {
                self.move_cursor(KeyCode::Down);
            },
            Event::Key(KeyEvent{code: KeyCode::Up, modifiers: _})  => {
                self.move_cursor(KeyCode::Up);
            },
            Event::Key(KeyEvent{code: KeyCode::Right, modifiers: _})  => {
                self.move_cursor(KeyCode::Right);
            },
            Event::Key(KeyEvent{code: KeyCode::Left, modifiers: _})  => {
                self.move_cursor(KeyCode::Left);
            },
            Event::Key(KeyEvent{code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL}) => {
                self.quit();
            },
            Event::Key(KeyEvent{code: KeyCode::Char('s'), modifiers: KeyModifiers::CONTROL}) => {
                self.document.save()?;
            },
            Event::Key(KeyEvent{code: KeyCode::Backspace, modifiers: _}) => {
                self.document.delete(&self.cell_position);
            },
            Event::Key(KeyEvent{code: KeyCode::Char(c), modifiers: _}) => {
                self.document.insert(&self.cell_position, c);
            },
            Event::Key(KeyEvent{code: KeyCode::Delete, modifiers: _}) => {
                if self.document.cell_exist(&self.cell_position){
                    self.document.rows[self.cell_position.y].cells[self.cell_position.x].reset()
                }
            },
            _ => ()
        }

        Ok(())
    }

    fn update(&mut self) -> Result<(), std::io::Error>{
        Terminal::hide_cursor();
        Terminal::goto(&Position{x:0, y:0});
        Terminal::clear_line();
        if let EditorMode::Edit = self.mode{
            write!(stdout(), "Edit: {}", self.document.rows[self.cell_position.y].cells[self.cell_position.x].val)?;
        }else{
            write!(stdout(), "")?;
        }
        
        Terminal::goto(&Position{x:0, y:1});
        Terminal::clear_line();
        // write!(stdout(), "{}/{}", self.cell_position.x, self.cell_position.y)?;
        // Terminal::clear();
        self.draw_edit_line()?;
        Terminal::goto(&Position{x:0, y:2});

        self.draw_grid()?;

        // Terminal::goto(&self.cursor_position);
        // Terminal::show_cursor();
        Terminal::flush()
    }

    fn quit(&mut self){
        self.quit = true;
    }

    fn move_cursor(&mut self, key: KeyCode){
        match key{
            KeyCode::Left => {
                // if self.cursor_position.x > BORDER_X{
                //     x = x.saturating_sub(10)
                // }

                self.cell_position.x = self.cell_position.x.saturating_sub(1);
            },
            KeyCode::Right => {
                // x = x.saturating_add(10);
                self.cell_position.x = self.cell_position.x.saturating_add(1);
            },
            KeyCode::Up => {
                // if self.cursor_position.y > 3{
                //     y = y.saturating_sub(2)
                // }
                self.cell_position.y = self.cell_position.y.saturating_sub(1);

            },
            KeyCode::Down => {
                // y = y.saturating_add(2);
                self.cell_position.y = self.cell_position.y.saturating_add(1);
            },
            _ => ()
        }

        // self.cursor_position = Position{x, y};
    }

    fn draw_edit_line(&mut self) -> Result<(), std::io::Error>{
        let curr_pos: &Position = &self.cell_position;
        match self.document.cell_exist(curr_pos){
            true => write!(stdout(), "{}", self.document.get_cell(curr_pos).unwrap().val),
            false => write!(stdout(), ""),
        }
    }

    fn draw_cell(&mut self, p: &Position) -> Result<(), std::io::Error>{
        if self.cell_position.x == p.x && self.cell_position.y == p.y{
            Terminal::set_bg_color(Color::White);
            Terminal::set_fg_color(Color::Black);
        }

        if self.document.cell_exist(p){
            let content = &self.document.rows[p.y].cells[p.x].val;
            let len = content.len();
            let margin_right: usize = (9 as usize).saturating_sub(len);
            if content.len() > 9{
                write!(stdout(), "{}{}", &content, &" ".repeat(margin_right))?;
            }else{
                write!(stdout(), "{}{}", &content, &" ".repeat(margin_right))?;
            }
        }else{
            write!(stdout(), "{}", &" ".repeat(9))?;
        }

        Terminal::reset_colors();
        Ok(())
    }

    fn draw_row(&mut self, y: usize) -> Result<(), std::io::Error>{
        if self.cell_position.y != y {
            Terminal::set_bg_color(Color::White);
            Terminal::set_fg_color(Color::Black);
        }
        write!(stdout(), "      ")?;
        Terminal::reset_colors();

        for x in 0..10{
            self.draw_cell(&Position{x,y})?;
        }

        if self.cell_position.y != y {
            Terminal::set_bg_color(Color::White);
            Terminal::set_fg_color(Color::Black);
        }
        write!(stdout(), "\r")?;
        write!(stdout(), "  {}\r\n", &y)?;
        Terminal::reset_colors();
        Ok(())
    }

    fn draw_grid(&mut self) -> Result<(), std::io::Error>{
        let size = self.terminal.size();
        let col_num = size.width.saturating_sub(5)/10;
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        write!(stdout(), "      ")?;
        for i in 0..col_num{
            if i == self.cell_position.x{
                Terminal::set_bg_color(Color::Black);
                Terminal::set_fg_color(Color::White);
                write!(stdout(), "    {}    ", &alphabet.chars().collect::<Vec<char>>()[i])?;
            }else{
                Terminal::set_bg_color(Color::White);
                Terminal::set_fg_color(Color::Black);
                write!(stdout(), "    {}    ", &alphabet.chars().collect::<Vec<char>>()[i])?;
            }
        }
        write!(stdout(), "\r\n")?;

        Terminal::reset_colors();

        for i in 0..size.height{
            Terminal::clear_line();
            self.draw_row(i)?;
            // write!(stdout(), "\u{2502} {}\r\n", i/2)?;
        }

        Ok(())
    }
}

fn die(e: std::io::Error){
    Terminal::clear();
    panic!("{}", e);
}