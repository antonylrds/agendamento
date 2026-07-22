use time::UtcDateTime;

use crate::model::{ClassId, RoomId};

pub type ScheduleId = u64;

pub struct Schedule {
    pub id: Option<ScheduleId>,
    pub class_id: ClassId,
    pub room_id: RoomId,
    pub time: UtcDateTime,
}
