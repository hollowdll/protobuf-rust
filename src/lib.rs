// Include generated Rust code which is generated from employees.proto
pub mod employees {
    include!(concat!(env!("OUT_DIR"), "/employees.rs"));
}

/// Creates a new employee based on the Employee in employees.proto
pub fn create_employee() -> employees::Employee {
    let mut employee = employees::Employee::default();
    employee.id = 1;
    employee.email = "john@smith.com".to_string();
    employee.name = "John Smith".to_string();
    employee.set_work_type(employees::employee::WorkType::Hybrid);

    return employee
}
