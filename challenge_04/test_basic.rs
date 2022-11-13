// Note: the project name should be "solution". If it doesn't say that, change it
// on this line:
use solution::*;

#[test]
fn test_basic_1() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Hallway").unwrap();
    dungeon.set_link("Entrance", Direction::East, "Hallway").unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
}

const TEST_INPUT_1: &str = "
## Rooms
- Entrance
- Hallway

## Links
- Entrance -> East -> Hallway
";

#[test]
fn test_basic_2() {
    // .trim() to remove the first and last lines:
    let dungeon = Dungeon::from_reader(TEST_INPUT_1.trim().as_bytes()).unwrap();

    assert_eq!(dungeon.get_room("Entrance").unwrap().name, "Entrance");
    assert_eq!(dungeon.get_room("Hallway").unwrap().name, "Hallway");

    assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
}

#[test]
fn test_basic_3() {
    let mut dungeon = Dungeon::new();

    dungeon.add_room("Entrance").unwrap();
    dungeon.add_room("Treasure Room").unwrap();
    dungeon.set_link("Entrance", Direction::West, "Treasure Room").unwrap();

    let path = dungeon.find_path("Entrance", "Treasure Room").unwrap().unwrap();
    assert!(path.len() > 0);
}
