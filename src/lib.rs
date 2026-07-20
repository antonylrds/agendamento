use std::collections::HashMap;
use time::Date;

pub struct Event {
    pub name: String,
    pub date: Date,
}

pub type EventId = u64;

#[derive(Default)]
pub struct EventStore {
    events: HashMap<EventId, Event>,
    next_id: EventId,
}

impl EventStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, event: Event) -> EventId {
        let id = self.next_id;
        self.next_id += 1;
        self.events.insert(id, event);
        id
    }

    pub fn get(&self, id: EventId) -> Option<&Event> {
        self.events.get(&id)
    }

    pub fn all(&self) -> impl Iterator<Item = (&EventId, &Event)> {
        self.events.iter()
    }
}
