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
        let rooms = &mut self.rooms.clone();
        let room_names = rooms.clone().iter().map(|x| x.name.clone()).collect::<Vec<String>>();

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
            *self = Dungeon { rooms: (*rooms.clone()).to_vec() };

            return Ok(());
        }
    }

    pub fn get_room(&self, room_name: &str) -> Result<&Room, Errors> {
        let room_names = &self.rooms.clone().iter().map(|x| x.name.clone()).collect::<Vec<String>>();

        if room_names.contains(&String::from(room_name.clone())) {
            return Ok(&self.rooms.iter().find(|x| x.name == String::from(room_name)).unwrap());
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
        let rooms = &mut self.rooms.clone();
        let room_names = rooms.clone().iter().map(|x| x.name.clone()).collect::<Vec<String>>();
        let room_exists = room_names.clone().contains(&String::from(room_name.clone()));
        let other_room_exists = room_names.clone().contains(&String::from(other_room_name.clone()));


        if room_exists.clone() && other_room_exists.clone() {
            match direction {
                Direction::North => {
                    let _new_rooms = rooms.iter().map(|x| {
                        let mut new_x = x.clone();
                        if new_x.name == String::from(room_name.clone()) {
                            new_x.north = Some(String::from(other_room_name.clone()));
                        }
                        if new_x.name == String::from(other_room_name.clone()) {
                            new_x.south = Some(String::from(room_name.clone()));
                        }
                        return new_x;
                    }).collect::<Vec<Room>>();

                    *self = Dungeon { rooms: _new_rooms };
                },
                Direction::South => {
                    let _new_rooms = rooms.iter().map(|x| {
                        let mut new_x = x.clone();
                        if new_x.name == String::from(room_name.clone()) {
                            new_x.south = Some(String::from(other_room_name.clone()));
                        }
                        if new_x.name == String::from(other_room_name.clone()) {
                            new_x.north = Some(String::from(room_name.clone()));
                        }
                        return new_x;
                    }).collect::<Vec<Room>>();

                    *self = Dungeon { rooms: _new_rooms };
                },
                Direction::East => {
                    let _new_rooms = rooms.iter().map(|x| {
                        let mut new_x = x.clone();
                        if new_x.name == String::from(room_name.clone()) {
                            new_x.east = Some(String::from(other_room_name.clone()));
                        }
                        if new_x.name == String::from(other_room_name.clone()) {
                            new_x.west = Some(String::from(room_name.clone()));
                        }
                        return new_x;
                    }).collect::<Vec<Room>>();

                    *self = Dungeon { rooms: _new_rooms };
                },
                Direction::West => {
                    let _new_rooms = rooms.iter().map(|x| {
                        let mut new_x = x.clone();
                        if new_x.name == String::from(room_name.clone()) {
                            new_x.west = Some(String::from(other_room_name.clone()));
                        }
                        if new_x.name == String::from(other_room_name.clone()) {
                            new_x.east = Some(String::from(room_name.clone()));
                        }
                        return new_x;
                    }).collect::<Vec<Room>>();

                    *self = Dungeon { rooms: _new_rooms };
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
        let room_names = &self.rooms.clone().iter().map(|x| x.name.clone()).collect::<Vec<String>>();

        if room_names.contains(&String::from(room_name.clone())) {
            let room = &self.rooms.iter().find(|x| x.name == room_name);
            match direction {
                Direction::North => {
                    if room.clone().unwrap().north == None {
                        return Ok(None);
                    } else {
                        let next_room = &self.rooms.iter().find(|x| x.name == room.clone().unwrap().north.clone().unwrap());
                        Ok(*next_room)
                    }
                },
                Direction::South => {
                    if room.clone().unwrap().south == None {
                        return Ok(None);
                    } else {
                        let next_room = &self.rooms.iter().find(|x| x.name == room.clone().unwrap().south.clone().unwrap());
                        Ok(*next_room)
                    }
                },
                Direction::East => {
                    if room.clone().unwrap().east == None {
                        return Ok(None);
                    } else {
                        let next_room = &self.rooms.iter().find(|x| x.name == room.clone().unwrap().east.clone().unwrap());
                        Ok(*next_room)
                    }
                },
                Direction::West => {
                    if room.clone().unwrap().west == None {
                        return Ok(None);
                    } else {
                        let next_room = &self.rooms.iter().find(|x| x.name == room.clone().unwrap().west.clone().unwrap());
                        Ok(*next_room)
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