// Data structures for JSON

use std::fs::{self, File};
use std::io::{self, Write};
use serde::{Serialize, Deserialize};

const FILE_PATH: &str = "employee_list.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub id: u32,
    pub age: u32,
    pub name: String,
    pub email: String,
    pub boss: Option<Box<Employee>>,
    pub work_type: WorkType,
}

impl Employee {
    pub fn new() -> Employee {
        Employee {
            id: 0,
            age: 0,
            name: String::from(""),
            email: String::from(""),
            boss: None,
            work_type: WorkType::Remote,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WorkType {
    Remote,
    Office,
    Hybrid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeList {
    employees: Vec<Employee>,
}

impl EmployeeList {
    pub fn new() -> EmployeeList {
        EmployeeList {
            employees: Vec::new(),
        }
    }
}

pub fn create_employee() -> Employee {
    Employee::new()
}

pub fn write_employee_list_json(employee_list: &EmployeeList) -> io::Result<()> {
    let json = serde_json::to_string_pretty(&employee_list)?;
    let mut file = File::create(FILE_PATH)?;
    file.write(json.as_bytes())?;

    Ok(())
}

pub fn read_emloyee_list_json() -> io::Result<EmployeeList> {
    let contents = fs::read_to_string(FILE_PATH)?;
    let employee_list: EmployeeList = serde_json::from_str(contents.as_str())?;

    Ok(employee_list)
}
