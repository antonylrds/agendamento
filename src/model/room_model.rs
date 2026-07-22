#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RoomId(pub u64);

pub struct Room {
    pub id: Option<RoomId>,
    pub name: String,
}
