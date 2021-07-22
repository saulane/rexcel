use crate::Cell;

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
        self.cells.iter().map(|cell| cell.val.to_string()).collect::<Vec<String>>().join(sep)
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