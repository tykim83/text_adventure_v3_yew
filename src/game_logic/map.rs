use maplit::hashmap;
use std::collections::HashMap;

use super::item::{Item, Objects};

#[derive(Debug, Clone)]
pub struct Map {
    pub rooms: Vec<Room>,
}

impl Map {
    pub fn init() -> Self {
        Self {
            rooms: vec![
                Room {
                    id: Location::GameRoom,
                    name: String::from("Game room"),
                    description: String::from("There is a computer"),
                    exit: hashmap! { Direction::South => Location::Kitchen },
                    items: vec![
                        Item {
                            id: Objects::Table,
                            name: String::from("Table"),
                            description: String::from("It's an old table"),
                            can_picked_up: false,
                        },
                        Item {
                            id: Objects::Key,
                            name: String::from("Key"),
                            description: String::from("It's a key"),
                            can_picked_up: true,
                        },
                    ],
                },
                Room {
                    id: Location::Kitchen,
                    name: String::from("Kitchen"),
                    description: String::from(
                        "There is a table with a key on it. A door leading north.",
                    ),
                    exit: hashmap! { Direction::North => Location::GameRoom },
                    items: vec![],
                },
            ],
        }
    }

    pub fn current_location_mut(&mut self, location: &Location) -> &mut Room {
        // ToDo - remove unwrap()
        self.rooms.iter_mut().find(|c| c.id == *location).unwrap()
    }

    pub fn current_location(&self, location: &Location) -> Room {
        // ToDo - remove unwrap()
        self.rooms
            .iter()
            .find(|c| c.id == *location)
            .unwrap()
            .clone()
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub id: Location,
    pub name: String,
    pub description: String,
    pub exit: HashMap<Direction, Location>,
    pub items: Vec<Item>,
}

impl Room {
    pub fn get_direction(&self, dir: &Direction) -> Option<Location> {
        self.exit.get(dir).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Location {
    GameRoom,
    Kitchen,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
