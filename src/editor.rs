use crossterm::event::{Event, KeyCode, KeyModifiers, KeyEvent};
use crossterm::ErrorKind;
use crossterm::style::Color;

use std::cmp::max;
use std::io::{Write, stdout};
use std::env;

use crate::Terminal;
use crate::Document;
use crate::Cell;

#[derive(Default, Clone, Copy)]
pub struct Position{
    pub x: usize,
    pub y: usize
}

#[derive(PartialEq, Clone, Copy)]
pub enum SearchDirection{
    Forward,
    Backward
}

#[derive(Default)]
pub struct Status{
    pub message: String,
}

impl Status{
    fn from(s: String) -> Self {
        Self{
            message: s,
        }
    }
}

pub struct Editor{
    pub terminal: Terminal,
    pub cursor_position: Position,
    pub cell_position: Position,
    pub document: Document,
    clipboard: Option<Cell>,
    offset: Position,
    header: bool,
    status: Status,
    quit: bool
}

impl Editor{
    pub fn new() -> Result<Self, ErrorKind>{
        let args: Vec<String> = env::args().collect();
        let header: bool = args.contains(&"--header".to_string());
        let status: Status = Status::default();
        let document = if args.len() > 1{
            let filename = &args[1];
            if let Ok(doc) = Document::open(&filename){
                doc
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
            offset: Position::default(),
            clipboard: None,
            header,
            status,
            quit: false,
        })
    }

    pub fn run(&mut self){
        Terminal::enter();
        while !self.quit {
            if let Err(error) = self.update(){
                die(&error);
            }

            if let Err(error) = self.process_input() {
                die(&error);
            }
        }

        Terminal::leave();
    }

