use protobuf_rust::*;

fn main() {
    let mut employee = create_employee_populated();
    println!("{:?}", employee);
    employee.age = 50;

    write_employee_to_file(&serialize_employee(&employee));
    let employee_read = read_employee_from_file();
    println!("{:?}", employee_read);
}
