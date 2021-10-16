use maplit::hashmap;
use std::collections::HashMap;

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
                },
                Room {
                    id: Location::Kitchen,
                    name: String::from("Kitchen"),
                    description: String::from(
                        "There is a table with a key on it. A door leading north.",
                    ),
                    exit: hashmap! { Direction::North => Location::GameRoom },
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
