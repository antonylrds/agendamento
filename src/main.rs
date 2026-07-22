use agendamento::model::Student;
use agendamento::repository::Database;

fn main() {
    let mut db = Database::new();

    let id = db.add_student(Student {
        id: None,
        name: "Ada".to_string(),
        class: "Math".to_string(),
    });

    if let Some(student) = db.get_student(id) {
        println!("Student #{id}: {} in {}", student.name, student.class);
    }
}
