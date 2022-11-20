use crate::room;

pub struct User {
    pub rooms: Vec<room::Room>,
}

impl User {
    pub fn new() -> User {
        User { rooms: Vec::new() }
    }

    pub fn add_room(mut self, r: room::Room) {
        self.rooms.push(r);
    }
}