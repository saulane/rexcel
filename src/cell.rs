use std::fmt::Display;
use crossterm::style::Color;

use crate::Position;

#[derive(PartialEq,Clone, Debug)]
pub enum DataType{
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Empty
}

impl Default for DataType{
    fn default() -> Self {
        DataType::Empty
    }
}

impl Display for DataType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(),std::fmt::Error> {
        match self{
            DataType::String(s) => write!(f, "{}", s),
            DataType::Int(i) =>  write!(f, "{}", i),
            DataType::Float(fl) =>  write!(f, "{}", fl),
            DataType::Bool(b) => write!(f, "{}", b),
            DataType::Empty => write!(f, ""),
        }
    }
}

impl DataType{
    pub fn len(&self) -> usize{
        match &self{
            DataType::Int(e) => e.to_string().len(),
            DataType::Float(e) => e.to_string().len(),
            DataType::String(e) => e.len(),
            DataType::Bool(e) => {
                if *e{
                    4
                }else{
                    5
                }
            },
            DataType::Empty => 0,
        }
    }

    pub fn insert(&mut self, c:char){
        match self {
            DataType::String(x) => {
                x.push(c);
                if x.starts_with('='){
                    if let Ok(x) = x.parse::<f64>(){
                        self.switch_type(&DataType::Float(x));
                    };
                }
            },
            DataType::Empty => {
                self.switch_type(&DataType::String(String::default()));
                self.insert(c);
            },
            _ => ()
        }
    }

    pub fn delete(&mut self, _at:usize){
        match self{
            DataType::String(x) => {x.pop();},
            _ => ()
        }
    }

    fn switch_type(&mut self, nt: &DataType){
        *self = match nt{
            DataType::Bool(_) => DataType::Bool(bool::default()),
            DataType::Int(_) => DataType::Int(i64::default()),
            DataType::Float(f) => DataType::Float(*f),
            DataType::String(_) => DataType::String(String::default()),
            DataType::Empty => DataType::Empty,
        };
    }

    pub fn is_empy(&self) -> bool{
        *self == DataType::Empty
    }

    pub fn is_string(&self) -> bool{
        matches!(*self, DataType::String(_))
    }
    pub fn is_int(&self) -> bool{
        matches!(*self, DataType::Int(_))
    }
    pub fn is_float(&self) -> bool{
        matches!(*self, DataType::Float(_))
    }

    pub fn is_bool(&self) -> bool{
        matches!(*self, DataType::Bool(_))
    }
}

#[derive(Clone)]
pub struct Cell{
    pub val: DataType,
    pub pos: Position,
}

impl Default for Cell{
    fn default() -> Self {
        Self{
            val: DataType::Empty,
            pos: Position{x:0,y:0}
        }
    }
}

impl From<String> for Cell{
    fn from(val: String) -> Self {
        Self{
            val: DataType::String(val),
            pos: Position{x:0,y:0}
        }
    }
}

impl Cell{
    pub fn insert(&mut self, c: char){
        match &self.val{
            DataType::String(_) => self.val.insert(c),
            DataType::Empty => {
                self.val = DataType::String(String::from(c));
            }
            _ => (),
        }
    }

    pub fn delete(&mut self, at:usize){
        match &self.val{
            DataType::String(_) => self.val.delete(at),
            _ => (),
        }
    }

    pub fn reset(&mut self){
        self.val = DataType::Empty
    }

    pub fn render(&self, max_len: usize) -> String{
        let mut val = match &self.val{
            DataType::Int(s) => s.to_string(),
            DataType::Float(s) => s.to_string(),
            DataType::String(s) => s.to_string(),
            DataType::Bool(s) => s.to_string(),
            DataType::Empty => String::default(),
        };

        if max_len == 0{
            return val
        }else if val.len() > max_len{
            val.truncate(max_len.saturating_sub(2));
            val.push_str("..");
        }
        // format!("{}{}", crossterm::style::SetForegroundColor(Color::Red), val)
        val
    }
}