use std::fmt::Display;
use crate::Position;
// use std::io::{self, Write};

// use std::fmt::{Display, write};

// pub enum CellType {
//     Number(f64),
//     String(String),
// }

// impl Display for CellType{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self{
//             CellType::Number(x) => write!(f, "{}", x),
//             CellType::String(s) => write!(f, "{}", s),
//         }
//     }
// }

// pub enum CellContent {
//     String(String),
//     Number(f64)
// }

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

    pub fn reset(&mut self){
        self.switch_type(DataType::Empty)
    }

    pub fn insert(&mut self, c:char){
        match self {
            DataType::String(x) => x.push(c),
            DataType::Empty => {
                self.switch_type(DataType::String(String::default()));
                self.insert(c);
            },
            _ => ()
        }
    }

    pub fn delete(&mut self, at:usize){
        match self{
            DataType::String(x) => {x.pop();},
            _ => ()
        }
    }

    fn switch_type(&mut self, nt: DataType){
        *self = match nt{
            DataType::Bool(_) => DataType::Bool(bool::default()),
            DataType::Int(_) => DataType::Int(i64::default()),
            DataType::Float(_) => DataType::Float(f64::default()),
            DataType::String(_) => DataType::String(String::default()),
            DataType::Empty => DataType::Empty,
            
        };
    }

    pub fn is_empy(&self) -> bool{
        *self == DataType::Empty
    }

    pub fn is_string(&self) -> bool{
        if let DataType::String(_) = *self{
            true
        }else{
            false
        }
    }
    pub fn is_int(&self) -> bool{
        if let DataType::Int(_) = *self{
            true
        }else{
            false
        }
    }
    pub fn is_float(&self) -> bool{
        if let DataType::Float(_) = *self{
            true
        }else{
            false
        }
    }

    pub fn is_bool(&self) -> bool{
        if let DataType::Bool(_) = *self{
            true
        }else{
            false
        }
    }
}

pub struct Cell{
    pub val: DataType,
    pub pos: Position,
}

impl Default for Cell{
    fn default() -> Self {
        Self{
            val: DataType::String(String::from("")),
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
            DataType::String(s) => self.val.insert(c),
            _ => (),
        }
    }

    pub fn delete(&mut self, at:usize){
        match &self.val{
            DataType::String(s) => self.val.delete(at),
            _ => (),
        }
    }

    pub fn reset(&mut self){
        self.val.reset()
    }

    fn update_type(&mut self){
        if let DataType::String(s) = &self.val{
            match s.as_str().parse::<f64>(){
                Ok(x) => self.val = DataType::Float(x),
                Err(_) => (),
            }
        }
    }
}