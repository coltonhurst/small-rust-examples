// Example from: https://www.youtube.com/watch?v=gesNaLkUJeA
use std::error::Error;

struct Employee {
    name: String,
    id: i64,
}

impl Employee {
    fn new(name: String, id: i64) -> Employee {
        Employee { name, id } // shortcut b/c local & struct names match
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn id(&self) -> i64 {
        self.id
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let employee: Employee = Employee::new(String::from("Employee"), 100);

    println!(
        "employee => name: {} id: {}",
        employee.name(),
        employee.id(),
    );

    Ok(())
}
