// Protocol Buffers implementation.
// See `employee_json` module for JSON implementation.

pub mod employee_json;
pub mod employee_pb;

// Include generated code from employees.proto.
// The code is generated at compile time.
pub mod employees {
    include!(concat!(env!("OUT_DIR"), "/employees.rs"));
}
