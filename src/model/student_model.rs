#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StudentId(pub u64);

#[derive(Clone)]
pub struct Student {
    pub id: Option<StudentId>,
    pub name: String,
    pub class: String,
}
