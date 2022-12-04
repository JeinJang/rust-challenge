#[derive(Debug)]
pub struct Matrix<T: Clone> {
    // Fields you need
    rows: usize,
    cols: usize,
    data: Vec<Vec<T>>,
}

pub struct RowIter<'a, T> {
    index: usize,
    values: Vec<&'a T>,
}

pub struct ColIter<'a, T> {
    index: usize,
    values: Vec<&'a T>,
}

impl<T: Clone + std::fmt::Display> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: &[T]) -> Matrix<T> {
        let row_num = rows.clone();

        let mut new_matrix: Vec<Vec<T>> = vec![];

        for n in 0..row_num {
            let col_num = cols.clone();
            let arr = data.clone();
            let new_col = &arr[(n * col_num)..((n + 1) * col_num)];
            
            new_matrix.push(new_col.to_vec());
        }

        return Matrix {
            rows: rows,
            cols: cols,
            data: new_matrix,
        };
    }

    pub fn by_row<'a>(&'a self) -> RowIter<'a, T> {
        let data = &self.clone();
        let rows = data.rows.clone();
        let cols = data.cols.clone();

        
        let mut result = vec![];
        
        for i in 0..rows.clone() {
            for j in 0..cols.clone() {
                let val = &data.data[i][j];
                result.push(val);
            }
        }

        return RowIter {
            index: 0,
            values: result
        };
    }

    pub fn by_col<'a>(&'a self) -> ColIter<'a, T> {
        let data = &self.clone();
        let rows = data.rows.clone();
        let cols = data.cols.clone();

        let mut result = vec![];
        
        for i in 0..cols.clone() {
            for j in 0..rows.clone() {
                let val = &data.data[j][i];
                result.push(val);
            }
        }

        return ColIter {
            index: 0,
            values: result
        };
    }
}

impl <'a, T: Clone>Iterator for RowIter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.values.len() {
            let output = &self.values[self.index];
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
            let output = &self.values[self.index];
            self.index += 1;
            Some(output)
        } else {
            return None
        }
    }
}

