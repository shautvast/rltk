use crate::mat::{Mat, Numeric, Shape};

pub struct Csr<T> where T: Numeric{
    index_pointers: Vec<usize>,
    indices: Vec<usize>,
    data: Vec<T>,
}

impl <T> Csr<T> where T:Numeric{
    pub fn new() -> Self {
        Self {
            index_pointers: Vec::new(),
            indices: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl <T> Mat<T> for Csr<T> where T:Numeric {
    fn get(&self, row: usize, column: usize) -> T {
        if row + 2 > self.index_pointers.len() {
            return Numeric::default_value();
        }
        let start_index = self.index_pointers[row];
        let end_index = self.index_pointers[row + 1];

        if start_index == end_index {
            return Numeric::default_value();
        } else {
            let mut index = start_index;
            while index < end_index && column != self.indices[index] {
                index += 1;
            }
            if index == end_index {
                return Numeric::default_value();
            } else {
                self.data[index]
            }
        }
    }

    fn set(&mut self, _row: usize, _column: usize, _value: T) {
        panic!("Csr is immutable")
    }

    fn shape(&self) -> Shape {
        Shape::new(self.index_pointers.len() - 1,
                   self.indices.iter().map(|p| p + 1).max().unwrap_or(0))
    }
}

impl <T> From<Vec<Vec<T>>> for Csr<T> where T:Numeric + PartialEq{
    fn from(rows: Vec<Vec<T>>) -> Self {
        let mut this = Self::new();

        this.index_pointers.push(0);

        for row in rows {
            for (index,value) in row.into_iter().enumerate(){
                if value != value.default() {
                    this.data.push(value);
                    this.indices.push(index);
                }
            }
            this.index_pointers.push(this.indices.len());
        }
        this
    }
}