use agendamento::model::{Class, ClassId, Room, Schedule, Student};
use time::macros::datetime;
use agendamento::repository::{InMemoryDatabase, Repository};

fn create_students(db: &mut InMemoryDatabase, class_id: ClassId) {
    let students = vec![
        Student {
            id: None,
            name: "Alice".to_string(),
            class: Some(class_id),
        },
        Student {
            id: None,
            name: "Bob".to_string(),
            class: Some(class_id),
        },
        Student {
            id: None,
            name: "Charlie".to_string(),
            class: Some(class_id),
        },
    ];

    for student in students {
        db.add(student);
    }
}

fn main() {
    let mut db = InMemoryDatabase::new();
    let class_id = db.add(Class {
        id: None,
        name: "Math".to_string(),
        students: Vec::new(),
    });

    create_students(&mut db, class_id);

    let room_id = db.add(Room {
        id: None,
        name: "Room 101".to_string(),
    });

    let schedule = db.add(Schedule {
        id: None,
        class_id,
        room_id,
        time: datetime!(2026-07-26 10:00 UTC).to_utc(),
    });

    for (id, schedule) in Repository::<Schedule>::all(&db) {
        let class_name = Repository::<Class>::get(&db, schedule.class_id)
            .map(|c| c.name.as_str())
            .unwrap_or("None");
        let room_name = Repository::<Room>::get(&db, schedule.room_id)
            .map(|r| r.name.as_str())
            .unwrap_or("None");
        println!(
            "{}: Class: {}, Room: {}, Time: {}",
            id, class_name, room_name, schedule.time
        );
    }


}
