# Iterating matrices

From the previous challenge task 2, instead of `.by_row()` and `.by_col()` returning vectors, it would be much more flexible if they were iterators -- you can collect them in a vector, you can iterate them by reference. 

So here it is!

Consider the 2D matrix below dimensional elements.

Given the inputs `rows` and `cols`, we start by constructing and iterating arrays (with `rows * cols` elements divided into `rows` number of size `cols` array) and cells of elements:

```rust
#[derive(Debug)]
pub struct Matrix<T> {
    // TODO
}

impl<T: Clone> Matrix<T> {
    /// Constructs a matrix with the number of rows `rows` and the number of columns `cols`. The data should be
    /// at least `rows * cols`, but we won't test with arrays that are too small or too large.
    /// If you want to panic on invalid input, your choice.
    ///
    /// The data will be arranged in a flat array by rows. You are expected to take ownership of them,
    /// so clone them and arrange them as you see fit.
    ///
    pub fn new(rows: usize, cols: usize, data: &[T]) -> Self {
        todo!()
    }

    pub fn by_row(&self) -> RowIter<T> {
        todo!()
    }

    pub fn by_col(&self) -> ColIter<T> {
        todo!()
    }
}

pub struct RowIter<T> {
    // TODO
}

pub struct ColIter<T> {
    // TODO
}
```

We expect you to implement the `Iterator` trait for both `RowIter` and `ColIter`, and you will need to iterate by reference. You'll need to change the types above a bit to put in the necessary lifetimes.

Here is an example test that should compile and run:

```rust
#[test]
fn test_iteration_0() {
    let data = [1, 2,
                3, 4];
    let matrix = Matrix::new(2, 2, &data);

    assert_eq!(matrix.by_row().collect::<Vec<_>>(), vec![&1, &2, &3, &4]);
    assert_eq!(matrix.by_col().collect::<Vec<_>>(), vec![&1, &3, &2, &4]);
}
```
