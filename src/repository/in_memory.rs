use std::collections::HashMap;

use crate::model::{Class, ClassId, Room, RoomId, Schedule, ScheduleId, Student, StudentId};
use crate::repository::Repository;

#[derive(Default)]
pub struct InMemoryDatabase {
    sequence_student_id: u64,
    students: HashMap<StudentId, Student>,
    sequence_class_id: u64,
    classes: HashMap<ClassId, Class>,
    sequence_room_id: u64,
    rooms: HashMap<RoomId, Room>,
    sequence_schedule_id: u64,
    schedules: HashMap<ScheduleId, Schedule>,
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_students_to_class(
        &mut self,
        student_ids: Vec<StudentId>,
        class_id: ClassId,
    ) -> Result<(), String> {
        let mut students = Vec::with_capacity(student_ids.len());
        for student_id in &student_ids {
            let student = self.students.get(student_id).ok_or("Student not found")?;
            students.push(student.clone());
        }

        let class = self.classes.get_mut(&class_id).ok_or("Class not found")?;
        class.students.extend(students);
        Ok(())
    }
}

impl Repository<Student> for InMemoryDatabase {
    type Id = StudentId;

    fn add(&mut self, mut student: Student) -> StudentId {
        let id = StudentId(self.sequence_student_id);
        self.sequence_student_id += 1;
        student.id = Some(id);
        self.students.insert(id, student);
        id
    }

    fn get(&self, id: StudentId) -> Option<&Student> {
        self.students.get(&id)
    }

    fn all<'a>(&'a self) -> impl Iterator<Item = (&'a StudentId, &'a Student)>
    where
        StudentId: 'a,
        Student: 'a,
    {
        self.students.iter()
    }
}

impl Repository<Class> for InMemoryDatabase {
    type Id = ClassId;

    fn add(&mut self, mut class: Class) -> ClassId {
        let id = ClassId(self.sequence_class_id);
        self.sequence_class_id += 1;
        class.id = Some(id);
        self.classes.insert(id, class);
        id
    }

    fn get(&self, id: ClassId) -> Option<&Class> {
        self.classes.get(&id)
    }

    fn all<'a>(&'a self) -> impl Iterator<Item = (&'a ClassId, &'a Class)>
    where
        ClassId: 'a,
        Class: 'a,
    {
        self.classes.iter()
    }
}

impl Repository<Room> for InMemoryDatabase {
    type Id = RoomId;

    fn add(&mut self, mut room: Room) -> RoomId {
        let id = RoomId(self.sequence_room_id);
        self.sequence_room_id += 1;
        room.id = Some(id);
        self.rooms.insert(id, room);
        id
    }

    fn get(&self, id: RoomId) -> Option<&Room> {
        self.rooms.get(&id)
    }

    fn all<'a>(&'a self) -> impl Iterator<Item = (&'a RoomId, &'a Room)>
    where
        RoomId: 'a,
        Room: 'a,
    {
        self.rooms.iter()
    }
}

impl Repository<Schedule> for InMemoryDatabase {
    type Id = ScheduleId;

    fn add(&mut self, mut schedule: Schedule) -> ScheduleId {
        let id = ScheduleId(self.sequence_schedule_id);
        self.sequence_schedule_id += 1;
        schedule.id = Some(id);
        self.schedules.insert(id, schedule);
        id
    }

    fn get(&self, id: ScheduleId) -> Option<&Schedule> {
        self.schedules.get(&id)
    }

    fn all<'a>(&'a self) -> impl Iterator<Item = (&'a ScheduleId, &'a Schedule)>
    where
        ScheduleId: 'a,
        Schedule: 'a,
    {
        self.schedules.iter()
    }
}
