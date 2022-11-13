# Dungeon Map

This challenge task will involve displaying the rooms in the dungeon from the previous challenge task 4.

Let's consider the `Room` structure from challenge 04 to be like below:

```rust
pub struct Room {
    pub name: String,
    pub north: String,
    pub south: String,
    pub east: String,
    pub west: String,
}
```

Define a `Display` implementation to allow us to visualize a room and its neighbors:

```rust
let room = Room {
    name: String::from("Living Room"),
    north: String::from("Balcony"),
    south: String::from("Bathroom"),
    east: String::from("Kitchen"),
    west: String::from("Entrance"),
};

println!("{}", room);
```

The output should look like this:

```
           [   Balcony   ]
                  |
           +------N------+
Entrance - | Living Room | - Kitchen
           +------S------+
                  |
           [  Bathroom   ]
```


Let's go through the rules:

- The room name is surrounded by a box that has `+` symbols in the corners and `N` and `S` above and below centered on the sides of the box. If the room name has an even number of characters like "Living Room", add a space at the end to make it odd (so that the `N` and `S` are always right in the middle).
- The same goes for centering the north and south rooms -- if "center" is between two characters, assume there is an extra space to the right. When centering "Place" for example, the dash is above the "a" and not above the "l".
- The square brackets (`[]`) are aligned with the walls of the room box. You can assume that neither north nor south room will be longer than `name` -- we won't test with such examples. (If you want to implement this case, an option is to trim off too-long names, just as it's an option to choose the maximum length among the three for common.)
- Also assume that we won't be testing with empty strings for rooms.
- We expect no trailing whitespace after the text. That is, if you find a way somewhere in the standard library to center, great, but note that "center" means padding with spaces and to the right, which will fail the test below.
- We expect the first character written to be exactly one newline (symbol `\n`). So that if we print with `println!`, we can make sure that it starts on a new line. At the end of the string, however, no new line is output -- `println!` would print. You should pass the test below, basically.

```rust
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
```

As always, the challenge is all or nothing, so test carefully. The task shouldn't contain anything complicated from Rust features, but it has the potential to be annoying with centering and off-by-one errors. But that's why it's for bonus points, anyway -- you have our permission to stop tinkering for a moment and concentrate on your projects :).

For extra hard mode, try to minimize string allocations and keep the code clear and simple. You won't get bonus points, but it might bring you moral satisfaction (if you succeed).
