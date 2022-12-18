use crate::mat::{Mat, Numeric, Shape};
use crate::mat::sparse::SparseMat;

/// Compressed Sparse Row matrix
/// Immutable, can be constructed from Vec<Vec<T>>, or SparseMat<T>
/// Better performance when iterating (i think), less memory
pub struct CsrMat<T> where T: Numeric {
    index_pointers: Vec<usize>,
    indices: Vec<usize>,
    data: Vec<T>,
}

impl<T> CsrMat<T> where T: Numeric {
    pub fn new() -> Self {
        Self {
            index_pointers: Vec::new(),
            indices: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl<T> Mat<T> for CsrMat<T> where T: Numeric {
    fn get(&self, row: usize, column: usize) -> T {
        if row + 2 > self.index_pointers.len() {
            return Numeric::default::<T>();
        }
        let start_index = self.index_pointers[row];
        let end_index = self.index_pointers[row + 1];

        if start_index == end_index {
            return Numeric::default::<T>();
        } else {
            let mut index = start_index;
            while index < end_index && column != self.indices[index] {
                index += 1;
            }
            if index == end_index {
                return Numeric::default::<T>();
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

impl<T> From<Vec<Vec<T>>> for CsrMat<T> where T: Numeric + PartialEq {
    fn from(rows: Vec<Vec<T>>) -> Self {
        let mut this = Self::new();

        this.index_pointers.push(0);

        for row in rows {
            for (index, value) in row.into_iter().enumerate() {
                if value != Numeric::default::<T>() {
                    this.data.push(value);
                    this.indices.push(index);
                }
            }
            this.index_pointers.push(this.indices.len());
        }
        this
    }
}

impl<T> From<Box<dyn Mat<T>>> for CsrMat<T> where T: Numeric + PartialEq {
    fn from(this: Box<dyn Mat<T>>) -> Self {
        let mut csr = Self::new();

        csr.index_pointers.push(0);
        let (rows, cols) = Shape::into(this.shape());
        for row in 0..rows {
            for col in 0..cols {
                let value = this.get(row, col);
                if value != Numeric::default::<T>() {
                    csr.data.push(value);
                    csr.indices.push(col);
                }
            }
            csr.index_pointers.push(csr.indices.len());
        }
        csr
    }
}


#[cfg(test)]
mod test {
    use crate::mat::{Mat, Shape};
    use crate::mat::csr::CsrMat;
    use crate::mat::sparse::SparseMat;

    #[test]
    fn test_from_mat() {
        let mut mat: Box<dyn Mat<u32>> = Box::new(SparseMat::new());
        mat.set(1, 1, 1_u32);
        mat.set(2, 2, 2_u32);

        let csr: CsrMat<u32> = mat.into();
        assert_eq!(csr.get(1, 1), 1);
        assert_eq!(csr.get(2, 2), 2);
        assert_eq!(csr.shape(), Shape::new(3, 3));
    }
}