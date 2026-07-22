use crate::model::Student;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClassId(pub u64);

pub struct Class {
    pub id: Option<ClassId>,
    pub name: String,
    pub students: Vec<Student>,
}
