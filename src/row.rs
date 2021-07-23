use crate::{Cell, SearchDirection};

pub struct Row{
    pub cells: Vec<Cell>,
    pub len: usize,
}

impl Default for Row{
    fn default() -> Self {
        Self{
            cells: Vec::new(),
            len: 0
        }
    }
}

impl Row{
    pub fn render(&self) -> String{
        let mut result = String::new();
        for i in &self.cells{
            result.push_str(&format!("{} | ", i.val));
        }

        result
    }

    pub fn stringify(&self, sep: &str) -> String{
        self.cells.iter().map(|cell| cell.render(0)).collect::<Vec<String>>().join(sep)
    }

    pub fn insert(&mut self, c: char, at: usize){
        if self.len <= at{
            self.fill(at.saturating_sub(self.len).saturating_add(1))
        }

        self.cells[at].insert(c)
    }

    pub fn insert_cell(&mut self, at: usize, cell: &Cell){
        if self.len <= at{
            self.fill(at.saturating_sub(self.len).saturating_add(1));
        }
        self.cells[at] = cell.clone();
    }

    pub fn delete(&mut self, at: usize){
        if self.cells.len() <= at{
            return;
        }

        let len = self.cells[at].val.len();

        self.cells[at].delete(len)
    }

    pub fn find(&self, query: &str, at:usize, direction: SearchDirection) -> Option<usize>{
        if at > self.len || query.is_empty(){
            return None;
        }
        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.len
        } else {
            at
        };

        if direction == SearchDirection::Forward{
            for i in start..end{
                if self.cells[i].val.to_string().contains(query){
                    return Some(i)
                }
            }
        }else{
            for i in (start..end).rev(){
                if self.cells[i].val.to_string().contains(query){
                    return Some(i)
                }
            }
        }

        None
    }

    pub fn fill(&mut self, n: usize){
        for _ in 0..n{
            self.cells.push(Cell::default())
        }

        self.update_len()
    }

    fn update_len(&mut self){
        self.len = self.cells.len()
    }
}