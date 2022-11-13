// Note: the project name should be "solution". If it doesn't say that, change it
// on this line:
use solution::*;

#[test]
fn test_iteration_1() {
    let data = [1, 2,
                3, 4,
                5, 6];
    let matrix = Matrix::new(3, 2, &data);

    assert_eq!(matrix.by_row().collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6]);
    assert_eq!(matrix.by_col().collect::<Vec<_>>(), vec![&1, &3, &5, &2, &4, &6]);
}

#[test]
fn test_iteration_2() {
    let data = [1, 2, 3,
                4, 5, 6];
    let matrix = Matrix::new(2, 3, &data);

    assert_eq!(matrix.by_row().collect::<Vec<_>>(), vec![&1, &2, &3, &4, &5, &6]);
    assert_eq!(matrix.by_col().collect::<Vec<_>>(), vec![&1, &4, &2, &5, &3, &6]);
}

#[test]
fn test_iteration_3() {
    let data = ["1", "2", "3",
                "4", "5", "6",
                "7", "8", "9"];
    let matrix = Matrix::new(3, 3, &data);

    assert_eq!(
        matrix.by_row().collect::<Vec<_>>(),
        vec![&"1", &"2", &"3", &"4", &"5", &"6", &"7", &"8", &"9"]
    );
    assert_eq!(
        matrix.by_col().collect::<Vec<_>>(),
        vec![&"1", &"4", &"7", &"2", &"5", &"8", &"3", &"6", &"9"]
    );
}