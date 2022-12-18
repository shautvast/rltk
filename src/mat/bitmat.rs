use std::collections::BTreeMap;

use crate::mat::{Mat, Numeric, Shape};

// Mutable sparse matrix for boolean values
// stored in u128
struct SparseBitMat {
    data: BTreeMap<usize, BTreeMap<usize, u128>>,
}

impl SparseBitMat {
    fn new() -> Self {
        Self {
            data: BTreeMap::new()
        }
    }

    pub fn set_true(&mut self, row_index: usize, col_index: usize) {
        let d = self.get_byte(row_index, col_index);
        *d |= 1 << (col_index % 128);
    }

    pub fn set_false(&mut self, row_index: usize, col_index: usize) {
        let d = self.get_byte(row_index, col_index);
        *d &= !(1 << (col_index % 128));
    }

    fn get_byte(&mut self, row_index: usize, col_index: usize) -> &mut u128 {
        let row_index_b = row_index >> 7;
        let col_index_b = col_index >> 7;
        let mut row = self.data.entry(row_index_b).or_insert_with(BTreeMap::new);
        let d = row.entry(col_index_b).or_insert(0_u128);
        d
    }


}

impl Mat<bool> for SparseBitMat {
    fn get(&self, row_index: usize, col_index: usize) -> bool {
        let row_index_b = row_index >> 7;
        let col_index_b = col_index >> 7;
        let row = self.data.get(&row_index_b);
        if let Some(row) = row {
            let d = row.get(&col_index_b);
            if let Some(d) = d {
                let bit = 1 << (col_index % 128);
                return (*d & bit) != 0;
            }
        }
        false
    }

    fn set(&mut self, row_index: usize, col_index: usize, value: bool) {
        if value {
            self.set_true(row_index, col_index);
        } else {
            self.set_false(row_index, col_index);
        }
    }


    fn shape(&self) -> Shape {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::mat::Mat;

    #[test]
    fn test_get_and_set() {
        let mut mat = super::SparseBitMat::new();
        mat.set(15, 15, true);
        assert_eq!(mat.get(0, 0), false); //untouched
        assert_eq!(mat.get(15, 15), true); //touched

        mat.set(15, 15, false);
        assert_eq!(mat.get(15, 15), false); //touched, set to false
        mat.set(1001, 1001, false);
        assert_eq!(mat.get(1001, 1001), false); //untouched, set to false
    }
}