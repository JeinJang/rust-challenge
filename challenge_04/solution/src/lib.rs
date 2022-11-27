#[derive(Debug)]
pub enum Errors {
    DuplicateRoom(String),
    UnknownRoom(String),
    IoError(std::io::Error),
    LineParseError { line_number: usize },
    DirectionParseError(String),
}

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone)]
pub struct Room {
    pub name: String,
    pub north: Option<String>,
    pub south: Option<String>,
    pub east: Option<String>,
    pub west: Option<String>,
}

pub struct Dungeon {
    rooms: Vec<Room>,
}

impl Dungeon {
    pub fn new() -> Self {
        return Dungeon {
            rooms: vec![]
        };
    }

    pub fn add_room(&mut self, name: &str) -> Result<(), Errors> {
        let rooms = &self.rooms.clone();
        let room_names = rooms.clone().iter().map(|x| x.name).collect::<Vec<String>>();

        if room_names.contains(&String::from(name.clone())) {
            return Err(Errors::DuplicateRoom(String::from(name)));
        } else {
            let new_room = Room { 
                name: String::from(name),
                north: None,
                south: None,
                east: None,
                west: None,
            };
            rooms.push(new_room);
            &self.rooms = rooms;
            return Ok(());
        }
    }

    pub fn get_room(&self, room_name: &str) -> Result<&Room, Errors> {
        let rooms = &self.rooms.clone();
        let room_names = rooms.clone().iter().map(|x| x.name).collect::<Vec<String>>();

        if room_names.contains(&String::from(room_name.clone())) {
            return Ok(rooms.iter().find(|x| x.name == String::from(room_name)).unwrap());
        } else {
            return Err(Errors::UnknownRoom(String::from(room_name)));
        }
    }

    pub fn set_link(
        &mut self,
        room_name: &str,
        direction: Direction,
        other_room_name: &str,
    ) -> Result<(), Errors> {
        let rooms = &self.rooms.clone();
        let room_names = rooms.clone().iter().map(|x| x.name).collect::<Vec<String>>();
        let room_exists = room_names.clone().contains(&String::from(room_name.clone()));
        let other_room_exists = room_names.clone().contains(&String::from(other_room_name.clone()));


        if room_exists.clone() && other_room_exists.clone() {
            match direction {
                Direction::North => {
                    let new_rooms = rooms.iter().for_each(|x| {
                        if x.name == String::from(room_name.clone()) {
                            x.north = Some(String::from(other_room_name.clone()));
                        }
                        if x.name == String::from(other_room_name.clone()) {
                            x.south = Some(String::from(room_name.clone()));
                        }
                    });

                    &self.rooms = new_rooms;
                },
                Direction::South => {
                    let new_rooms = rooms.iter().for_each(|x| {
                        if x.name == String::from(room_name.clone()) {
                            x.south = Some(String::from(other_room_name.clone()));
                        }
                        if x.name == String::from(other_room_name.clone()) {
                            x.north = Some(String::from(room_name.clone()));
                        }
                    });

                    &self.rooms = new_rooms;
                },
                Direction::East => {
                    let new_rooms = rooms.iter().for_each(|x| {
                        if x.name == String::from(room_name.clone()) {
                            x.east = Some(String::from(other_room_name.clone()));
                        }
                        if x.name == String::from(other_room_name.clone()) {
                            x.west = Some(String::from(room_name.clone()));
                        }
                    });

                    &self.rooms = new_rooms;
                },
                Direction::West => {
                    let new_rooms = rooms.iter().for_each(|x| {
                        if x.name == String::from(room_name.clone()) {
                            x.west = Some(String::from(other_room_name.clone()));
                        }
                        if x.name == String::from(other_room_name.clone()) {
                            x.east = Some(String::from(room_name.clone()));
                        }
                    });

                    &self.rooms = new_rooms;
                },
            }
            return Ok(());
        } else if other_room_exists.clone() {
            return Err(Errors::UnknownRoom(String::from(other_room_name)));
        } else {
            return Err(Errors::UnknownRoom(String::from(room_name)));
        }
    }

    pub fn get_next_room(&self, room_name: &str, direction: Direction) -> Result<Option<&Room>, Errors> {
        let rooms = &self.rooms.clone();
        let room_names = rooms.clone().iter().map(|x| x.name).collect::<Vec<String>>();

        if room_names.contains(&String::from(room_name.clone())) {
            let room = rooms.clone().iter().find(|x| x.name == room_name);
            match direction {
                Direction::North => {
                    if room.clone().north == None {
                        return Ok(None);
                    } else {
                        return Ok(rooms.iter().find(|x| x.name == room.north));
                    }
                },
                Direction::South => {
                    if room.clone().south == None {
                        return Ok(None);
                    } else {
                        return Ok(rooms.iter().find(|x| x.name == room.south));
                    }
                },
                Direction::East => {
                    if room.clone().east == None {
                        return Ok(None);
                    } else {
                        return Ok(rooms.iter().find(|x| x.name == room.east));
                    }
                },
                Direction::West => {
                    if room.clone().west == None {
                        return Ok(None);
                    } else {
                        return Ok(rooms.iter().find(|x| x.name == room.west));
                    }
                },
            }
        } else {
            return Err(Errors::UnknownRoom(String::from(room_name)));
        }
    }

    // fn match_prefix<'a, 'b>(prefix: &'a str, input: &'b str) -> Option<&'b str> {
    //     todo!()
    // }

    // pub fn find_path(
    //     &self,
    //     start_room_name: &str,
    //     end_room_name: &str
    // ) -> Result<Option<Vec<&Room>>, Errors> {
    //     todo!()
    // }
}