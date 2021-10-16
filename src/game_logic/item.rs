#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub id: Objects,
    pub name: String,
    pub description: String,
    pub can_picked_up: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Objects {
    Key,
    Table,
    GameRoomDoor,
    Computer,
    Game,
    NotFound,
}