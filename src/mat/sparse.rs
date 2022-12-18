use std::collections::BTreeMap;

use crate::mat::{Mat, Numeric, Shape};

/// BTreeMap based implementation, useful for mutating
/// every row is a map<index, value> and the matrix is a map<index, map>
/// resulting in a map<row_index<map<col_index,value>>
/// uses a BTreeMap to keep the keys (indexes) ordered.
pub struct SparseMat<T: Numeric> {
    data: BTreeMap<usize, BTreeMap<usize, T>>,
}

impl<T: Numeric> SparseMat<T> {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new()
        }
    }
}

impl<T: Numeric> Mat<T> for SparseMat<T> {
    fn get(&self, row_index: usize, col_index: usize) -> T {
        self.data.get(&row_index)
            .map(|row| row.get(&col_index)
                .map(|v|*v)
                .unwrap_or(Numeric::default::<T>()))
            .unwrap_or(Numeric::default::<T>())
    }

    fn set(&mut self, row_index: usize, col_index: usize, value: T) {
        let row = self.data.entry(row_index).or_insert_with(BTreeMap::new);
        row.insert(col_index, value);
    }

    fn shape(&self) -> Shape {
        let mut max_rows = 0;
        let mut max_cols = 0;
        for row_index in self.data.keys() {
            let row_index = *row_index;
            if row_index > max_rows {
                max_rows = row_index;
            }
            let row = self.data.get(&row_index).unwrap();
            let last_col = *row.keys().max().unwrap();
            if last_col > max_cols {
                max_cols = last_col;
            }
        }
        Shape::new(max_rows + 1, max_cols + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::mat::{Mat, Shape};
    use crate::mat::sparse::SparseMat;

    #[test]
    fn shape() {
        let mut mat = SparseMat::new();
        mat.set(10, 11, 1.5);
        assert_eq!(mat.shape(), Shape::new(11, 12));
    }
}
