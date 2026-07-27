use std::fmt;

use crate::model::ClassId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StudentId(pub u64);

impl fmt::Display for StudentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
pub struct Student {
    pub id: Option<StudentId>,
    pub name: String,
    pub class: Option<ClassId>,
}
