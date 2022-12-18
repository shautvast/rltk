pub trait Mat<T> {
    fn get(&self, row: usize, column: usize) -> T;
    fn set(&mut self, row: usize, column: usize, value: T);
    fn rows() -> Vec<Vec<T>>;
}

pub trait Csr<T>: Mat<T> {

}

pub struct Csr_f64 {
    index_pointers: Vec<usize>,
    indices: Vec<usize>,
    data: Vec<f64>,
}

impl Csr_f64 {
    pub fn new() -> Self {
        Self {
            index_pointers: Vec::new(),
            indices: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl Csr<f64> for Csr_f64 {}

impl Mat<f64> for Csr_f64 {
    fn get(&self, row: usize, column: usize) -> f64 {
        if row + 2 > self.index_pointers.len() {
            0.0
        }
        let start_index = self.index_pointers[row];
        let end_index = self.index_pointers[row + 1];

        if start_index == end_index {
            0.0
        } else {
            let mut index = start_index;
            while index < end_index && column != self.indices[index] {
                index += 1;
            }
            if index == end_index {
                0.0
            } else {
                self.data[index]
            }
        }
    }

    fn set(&mut self, _row: usize, _column: usize, _value: T) {
        panic!("Csr is immutable")
    }

    fn rows() -> Vec<Vec<f64>> {
        todo!()
        // public double[][] getRows() {
        //     return toDense().getRows();
        // }
    }


}

impl Into<dyn Mat<T>> for Csr_f64{
    fn into(self) -> Box<dyn Mat<T>> {
        todo!()
    }
}