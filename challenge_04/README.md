# Dungeons and Compilers

[Zork](https://en.wikipedia.org/wiki/Zork) is one of the classic PC games -- you are in a dungeon room, you choose whether to travel north, east, etc. The rooms have treasures, enemies, and various flavor text. We will write the basics of some such game.

First, the basic type definitions we'll be working with:

```rust
/// The various errors we'll expect to return as a result of some invalid operation.
/// More details below:
///
#[derive(Debug)]
pub enum Errors {
    DuplicateRoom(String),
    UnknownRoom(String),
    IoError(std::io::Error),
    LineParseError { line_number: usize },
    DirectionParseError(String),
}

/// The four directions in which a room can have neighbors. You can add more traits
/// implementations to make your life easier.
///
#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// A room in the dungeons. It is defined by name only, although in a more interesting implementation it might
/// holds items, enemies...
///
pub struct Room {
    pub name: String,
    // Other fields you need
}

/// Container for the rooms and more. We will mostly work with this structure.
///
pub struct Dungeon {
    // Fields you need
}
```


## Building Dungeon

We have a `Dungeon` structure, what to do with it?

```rust
impl Dungeon {
    /// Constructs an empty Dungeon that does not have any rooms.
    ///
    pub fn new() -> Self {
        todo!()
    }

    /// Adding room to Dungeon named `name`. Returns `Ok(())` on success. If there is already a room with
    /// such a name, we expect you to return `Errors::DuplicateRoom` with the name.
    ///
    pub fn add_room(&mut self, name: &str) -> Result<(), Errors> {
        todo!()
    }

    /// Reading a given room -- when we call `get_room`, we expect a reference to `Room`
    /// the structure with this name.
    ///
    /// If there is no such room, we expect `Errors::UnknownRoom` with the supplied name.
    ///
    pub fn get_room(&self, room_name: &str) -> Result<&Room, Errors> {
        todo!()
    }

    /// Add a room neighbor. After calling the function, we expect the room with a name
    /// `room_name` has a connection in the `direction` direction with the room named `other_room_name`.
    ///
    /// We also expect `other_room_name` to relate to `room_name` in the *reverse* direction.
    ///
    /// A successful result is `Ok(())`. In the event that one of the two rooms does not exist, we expect an error
    /// `Errors::UnknownRoom` with the corresponding missing room name. If both are missing, then
    /// return the one you check first.
    ///
    pub fn set_link(
        &mut self,
        room_name: &str,
        direction: Direction,
        other_room_name: &str,
    ) -> Result<(), Errors> {
        todo!()
    }

    /// Reading neighbor of room named `room_name` in direction `direction`. There are several here
    /// the output variant:
    ///
    /// - If the room passed does not exist, we expect an `Errors::UnknownRoom` error
    /// - If the given room has no neighbor in that direction, Ok(None) is the meaningful result
    /// - Otherwise, we wait for a reference to the `Room` structure of the neighbor in question, wrapped in `Ok(Some(`.
    ///
    pub fn get_next_room(&self, room_name: &str, direction: Direction) -> Result<Option<&Room>, Errors> {
        todo!()
    }
}
```

Here's an example dungeon build:

```rust
let mut dungeon = Dungeon::new();

dungeon.add_room("Entrance").unwrap();
dungeon.add_room("Hallway").unwrap();
dungeon.set_link("Entrance", Direction::East, "Hallway").unwrap();

assert_eq!(dungeon.get_room("Hallway").unwrap().name, "Hallway");
assert_eq!(dungeon.get_next_room("Hallway", Direction::West).unwrap().unwrap().name, "Entrance");
```

Note what we noted above -- adding a link from "Entrance" to "Hallway" in the east means adding the reverse link in the west.

Is it possible to construct two links that are overwritten? Yes, no problem:

```rust
let mut dungeon = Dungeon::new();

dungeon.add_room("Entrance").unwrap();
dungeon.add_room("Hallway").unwrap();
dungeon.add_room("Magic Lab").unwrap();

dungeon.set_link("Entrance", Direction::East, "Hallway").unwrap();
dungeon.set_link("Hallway", Direction::West, "Magic Lab").unwrap();

assert_eq!(dungeon.get_next_room("Entrance", Direction::East).unwrap().unwrap().name, "Hallway");
assert_eq!(dungeon.get_next_room("Hallway", Direction::West).unwrap().unwrap().name, "Magic Lab");
```

We go east from the entrance and find ourselves in a corridor. We return to the west and suddenly we are in some kind of magical laboratory. That's just how magic dungeons roll, go with it. When we say ".set_link adds links in both directions", just do it yourself, and if further calls change something, no problem. There are no problems with .set_link overwriting forward links in the running process.

It is also quite possible to create a connection from one room to the same room. It can also have no loop.

(**Hint**: If you're wondering exactly how to store the data in Room structures so that you can easily access them by name, a HashMap is probably the easiest option.)


## Parsing

Here's an example file we'll want to be able to read that follows the structure above:

```
## Rooms
- Entrance
- Hallway
- Magic Lab

## Links
- Entrance -> East -> Hallway
- Hallway -> West -> Magic Lab
```

Here is the structure of the function we will use:

```rust
impl Dungeon {
    /// We read the dungeon structure from something that implements `BufRead`. This could be it
    /// file, or, if we're testing, it could just be a collection of bytes.
    ///
    /// A successful result returns the newly created dungeon packaged in `Ok`.
    ///
    /// See below for an explanation of the errors we expect.
    ///
    pub fn from_reader<B: BufRead>(reader: B) -> Result<Self, Errors> {
        todo!()
    }
}
```

The first line should always be `## Rooms`. Otherwise, return `LineParseError` with line number 1. After that, each subsequent line is expected to either start with `-` and the name of the room to add (`add_room`), or an empty line, which means we're done adding rooms. If it's anything else, `LineParseError` with the corresponding line (first line is 1).

Any errors that may occur when calling `add_room` are expected to be returned by this function if they occur.

Any `std::io::Error` errors when reading the reader are expected to be wrapped in `Errors::IoError` and returned.

The next line is expected to be exactly `## Links`. Otherwise, we expect a `LineParseError` with the line number. From there, all subsequent lines should have the form:

```
- <room name> -> <direction> -> <other room>
```

That is, the row starts with the string `"- "`, and individual rooms are separated by the string `" -> "`. Assume that room names will not contain `->` and even `->`. Whether it is up to you to clear the spaces around the name, we will not test with room names that start or end with whitespace.

As you may remember, feed the hashed content of this line to `set_link`. If the direction is not one of North/South/East/West, we expect an `Errors::DirectionParseError` with the string you are trying to parse into a direction. If there is any error in the format of this line, `LineParseError` with the corresponding line number.

If `set_link` returns an error, we expect you to return it from this function.

In case the function is passed an empty reader, we expect a `LineParseError` with line number 0.

Here's the prototype of a helper function that we won't test directly (it's not `pub`, we won't call it in the tests), but it might make your life easier:

```rust
/// match_prefix("- ", "- Foo") //=> Some("Foo")
/// match_prefix("- ", "Bar")   //=> None
///
fn match_prefix<'a, 'b>(prefix: &'a str, input: &'b str) -> Option<&'b str> {
    todo!()
}
```

You are free to use it or not, you are free to write a different similar function. If lifetime annotations bother you, we certainly advise you to implement and use it, and read the "Lifetimes" lecture until they stop bothering you. We have just such an example somewhere in the slides.

Finally, after we have constructed a dungeon, let's do something interesting with it.


## Pathfinding

It would make sense to do things like find a path to the treasure that bypasses opponents, or maybe seek out opponents for maximum conflict and leveling up. Of course, we don't have treasure, enemies, or levels here, so let's just write a simple wayfinding from one room to another:

```rust
impl Dungeon {
    /// Searches for a path from `start_room_name` to `end_room_name` and returns it in a vector packed in
    /// `Ok(Some(` if found.
    ///
    /// If there is no path between these two rooms, returns `Ok(None)`.
    ///
    /// If reading rooms at some point returns an error, we expect you to raise the error.
    ///
    pub fn find_path(
        &self,
        start_room_name: &str,
        end_room_name: &str
    ) -> Result<Option<Vec<&Room>>, Errors> {
        todo!()
    }
}
```

What algorithm to use? Whatever you want -- we won't check the result path directly, we'll check:

- Is it a road -- each room must have a direct connection to the next in some direction
- Are the start and end the right rooms

Something easy you can use is [Breadth-first search](https://en.wikipedia.org/wiki/Breadth-first_search). The algorithm is pretty well described on wikipedia, although there is no logic to find the full path, only the node is found. They mention that you can hold parents, but don't show where it fits in the code. Here's a summary:

1. You prepare a queue in which you put the initial room (the queue can be a vector, or there may be another suitable structure in [std::collections](https://doc.rust-lang.org/stable/std/collections/index.html) that will work for you)
2. You mark the starting room as "visited" (maybe in a vector of visited rooms, maybe something else from [std::collections](https://doc.rust-lang.org/stable/std/collections/index.html))
3. You prepare a `HashMap` of links from a room to its parent.
4. You start pulling rooms from the end of the visit queue while there are rooms in it:
   - If you reach the end room, you may end the cycle
   - If not, you go through all the neighbors of a room, note that their parent is the current one, mark them as "visited" and add them to the top of the queue.
5. We have completed the cycle. Is there a parent marked for the end room?
   - If not, then we haven't reached the end room, we can safely return `Ok(None)`
   - If so, we must construct our way back to the beginning, parent by parent. We start with the end room and its parent, and we put them into a vector, we keep taking the front parent and the front parent until we get to a room with no parent -- that will be the starting one. We return the path in "straight" order -- start to end room.

If it seems disturbingly complicated -- try it, go step by step, it will work. If it seems boringly simple, by all means implement whatever search interests you.


## Advices

- There is a lot of error handling that can be simplified by using some methods from `Result` or from `Option`. Feel free to review the [Documentation](https://doc.rust-lang.org/stable/std/option/enum.Option.html)
- Nothing prevents us from finding a path from one room to the same room -- it's just a path with that one room. Also, there will be nothing surprising in that there will be a cycle of rooms. If you implement the above algorithm, you shouldn't have any problems, but it's definitely a good idea to test.
- HashMap is an incredibly convenient structure that is used for a great many things and is built-in in many languages. It's a dictionary where the key is something that implements the Hash trait (very often a String) and the value can be anything. The documentation has great examples, but we might as well do a bonus demo video where we show use.
