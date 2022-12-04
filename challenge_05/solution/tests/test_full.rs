// Note: the project name should be "solution". If it doesn't say that, change it
// on this line:
use solution::*;

#[test]
fn test_basic() {
    let room = Room {
        name: String::from("Living Room"),
        north: String::from("Balcony"),
        south: String::from("Bathroom"),
        east: String::from("Kitchen"),
        west: String::from("Entrance"),
    };

    let expected = "
           [   Balcony   ]
                  |
           +------N------+
Entrance - | Living Room | - Kitchen
           +------S------+
                  |
           [  Bathroom   ]";

    assert_eq!(format!("{}", room), expected);
}

#[test]
fn test_another() {
    let room = Room {
        name: String::from("Office"),
        north: String::from("Garage"),
        south: String::from("Hall"),
        east: String::from("Study"),
        west: String::from("Room"),
    };

    let expected = "
       [ Garage  ]
            |
       +----N----+
Room - | Office  | - Study
       +----S----+
            |
       [  Hall   ]";

    assert_eq!(format!("{}", room), expected);
}

#[test]
fn test_short() {
    let room = Room {
        name: String::from("X"),
        north: String::from("N"),
        south: String::from("S"),
        east: String::from("E"),
        west: String::from("W"),
    };

    let expected = "
    [ N ]
      |
    +-N-+
W - | X | - E
    +-S-+
      |
    [ S ]";

    assert_eq!(format!("{}", room), expected);
}