    fn process_input(&mut self) -> Result<(), ErrorKind>{
        let curr_cell = self.cell_position;
        match Terminal::read_event()?{
            Event::Resize(width, height) => {
                self.terminal.update_size(width as usize, height as usize)?;
            },
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
            Event::Key(KeyEvent{code: KeyCode::Char('f'), modifiers: KeyModifiers::CONTROL}) => self.search(),
            Event::Key(KeyEvent{code: KeyCode::Char('x'), modifiers: KeyModifiers::CONTROL}) => self.cut(&curr_cell),
            Event::Key(KeyEvent{code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL}) => self.copy(&curr_cell),
            Event::Key(KeyEvent{code: KeyCode::Char('v'), modifiers: KeyModifiers::CONTROL}) => self.paste(&curr_cell),
            Event::Key(KeyEvent{code: KeyCode::Backspace, modifiers: _}) => {
                self.document.delete(&self.cell_position);
            },
            Event::Key(KeyEvent{code: KeyCode::Char(c), modifiers}) => {
                if c == 's' && modifiers.contains(KeyModifiers::CONTROL){
                    if modifiers.contains(KeyModifiers::ALT){
                        self.save_as();
                    }else{
                        self.save();
                    }
                    return Ok(());
                }
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

    fn save(&mut self){
        if self.document.file_name.is_none(){
            let new_name = self.prompt("Save as: ", |_,_,_|{}).unwrap_or(None);
            if new_name.is_none(){
                self.status = Status::from("Canceled.".to_string());
                return;
            }

            self.document.file_name = new_name;
        }

        if self.document.save().is_ok(){
            self.status = Status::from("File saved successfully.".to_string());
        }else{
            self.status = Status::from("Error saving file.".to_string());
        }
    }

    fn cut(&mut self, p: &Position){
        if self.document.cell_exist(p){
            self.clipboard = Some(self.document.rows[p.y].cells[p.x].clone());
            self.document.rows[p.y].cells[p.x].reset();

            self.status = Status::from("Cell Cut".to_string());

        }
    }

    fn copy(&mut self, p: &Position){
        if self.document.cell_exist(p){
            let new_cell = self.document.rows[p.y].cells[p.x].clone();
            self.clipboard = Some(new_cell);

            self.status = Status::from("Cell Copied".to_string());
        }
    }

    fn paste(&mut self, p: &Position){
        if self.clipboard.is_some(){
            self.document.insert_cell(p, self.clipboard.as_ref().unwrap());

            self.status = Status::from("Cell Pasted".to_string());

        }
    }

    fn save_as(&mut self){
        let new_name = self.prompt("Save as: ", |_,_,_|{}).unwrap_or(None);
        if new_name.is_none(){
            self.status = Status::from("Canceled.".to_string());
            return;
        }

        self.document.file_name = new_name;

        if self.document.save().is_ok(){
            self.status = Status::from("File saved successfully.".to_string());
        }else{
            self.status = Status::from("Error saving file.".to_string());
        }
    }

    fn update(&mut self) -> Result<(), std::io::Error>{
        Terminal::hide_cursor();
        Terminal::goto(&Position::default());
        self.draw_status_message()?;
        
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
        let size = self .terminal.size();
        match key{
            KeyCode::Left => {
                self.cell_position.x = self.cell_position.x.saturating_sub(1);
            },
            KeyCode::Right => {
                self.cell_position.x = self.cell_position.x.saturating_add(1);
            },
            KeyCode::Up => {
                if self.cell_position.y.saturating_sub(self.offset.y) == 0 && self.offset.y > 0{
                    self.offset.y = self.offset.y.saturating_sub(1);
                }
                self.cell_position.y = self.cell_position.y.saturating_sub(1);
            },
            KeyCode::Down => {
                if self.cell_position.y.saturating_sub(self.offset.y) == size.height.saturating_sub(1){
                    self.offset.y = self.offset.y.saturating_add(1);
                }
                self.cell_position.y = self.cell_position.y.saturating_add(1);
                
            },
            _ => ()
        }
    }

    fn teleport(&mut self, to: &Position){
        self.cell_position = *to;
        let mut new_offset: Position = Position::default();
        new_offset.y = to.y.saturating_sub(self.terminal.size().height);
        self.offset = new_offset;
    }

    fn search(&mut self){
        let old_position = self.cell_position.clone(); 
        let mut direction = SearchDirection::Forward;
        let query = self.prompt(
            "Search (ESC to cancel, Arrows to navigate): ", 
            |editor, key, query|{
                let mut moved = false;
                match key{
                    KeyCode::Right | KeyCode::Down => {
                        direction = SearchDirection::Forward;
                        editor.move_cursor(KeyCode::Right);
                        moved = true;
                    },
                    KeyCode::Left | KeyCode::Up => direction = SearchDirection::Backward,
                    _ => direction = SearchDirection::Forward,
                }
                if let Some(position) = editor.document.find(&query, &editor.cell_position, direction){
                    editor.cell_position = position;
                    editor.scroll();
                }else if moved{
                    editor.move_cursor(KeyCode::Left);
                }
            }).unwrap_or(None);

        if query.is_none(){
            self.cursor_position = old_position;
            self.scroll();
        }

        self.status = Status::default();
    }

    fn scroll(&mut self){
        let Position {x,y} = self.cell_position;
        let size = self.terminal.size();
        let mut offset = &mut self.offset;
        if y < offset.y{
            offset.y = y;
        }else if y >= offset.y.saturating_add(size.height){
            offset.y = y.saturating_sub(size.height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(size.width) {
            offset.x = x.saturating_sub(size.width).saturating_add(1);
        }
    }

    fn draw_edit_line(&mut self) -> Result<(), std::io::Error>{
        let curr_pos: &Position = &self.cell_position;
        if self.document.cell_exist(curr_pos){
            write!(stdout(), "{}", self.document.get_cell(curr_pos).unwrap().val)
        }else{
            write!(stdout(), "")
        }
    }

    fn draw_cell(&mut self, p: &Position) -> Result<(), std::io::Error>{
        if self.cell_position.x == p.x && self.cell_position.y == p.y{
            Terminal::set_bg_color(Color::White);
            Terminal::set_fg_color(Color::Black);
        }

        if self.document.cell_exist(p){
            let content = &self.document.rows[p.y].cells[p.x].render(9);
            let len = content.len();
            let margin_right: usize = 9_usize.saturating_sub(len);
            write!(stdout(), "{}{}", &content, &" ".repeat(margin_right))?;
        }else{
            write!(stdout(), "{}", &" ".repeat(9))?;
        }

        Terminal::reset_colors();
        Ok(())
    }

    fn draw_row(&mut self, y: usize) -> Result<(), std::io::Error>{
        let num_col = self.terminal.size.width.saturating_sub(5) / 9;

        if self.cell_position.y != y {
            Terminal::set_bg_color(Color::White);
            Terminal::set_fg_color(Color::Black);
        }
        write!(stdout(), "      ")?;
        Terminal::reset_colors();

        for x in 0..num_col{
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

        self.draw_header()?;
        write!(stdout(), "\r\n")?;

        Terminal::reset_colors();

        for i in self.offset.y..size.height.saturating_add(self.offset.y){
            Terminal::clear_line();
            self.draw_row(i)?;
            // write!(stdout(), "\u{2502} {}\r\n", i/2)?;
        }

        Ok(())
    }

    fn draw_status_message(&mut self) -> Result<(), std::io::Error>{
        Terminal::goto(&Position{x:0,y:0});
        Terminal::clear_line();
        let size = self.terminal.size();

        let helper_message = "CTRL + Q: Quit | CTRL + S: Save | CTRL + ALT + S: Save as".to_string();
        let helper_message_len = helper_message.len();

        if self.status.message.is_empty(){
            write!(stdout(), "Editing: {}", self.document.file_name.as_ref().unwrap_or(&"[No Name]".to_string()))?;
        }else{
            write!(stdout(), "{}", self.status.message)?;
        }

        if size.width > 2*helper_message_len{
            Terminal::goto(&Position{x: size.width.saturating_sub(helper_message_len), y:0});
            Terminal::set_bg_color(Color::White);
            Terminal::set_fg_color(Color::Black);
            write!(stdout(), "{}", helper_message)?;
            Terminal::reset_colors();
        }

        Ok(())
    }

    fn draw_header(&mut self) -> Result<(), std::io::Error>{
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let cols: usize = max(self.document.col_count(), self.cell_position.x.saturating_add(1));

        let headers: Vec<String>= if self.header {
            let col_titles = self.document.rows[0].cells.iter().map(|c| c.render(9)).collect::<Vec<String>>();
            let lens = col_titles.iter().map(std::string::String::len).collect::<Vec<usize>>();
            let margin = lens.iter().map(|l| 9_usize.saturating_sub(*l)/2).collect::<Vec<usize>>();
            let headers_str = col_titles.iter().enumerate().map(|i| format!("{}{}{}", &" ".repeat(margin[i.0]),i.1, &" ".repeat(9_usize.saturating_sub(lens[i.0].saturating_add(margin[i.0]))))).collect::<Vec<String>>();
            headers_str
        }else{
            let mut headers_str: Vec<String> = Vec::new();
            let alphabet_chars = alphabet.chars().collect::<Vec<char>>();
            for i in alphabet_chars.iter().take(cols){
                let h_fmt = format!("    {}    ", i);
                headers_str.push(h_fmt);
            }
            headers_str
        };
        //Columns Index Margin
        Terminal::clear_line();
        write!(stdout(), "      ")?;

        for i in headers.iter().enumerate(){
            if i.0 == self.cell_position.x{
                Terminal::set_bg_color(Color::Black);
                Terminal::set_fg_color(Color::White);
            }else{
                Terminal::set_bg_color(Color::White);
                Terminal::set_fg_color(Color::Black);
            }

            write!(stdout(), "{}", i.1)?;
        }

        Terminal::reset_colors();
        Ok(())
    }

    pub fn prompt<C>(&mut self, message: &str, mut callback: C) -> Result<Option<String>, std::io::Error> where C: FnMut(&mut Self, KeyCode, &String){
        let mut result = String::new();
        loop {
            self.status = Status{message:format!("{}{}", &message,&result)};
            self.update()?;
            let event = Terminal::read_event()?;
            if let Event::Key(KeyEvent { code: key, modifiers: _ }) = event{
                match key{
                    KeyCode::Char(c) => result.push(c),
                    KeyCode::Backspace => {result.pop();},
                    KeyCode::Enter => break,
                    KeyCode::Esc => {
                        result.truncate(0);
                        break;
                    },
                    _ => ()
                };
                callback(self,key, &result);
            }
        }

        if result.is_empty(){
            return Ok(None);
        }

        Ok(Some(result))
    }
}

fn die(e: &std::io::Error){
    Terminal::clear();
    panic!("{}", e);
}