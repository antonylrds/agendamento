use std::collections::HashMap;

use crate::model::{Class, ClassId, Room, RoomId, Schedule, ScheduleId, Student, StudentId};

#[derive(Default)]
pub struct Database {
    sequence_student_id: StudentId,
    students: HashMap<StudentId, Student>,
    sequence_class_id: ClassId,
    classes: HashMap<ClassId, Class>,
    sequence_room_id: RoomId,
    rooms: HashMap<RoomId, Room>,
    sequence_schedule_id: ScheduleId,
    schedules: HashMap<ScheduleId, Schedule>,
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_student(&mut self, mut student: Student) -> StudentId {
        let id = self.sequence_student_id;
        self.sequence_student_id += 1;
        student.id = Some(id);
        self.students.insert(id, student);
        id
    }

    pub fn get_student(&self, id: StudentId) -> Option<&Student> {
        self.students.get(&id)
    }

    pub fn all_students(&self) -> impl Iterator<Item = (&StudentId, &Student)> {
        self.students.iter()
    }

    pub fn add_class(&mut self, mut class: Class) -> ClassId {
        let id = self.sequence_class_id;
        self.sequence_class_id += 1;
        class.id = Some(id);
        self.classes.insert(id, class);
        id
    }

    pub fn get_class(&self, id: ClassId) -> Option<&Class> {
        self.classes.get(&id)
    }

    fn get_class_mut(&mut self, id: ClassId) -> Option<&mut Class> {
        self.classes.get_mut(&id)
    }

    pub fn all_classes(&self) -> impl Iterator<Item = (&ClassId, &Class)> {
        self.classes.iter()
    }

    pub fn add_room(&mut self, mut room: Room) -> RoomId {
        let id = self.sequence_room_id;
        self.sequence_room_id += 1;
        room.id = Some(id);
        self.rooms.insert(id, room);
        id
    }

    pub fn get_room(&self, id: RoomId) -> Option<&Room> {
        self.rooms.get(&id)
    }

    pub fn all_rooms(&self) -> impl Iterator<Item = (&RoomId, &Room)> {
        self.rooms.iter()
    }

    pub fn add_students_to_class(&mut self, student_ids: Vec<StudentId>, class_id: ClassId) -> Result<(), String> {
        let mut students = Vec::with_capacity(student_ids.len());
        for student_id in &student_ids {
            let student = self.get_student(*student_id).ok_or("Student not found")?;
            students.push(student.clone());
        }

        let class = self.get_class_mut(class_id).ok_or("Class not found")?;
        class.students.extend(students);
        Ok(())
    }

    pub fn add_schedule(&mut self, mut schedule: Schedule) -> ScheduleId {
        let id = self.sequence_schedule_id;
        self.sequence_schedule_id += 1;
        schedule.id = Some(id);
        self.schedules.insert(id, schedule);
        id
    }
}
