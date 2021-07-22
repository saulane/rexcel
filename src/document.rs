use crate::Row;
use crate::Cell;
use crate::Position;

use std::fs::{self, File};
use std::io::Write;

#[derive(Default)]
pub struct Document{
    pub rows: Vec<Row>,
    pub file_name: Option<String>,
    pub len: usize,
}

impl Document{
    pub fn open(filename: &str) -> Result<Self, std::io::Error>{
        let content = fs::read_to_string(filename)?;
        let mut rows: Vec<Row> = Vec::new();
        let mut len: usize = 0;
        for row in content.lines(){
            let cells: Vec<Cell> = row.split(";").map(|s| Cell::from(s.to_string())).collect();
            let row_len = cells.len();
            rows.push(Row{cells, len: row_len});
            len+=1;
        }

        Ok(Self{
            rows,
            file_name: Some(filename.to_string()),
            len,
        })
    }

    pub fn save(&mut self) -> Result<(), std::io::Error>{
        if let Some(filename) = &self.file_name{
            let mut file = File::create(filename)?;
            for row in &self.rows{
                file.write_all(row.stringify(";").as_bytes())?;
                file.write_all(b"\n")?;
            }
        }

        Ok(())
    }

    pub fn insert(&mut self, at: &Position, c: char){
        if self.rows.len() <= at.y{
            self.fill(at.y.saturating_sub(self.len).saturating_add(1))
        }

        self.rows[at.y].insert(c, at.x)
    }

    pub fn insert_cell(&mut self, at: &Position, cell: &Cell){
        if self.rows.len() <= at.y{
            self.fill(at.y.saturating_sub(self.len).saturating_add(1));
        }
        self.rows[at.y].insert_cell(at.x, cell);
    }

    pub fn delete(&mut self, at:&Position){
        if self.rows.len() <= at.y{
            return;
        }

        self.rows[at.y].delete(at.x)
    }

    pub fn fill(&mut self, n: usize){
        for _ in 0..n{
            self.add_row();
        }
        self.update_len()
    }

    fn update_len(&mut self){
        self.len = self.rows.len()
    }

    fn max_row_len(&self) -> usize{
        let max = self.rows.iter().map(|r| r.cells.len()).max().unwrap_or(0);
        max
    }

    pub fn add_column(&mut self){
        let max_len = self.max_row_len();
        for i in 0..self.len{
            let row_len = self.rows[i].len;
            if row_len <= max_len{
                self.rows[i].fill(max_len.saturating_sub(row_len));
            }

            self.rows[i].cells.push(Cell::default());
        }
    }

    pub fn add_row(&mut self){
        self.rows.push(Row::default());
    }

    pub fn cell_exist(&self, p: &Position) -> bool{
        if p.y < self.len && p.x < self.rows[p.y].len {
            true
        }else{
            false
        }
    }

    pub fn get_cell(&self, p: &Position) -> Option<&Cell>{
        match self.cell_exist(p){
            true => Some(&self.rows[p.y].cells[p.x]),
            false => None,
        }
    }
}