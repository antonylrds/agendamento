use time::UtcDateTime;

use crate::model::{ClassId, RoomId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScheduleId(pub u64);

pub struct Schedule {
    pub id: Option<ScheduleId>,
    pub class_id: ClassId,
    pub room_id: RoomId,
    pub time: UtcDateTime,
}
