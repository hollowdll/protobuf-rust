// Protocol Buffers implementation.
// See `employee_json` module for JSON implementation.

use std::{
    io::{
        Cursor,
        Write,
    },
    fs::{
        self,
        File,
    }
};
use prost::{
    Message,
    DecodeError,
};

pub mod employee_json;

const FILE_PATH: &str = "employee_buffer.txt";
const LIST_FILE_PATH: &str = "employee_list.txt";

// Include generated Rust code which is generated from employees.proto.
// The code is generated at compile time.
pub mod employees {
    include!(concat!(env!("OUT_DIR"), "/employees.rs"));
}

/// Creates a new employee based on the Employee in employees.proto.
pub fn create_employee_populated() -> employees::Employee {
    let mut employee = employees::Employee::default();
    employee.id = 1;
    employee.email = "john@smith.com".to_string();
    employee.name = "John Smith".to_string();
    employee.set_work_type(employees::employee::WorkType::Hybrid);

    return employee
}

/// Creates a new employee with default values.
pub fn create_employee() -> employees::Employee {
    employees::Employee::default()    
}

/// Creates a new employee list with default values.
pub fn create_employee_list() -> employees::EmployeeList {
    employees::EmployeeList::default()
}

/// Serializes employee and encodes it to a buffer.
pub fn serialize_employee(employee: &employees::Employee) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(employee.encoded_len());
    employee.encode(&mut buf).unwrap();

    return buf
}

/// Deserializes employee buffer from binary format to rust data structure.
pub fn deserialize_employee(buf: &[u8]) -> Result<employees::Employee, DecodeError> {
    employees::Employee::decode(&mut Cursor::new(buf))
}

/// Serializes employee list and encodes it to a buffer.
pub fn serialize_employee_list(employee_list: &employees::EmployeeList) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(employee_list.encoded_len());
    employee_list.encode(&mut buf).unwrap();

    return buf
}

/// Deserializes employee list buffer from binary format to rust data structure.
pub fn deserialize_employee_list(buf: &[u8]) -> Result<employees::EmployeeList, DecodeError> {
    employees::EmployeeList::decode(&mut Cursor::new(buf))
}

/// Writes employee as buffer to a file.
pub fn write_employee_to_file(buf: &[u8]) {
    let mut file = File::create(FILE_PATH).unwrap();
    file.write(buf).unwrap();
}

/// Reads employee as buffer from a file.
pub fn read_employee_from_file() -> employees::Employee {
    let buf = fs::read(FILE_PATH).unwrap();
    let employee = deserialize_employee(&buf).unwrap();

    return employee
}

/// Writes employee list as buffer to a file.
pub fn write_employee_list_to_file(buf: &[u8]) {
    let mut file = File::create(LIST_FILE_PATH).unwrap();
    file.write(buf).unwrap();
}

/// Reads employee list as buffer from a file.
pub fn read_employee_list_from_file() -> employees::EmployeeList {
    let buf = fs::read(LIST_FILE_PATH).unwrap();
    let employee_list = deserialize_employee_list(&buf).unwrap();

    return employee_list
}
