mod csr;
mod sparse;
mod bitmat;

pub trait Mat<T: Numeric> {
    fn get(&self, row_index: usize, col_index: usize) -> T;
    fn set(&mut self, row_index: usize, col_index: usize, value: T);
    fn shape(&self) -> Shape;
}

#[derive(PartialEq, Eq, Debug)]
pub struct Shape {
    rows: usize,
    cols: usize,
}

impl Shape {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
        }
    }
}

impl From<Shape> for (usize, usize){
    fn from(this: Shape) -> Self {
        (this.rows, this.cols)
    }
}

pub trait Numeric: Copy + Default {
    fn default<T>() -> Self;
}

impl Numeric for f64 {
    fn default<T>() -> f64 { 0.0 }
}

impl Numeric for f32 {
    fn default<T>() -> f32 { 0.0 }
}

impl Numeric for usize {
    fn default<T>() -> usize { 0 }
}

impl Numeric for isize {
    fn default<T>() -> isize { 0 }
}

impl Numeric for i8 {
    fn default<T>() -> i8 { 0 }
}

impl Numeric for u8 {
    fn default<T>() -> u8 { 0 }
}

impl Numeric for i16 {
    fn default<T>() -> i16 { 0 }
}

impl Numeric for u16 {
    fn default<T>() -> u16 { 0 }
}

impl Numeric for i32 {
    fn default<T>() -> i32 { 0 }
}

impl Numeric for u32 {
    fn default<T>() -> u32 { 0 }
}

impl Numeric for i64 {
    fn default<T>() -> i64 { 0 }
}

impl Numeric for u64 {
    fn default<T>() -> u64 { 0 }
}

impl Numeric for i128 {
    fn default<T>() -> i128 { 0 }
}

impl Numeric for bool {
    fn default<T>() -> Self {
        false
    }
}

impl Numeric for u128 {
    fn default<T>() -> u128 { 0 }
}

#[cfg(test)]
mod test {
    use mat::csr::CsrMat;
    use mat::Mat;

    use crate::mat;

    #[test]
    fn test_i32() {
        let rows = vec![vec![1, 0, 0, 0], vec![2]];
        let new_mat = CsrMat::from(rows);
        assert_eq!(2, new_mat.get(1, 0));
        assert_eq!(0, new_mat.get(10, 0));
    }

    #[test]
    fn test_f64() {
        let rows = vec![vec![1.0, 0.0, 0.0, 0.0], vec![2.0]];
        let new_mat = CsrMat::from(rows);
        assert_eq!(2.0, new_mat.get(1, 0));
        assert_eq!(0.0, new_mat.get(10, 0));
    }
}