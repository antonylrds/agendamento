use std::fmt;
use time::UtcDateTime;

use crate::model::{ClassId, RoomId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScheduleId(pub u64);

impl fmt::Display for ScheduleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Schedule {
    pub id: Option<ScheduleId>,
    pub class_id: ClassId,
    pub room_id: RoomId,
    pub time: UtcDateTime,
}
