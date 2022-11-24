#[derive(Debug)]
pub struct Matrix<T: Clone> {
    // Fields you need
    rows: usize,
    cols: usize,
    data: Vec<Vec<T>>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: &[T]) -> Matrix<T> {
        let arrays = data.clone();
        let row_num = rows.clone();

        let mut new_matrix: Vec<Vec<T>> = vec![];

        for n in 0..row_num {
            let col_num = cols.clone();
            let arr = arrays.clone();
            let new_col = &arr[n..n+col_num];
            
            new_matrix.push(new_col.to_vec());
        }

        return Matrix {
            rows: rows,
            cols: cols,
            data: new_matrix,
        };
    }

    pub fn by_row<'a>(&self) -> RowIter<'a, T> {
        let data = &self.clone();
        let rows = data.rows.clone();
        let cols = data.cols.clone();
        let matrix = data.data.clone();

        return RowIter::new(
            rows, cols, &matrix
        )
    }

    pub fn by_col<'a>(&self) -> ColIter<'a, T> {
        let data = &self.clone();
        let rows = data.rows.clone();
        let cols = data.cols.clone();
        let matrix = data.data.clone();

        return ColIter::new(
            rows, cols, &matrix
        )
    }
}

pub struct RowIter<'a, T> {
    index: usize,
    values: &'a Vec<T>,
}

pub struct ColIter<'a, T> {
    index: usize,
    values: &'a Vec<T>,
}

impl<'a, T:Clone> RowIter<'a, T> {
    pub fn new(rows: usize, cols: usize, data: &Vec<Vec<T>>) -> Self {
        let rows = rows.clone();
        let cols = cols.clone();
        let matrix = &data.clone();

        let mut result: Vec<T> = vec![];
        
        for i in 0..rows.clone() {
            for j in 0..cols.clone() {
                let mat = matrix.clone();
                let val = mat[i][j].clone();
                result.push(val);
            }
        }

        return RowIter {
            index: rows * cols,
            values: result
        };
    }
}

impl<'a, T:Clone> ColIter<'a, T> {
    pub fn new(rows: usize, cols: usize, data: &Vec<Vec<T>>) -> Self {
        let rows = rows.clone();
        let cols = cols.clone();
        let matrix = &data.clone();

        let mut result: Vec<T> = vec![];
        
        for i in 0..cols.clone() {
            for j in 0..rows.clone() {
                let mat = matrix.clone();
                let val = mat[j][i].clone();
                result.push(val);
            }
        }

        return ColIter {
            index: rows * cols,
            values: result
        };
    }
}

impl <'a, T: Clone>Iterator for RowIter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.values.len() {
            let output = &self.values[self.index].clone();
            self.index += 1;
            Some(output)
        } else {
            return None
        }
    }
}

impl <'a, T: Clone>Iterator for ColIter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.values.len() {
            let output = &self.values[self.index].clone();
            self.index += 1;
            Some(output)
        } else {
            return None
        }
    }
}

