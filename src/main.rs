// This program compares the performance differences
// of Protocol Buffers and JSON.
// Writes and reads large amounts of data and shows
// how long it took for each to complete.
// Protocol Buffers completes several times faster
// than JSON when working with large amounts of data.

use protobuf_rust::*;
use std::time::Instant;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of employees to create
    #[arg(short, long, default_value_t = 1000)]
    employee_count: u16,
}

// Protocol Buffers write and read
fn protocol_buffers(employee_count: u16) -> f64 {
    let start_time = Instant::now();
    let mut employee_list = employee_pb::create_employee_list();
    
    println!("\nCreating {} employees...", employee_count);
    for _ in 0..employee_count {
        employee_list.employees.push(employee_pb::create_employee());
    }

    println!("Writing employee list to file and reading it...");
    employee_pb::write_employee_list_to_file(
        &employee_pb::serialize_employee_list(&employee_list)
    );
    let employee_list_read = employee_pb::read_employee_list_from_file();
    println!("First employee: {:?}", employee_list_read.employees.get(0).unwrap());

    let elapsed = start_time.elapsed().as_secs_f64();
    println!("[Protocol Buffers] Time elapsed as seconds: {}", elapsed);

    return elapsed
}

// JSON write and read
fn json(employee_count: u16) -> f64 {
    let start_time = Instant::now();
    let mut employee_list = employee_json::create_employee_list();

    println!("\nCreating {} employees...", employee_count);
    for _ in 0..employee_count {
        employee_list.employees.push(employee_json::create_employee());
    }

    println!("Writing employee list to file and reading it...");
    employee_json::write_employee_list_json(&employee_list).unwrap();
    let employee_list_read = employee_json::read_emloyee_list_json().unwrap();
    println!("First employee: {:?}", employee_list_read.employees.get(0).unwrap());

    let elapsed = start_time.elapsed().as_secs_f64();
    println!("[JSON] Time elapsed as seconds: {}", elapsed);

    return elapsed
}

fn main() {
    let args = Args::parse();

    let pb_seconds = protocol_buffers(args.employee_count);
    let json_seconds = json(args.employee_count);
    let difference = json_seconds / pb_seconds;

    println!("\nProtocol Buffers was {} times faster than JSON\n", difference);
}
