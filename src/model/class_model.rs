use crate::model::Student;

pub type ClassId = u64;

pub struct Class {
    pub id: Option<ClassId>,
    pub name: String,
    pub students: Vec<Student>,
}
