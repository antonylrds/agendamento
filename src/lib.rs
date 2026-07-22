use std::collections::HashMap;
use time::UtcDateTime;

pub type StudentId = u64;
#[derive(Clone)]
pub struct Student {
    pub id: StudentId,
    pub name: String,
    pub class: String
}

pub type ClassId = u64;
pub struct Class {
    pub id: ClassId,
    pub name: String,
    pub students: Vec<Student>,
}

pub type RoomId = u64;
pub struct Room {
    pub id: RoomId,
    pub name: String,
}

pub type ScheduleId = u64;
pub struct Schedule {
    pub id: ScheduleId,
    pub class_id: ClassId,
    pub room_id: RoomId,
    pub time: UtcDateTime
}

#[derive(Default)]
pub struct Studio {
    next_student_id: StudentId,
    students: HashMap<StudentId, Student>,
    next_class_id: ClassId,
    classes: HashMap<ClassId, Class>,
    next_room_id: RoomId,
    rooms: HashMap<RoomId, Room>,
    next_schedule_id: ScheduleId,
    schedules: HashMap<ScheduleId, Schedule>,
}


impl Studio {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_student(&mut self, student: Student) -> StudentId {
        let id = self.next_student_id;
        self.next_student_id += 1;
        self.students.insert(id, student);
        id
    }

    pub fn get_student(&self, id: StudentId) -> Option<&Student> {
        self.students.get(&id)
    }

    pub fn all_students(&self) -> impl Iterator<Item = (&StudentId, &Student)> {
        self.students.iter()
    }

    pub fn add_class(&mut self, class: Class) -> ClassId {
        let id = self.next_class_id;
        self.next_class_id += 1;
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

    pub fn add_room(&mut self, room: Room) -> RoomId {
        let id = self.next_room_id;
        self.next_room_id += 1;
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

    pub fn add_schedule(&mut self, schedule: Schedule) -> ScheduleId {
        let id = self.next_schedule_id;
        self.next_schedule_id += 1;
        self.schedules.insert(id, schedule);
        id
    }
}
