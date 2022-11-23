use std::ops;

#[derive(Debug)]
pub struct Matrix<T: Clone> {
    // Fields you need
    data: [[Cell<T>; 2]; 2],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell<T>(pub T);

impl ops::Add<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn add(self, _rhs: Cell<String>) -> Cell<String> {
        let num = self.0;
        let original = _rhs.0;
        let mut new_string = String::from("");

        if num >= 0 {
           let num_to_char = num.to_string();
           new_string = num_to_char + &String::from(" ") + &original;
        }
        if num < 0 {
            let reverse = original.chars().rev().collect::<String>();
            let num_to_char = (num * -1).to_string();
            new_string = reverse + &String::from(" ") + &num_to_char;
        }

        return Cell(new_string);
    }
}

impl ops::Mul<Cell<String>> for Cell<i32> {
    type Output = Cell<String>;

    fn mul(self, _rhs: Cell<String>) -> Cell<String> {
        let num = self.0;
        let original = _rhs.0;
        let mut new_string = String::from("");

        if num > 0 {
            new_string = original.repeat(num.try_into().unwrap());
        }
        if num < 0 {
            let reverse = original.chars().rev().collect::<String>();
            let pos_num = num * -1;
            new_string = reverse.repeat(pos_num.try_into().unwrap());
        }
        return Cell(new_string);
    }
}

impl ops::Add<Matrix<String>> for Matrix<i32> {
    type Output = Matrix<String>;

    fn add(self, _rhs: Matrix<String>) -> Matrix<String> {
        let [[a0, a1], [a2, a3]] = self.data;
        let [[b0, b1], [b2, b3]] = _rhs.data;

        return Matrix {
            data: [
                [a0 + b0, a1 + b1], 
                [a2 + b2, a3 + b3]
            ]
        };
    }
}

impl ops::Mul<Matrix<String>> for Matrix<i32> {
    type Output = String;

    fn mul(self, _rhs: Matrix<String>) -> String {
        let [[a0, a1], [a2, a3]] = self.data;
        let [[b0, b1], [b2, b3]] = _rhs.data;

        return (a0 * b0).0 + &String::from(" ") + &(a1 * b2).0 + &String::from(" ") + &(a2 * b1).0 + &String::from(" ") + &(a3 * b3).0;
    }
}

impl<T: Clone> Matrix<T> {
    pub fn new(data: &[T; 4]) -> Matrix<T> {
        let [a,b,c,d] = data.clone();
        let new_matrix = Matrix {
            data: [
                [Cell(a), Cell(b)], 
                [Cell(c), Cell(d)]
            ]
        };
        return new_matrix;
    }

    pub fn by_row(&self) -> Vec<Cell<T>> {
        let data = &self.data.clone();
        let [[a,b], [c,d]] = data.clone();
        return vec![a,b,c,d];
    }
    
    pub fn by_col(&self) -> Vec<Cell<T>> {
        let data = &self.data.clone();
        let [[a,b], [c,d]] = data.clone();
        return vec![a,c,b,d];
    }
}