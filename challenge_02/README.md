# 2x2 Matrix

Surprisingly, the cult film The Matrix is ​​getting a 4th version soon. The original is a cyberpunk classic, and in its honor we'll be playing around with matrices with four dimensional elements.

We start by constructing and iterating arrays (with four elements divided into two rows) and cells of elements:

```rust
#[derive(Debug)]
pub struct Matrix<T: Clone> {
    // Fields you need
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell<T>(pub T);

impl<T: Clone> Matrix<T> {
    /// Data is expected to be passed with a static array -- see below for examples of
    /// construct. What might the elements be? We will only test with two types: String and i32.
    ///
    /// Expected to be passed in rows, from left to right. That is, if we pass as input a list
    /// with elements: 1, 2, 3, 4, the constructed matrix is ​​expected:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// Note that we pass as input some slice -- reference type. We do not expect the matrix to
    /// holds a reference, clone your data to have ownership.
    ///
    pub fn new(data: &[T; 4]) -> Matrix<T> {
        todo!()
    }

    /// Returns a vector that contains all 4 elements of the matrix, arranged in rows,
    /// left to right and top to bottom wrapped in `Cell`. That is, if the matrix looks like this:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// then `.by_row` to return the elements in order: 1, 2, 3, 4
    ///
    pub fn by_row(&self) -> Vec<Cell<T>> {
        todo!()
    }

    /// Returns a vector that contains all 4 elements of the matrix, arranged by column,
    /// from top to bottom and from left to right, Wrapped in `Cell`. That is, if the matrix looks like this:
    ///
    /// | 1 2 |
    /// | 3 4 |
    ///
    /// then `.by_col` to return the elements in order: 1, 3, 2, 4
    ///
    pub fn by_col(&self) -> Vec<Cell<T>> {
        todo!()
    }
}
```

What data to store in the matrix? You can store the elements as given and construct `Cell`s on iteration, or you can store `Cell`s and clone them. It could be a vector, or it could be something simpler. You can keep them in rows or columns, or even both. Your choice. The important thing is that you have ownership over them, so that you don't end up with lifetime annotations (we will exercise them in future homework and challenges).

Notice that we put derived types on both structs: `Debug`, `Clone`, `PartialEq`. Leave them as we gave them, although technically it doesn't stop you from implementing them yourself (you don't have to).

(**Hide**: can't you write your own implementation of `PartialEq` that just says all cells are equal to each other and sneak through the tests? You can! And if you do, we'll sigh heavily and have to copy-paste eg `assert_ne!(Cell(0), Cell(1))` in each of the affected tests. Please don't.)


## Collection and multiplication of cells

Now we start with the more interesting stuff. Matrices and cells are generic, but for the purpose of the task, we will only work with `String` and `i32`. Moreover, adding numbers and adding strings separately is relatively boring, so you'll only implement addition of `i32` and `String` in that order -- number on the left, string on the right (it's not hard to implement the mirror operation, it's not worth we're wasting your time).

How to add a number-cell to a string-cell? If the number is positive or zero, we simply concatenate the two things as a string with a space between them:

```rust
assert_eq!(Cell(22) + Cell(String::from("years ago")), Cell(String::from("22 years ago")))
assert_eq!(Cell(0) + Cell(String::from("expectation")), Cell(String::from("0 expectation")))
```

If the number is negative, we reverse the text of the string and put it before the number by absolute value:

```rust
assert_eq!(Cell(-4) + Cell(String::from("xirtam")), Cell(String::from("matrix 4")))
```

We expect you to implement only collection of `Cell<i32>` on the left with `Cell<String>` on the right, and the result should be `Cell<String>`. This should be enough information for you to implement collection with the [std::ops::Add](https://doc.rust-lang.org/stable/std/ops/trait.Add.html) trait. As long as you can handle it, the above examples (and the base test) will compile for you.

How to multiply a number-cell by a string-cell? If the number is positive or zero, we repeat the string this many times:

```rust
assert_eq!(Cell(3) * Cell(String::from("boom!")), Cell(String::from("boom!boom!boom!")))
assert_eq!(Cell(0) * Cell(String::from("boom?")), Cell(String::from("")))
```

If the number is negative, we reverse the string text and repeat it that many times

```rust
assert_eq!(Cell(-3) * Cell(String::from(",regdab")), Cell(String::from("badger,badger,badger,")))
```

Just like the above example, the multiplication has `Cell<i32>` on the left side and `Cell<String>` on the right, and the result is `Cell<String>`. Trait is [std::ops::Mul](https://doc.rust-lang.org/stable/std/ops/trait.Mul.html)


## Addition and multiplication of matrices

Knowing how to do cell operations, we can define how to do things with matrices. A significant difference is that addition gives us a new matrix, and multiplication - a single string. We collect matrices element by element -- each element of a certain row and column in the left matrix with the element of the same coordinates in the right:

```
| 1 2 | + | "one"   "two"  | = | "1 one"   "2 two"  |
| 3 4 |   | "three" "four" |   | "3 three" "4 four" |
```

Matrices are multiplied by the "rows by columns" rule -- the elements of the first row of the left matrix are multiplied element by element by the first column of the right, the second row by the second column, and are combined into a single string with spaces:

```
| 1 2 | * | "one"   "two"        | = "one threethree twotwotwo you get it"
| 3 1 |   | "three" "you get it" |
```

Here, "addition" and "multiplication" work according to the rules of the cells above. See the base test for examples, but the types are:

- `Matrix<i32>` + `Matrix<String>` = `Matrix<String>`
- `Matrix<i32>` * `Matrix<String>` = `String`

Traits as above are [std::ops::Add](https://doc.rust-lang.org/stable/std/ops/trait.Add.html) and [std::ops::Mul](https://doc.rust-lang.org/stable/std/ops/trait.Mul.html).


## Important

Make sure you have the Rust compiler and cargo version at least 1.56.0. You can check by calling `rustc --version` and `cargo --version`. Also check that you have the following line in your `Cargo.toml` file:

```
edition = "2021"
```

In case your versions are old, you can use this command to update them:

```
rustup update stable
```
