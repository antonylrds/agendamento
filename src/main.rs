use agendamento::{Event, EventStore};
use time::macros::date;

fn main() {
    let mut store = EventStore::new();

    let id = store.add(Event {
        name: "Standup".to_string(),
        date: date!(2026 - 07 - 20),
    });

    if let Some(event) = store.get(id) {
        println!("Event #{id}: {} on {}", event.name, event.date);
    }
}
