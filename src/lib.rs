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

const FILE_NAME: &str = "employee_buffer.txt";

// Include generated Rust code which is generated from employees.proto.
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

/// Serializes employee and encodes it to a buffer (binary format).
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

/// Writes employee as buffer to a file.
pub fn write_employee_to_file(buf: &[u8]) {
    let mut file = File::create(FILE_NAME).unwrap();
    file.write(buf).unwrap();
}

/// Reads employee as buffer from a file.
pub fn read_employee_from_file() -> employees::Employee {
    let buf = fs::read(FILE_NAME).unwrap();
    let employee = deserialize_employee(&buf).unwrap();

    return employee
}
