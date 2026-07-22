use agendamento::model::Student;
use agendamento::repository::{InMemoryDatabase, Repository};

fn main() {
    let mut db = InMemoryDatabase::new();

    let id = db.add(Student {
        id: None,
        name: "Ada".to_string(),
        class: "Math".to_string(),
    });

    // `get`'s entity type can't be inferred from the id alone (the id is an
    // associated type of the trait), so annotate the binding to pick the impl.
    let student: Option<&Student> = db.get(id);
    if let Some(student) = student {
        println!("Student #{}: {} in {}", id.0, student.name, student.class);
    }
}
