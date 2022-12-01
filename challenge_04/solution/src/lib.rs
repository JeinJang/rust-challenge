use std::str;
use std::io::{BufRead};
use std::collections::VecDeque;
use std::collections::HashMap;

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

impl Direction {
    pub fn get_direction(direction: &str) -> Result<Self, Errors> {
        if direction == "South" {
            return Ok(Self::South);
        } else if direction == "North" {
            return Ok(Self::North);
        } else if direction == "East" {
            return Ok(Self::East);
        } else if direction == "West" {
            return Ok(Self::West);
        } else {
            return Err(Errors::DirectionParseError(String::from(direction)));
        }
    }
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


    pub fn from_reader<B: BufRead>(reader: B) -> Result<Self, Errors> {
        // line number
        let mut line_number = 0;
        // line number: link로 넘어갈 때 초기화됨
        let mut _index = 0;
        let mut current = 0;

        let mut new_dungeon = Self::new();
        
        for value in reader.lines() {
            // io error
            match value {
                Ok(..) => (),
                Err(error) => return Err(Errors::IoError(error)),
            };

            let line = &value.unwrap();

            if _index == 0 && ((current == 0 && line != "## Rooms") || (current == 1 && line != "## Links")) {
                return Err(Errors::LineParseError { line_number: line_number + 1 });
            }
            if _index > 0 && line != "" {
                if !line.starts_with("- ") {
                    return Err(Errors::LineParseError { line_number: line_number + 1 });
                }

                let new_line = &line[2..line.len()];

                if current == 0 {
                    let result = new_dungeon.add_room(new_line);
                    match result {
                        Ok(()) => result.unwrap(),
                        Err(error) => return Err(error),
                    };
                } else {
                    let link_info = new_line.split(" -> ").collect::<Vec<&str>>();
                    let direction = Direction::get_direction(link_info[1]);

                    match direction {
                        Ok(value) => {
                            let result = new_dungeon.set_link(link_info[0], value, link_info[2]);

                            match result {
                                Ok(()) => result.unwrap(),
                                Err(error) => return Err(error),
                            };
                        },
                        Err(error) => return Err(error),
                    }
                }
            }

            _index += 1;
            line_number += 1;

            if _index > 0 && current == 0 && line == "" {
                current += 1;
                _index = 0;
            }
        }

        // empty reader
        if _index == 0 && current == 0 {
            return Err(Errors::LineParseError { line_number: 0 });
        }

        return Ok(new_dungeon);
    }

    pub fn find_path(
        &self,
        start_room_name: &str,
        end_room_name: &str
    ) -> Result<Option<Vec<&Room>>, Errors> {
        let mut q = VecDeque::new();
        let mut visited = vec![String::from(start_room_name.clone())];
        let mut map = HashMap::new();

        q.push_back(String::from(start_room_name.clone()));

        // 있는 방인지 없는 방인지 여부 확인
        match self.clone().get_room(start_room_name.clone()) {
            Ok(_value) => (),
            Err(error) => return Err(error),
        };

        match self.clone().get_room(end_room_name.clone()) {
            Ok(_value) => (),
            Err(error) => return Err(error),
        };

        // reflexive
        if start_room_name.clone() == end_room_name.clone() {
            return Ok(Some(self.rooms.iter().filter(|x| x.name == start_room_name).collect()));
        }

        for room in &self.rooms.clone() {
            let mut children = vec![];
            if room.clone().north != None {
                children.push(String::from(room.clone().north.unwrap()));
            }
            if room.clone().south != None {
                children.push(String::from(room.clone().south.unwrap()));
            }
            if room.clone().east != None {
                children.push(String::from(room.clone().east.unwrap()));
            }
            if room.clone().west != None {
                children.push(String::from(room.clone().west.unwrap()));
            }
            map.insert(room.clone().name, children);
        }

        let mut path_map = HashMap::new();

        while let Some(t) = q.pop_front() {
            let children = map.get(&t);

            for child in children.clone().unwrap() {
                if !visited.contains(&child.clone()) {
                    visited.push(child.clone());
                    q.push_back(child.clone());

                    path_map.insert(child.clone(), t.clone());
                }
            }

            if end_room_name.clone() == String::from(t.clone()) {
                break;
            }
        }

        if visited.contains(&String::from(end_room_name.clone())) {
            let mut path_room = String::from(end_room_name.clone());
            let mut _path = vec![self.clone().get_room(&path_room.clone()).unwrap()];

            while path_room != String::from(start_room_name.clone()) {
                let ele = &path_map.get(&path_room).unwrap();
                let target = &self.clone().get_room(ele.clone()).unwrap().name;
                _path.push(self.clone().get_room(&target).unwrap());
                path_room = (ele.clone()).to_string();
            }

            return Ok(Some(_path.into_iter().rev().collect()));
        } else {
            return Ok(None);
        }
    }
}