
pub type Matrix<T> = Vec<Vec<T>>;

#[derive(Debug)]
pub struct MatrixWrapper<T>(Matrix<T>);

impl<T> MatrixWrapper<T> {
    pub fn rows_len(&self) -> usize {
        self.0.len()
    }
    pub fn cols_len(&self) -> Option<usize> {
        if self.0.len() > 0 {
            Some(self.0[0].len())
        } else {
            None
        }
    }
    pub fn enumerate(&self) -> impl Iterator<Item = (i32, i32, &T)> {
       self
            .0
            .iter()
            .enumerate()
            .flat_map(move|(i, row)| row.iter().enumerate().map(move |(j, col)| (i as i32, j as i32, col)))
    }
    pub fn at(&self, row: i32, col: i32) -> Option<&T> {
        if row < 0 || col < 0 {
            return None;
        }
        let (row, col) = (row as usize, col as usize);
        if row < self.rows_len() && self.cols_len().is_some_and(|l| col < l) {
            Some(&self.0[row][col])
        } else {
            None
        }
    }
}

impl<U> FromIterator<Vec<U>> for MatrixWrapper<U> {
    fn from_iter<T: IntoIterator<Item = Vec<U>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
