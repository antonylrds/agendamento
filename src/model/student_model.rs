pub type StudentId = u64;
#[derive(Clone)]
pub struct Student {
    pub id: Option<StudentId>,
    pub name: String,
    pub class: String
}