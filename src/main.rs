// This program compares the performance differences
// of Protocol Buffers and JSON.
// Writes and reads large amounts of data and shows
// how long it took for each to complete.
// Protocol Buffers completes several times faster
// than JSON when working with large amounts of data.

use protobuf_rust::*;
use std::time::Instant;

// Change this to see more accurate results.
const EMPLOYEE_COUNT: u16 = 10000;

// Protocol Buffers write and read
fn protocol_buffers() {
    let start_time = Instant::now();
    let mut employee_list = employee_pb::create_employee_list();
    
    println!("\nCreating {} employees...", EMPLOYEE_COUNT);
    for _ in 0..EMPLOYEE_COUNT {
        employee_list.employees.push(employee_pb::create_employee());
    }

    println!("Writing employee list to file and reading it...");
    employee_pb::write_employee_list_to_file(
        &employee_pb::serialize_employee_list(&employee_list)
    );
    let employee_list_read = employee_pb::read_employee_list_from_file();
    println!("First employee: {:?}", employee_list_read.employees.get(0).unwrap());

    let elapsed = start_time.elapsed();
    println!("[Protocol Buffers] Time elapsed as seconds: {}", elapsed.as_secs_f64());
}

// JSON write and read
fn json() {
    let start_time = Instant::now();
    let mut employee_list = employee_json::create_employee_list();

    println!("\nCreating {} employees...", EMPLOYEE_COUNT);
    for _ in 0..EMPLOYEE_COUNT {
        employee_list.employees.push(employee_json::create_employee());
    }

    println!("Writing employee list to file and reading it...");
    employee_json::write_employee_list_json(&employee_list).unwrap();
    let employee_list_read = employee_json::read_emloyee_list_json().unwrap();
    println!("First employee: {:?}", employee_list_read.employees.get(0).unwrap());

    let elapsed = start_time.elapsed();
    println!("[JSON] Time elapsed as seconds: {}", elapsed.as_secs_f64());
}

fn main() {
    protocol_buffers();
    json();
    println!();
}
