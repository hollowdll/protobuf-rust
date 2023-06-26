use protobuf_rust::create_employee_populated;

fn main() {
    let employee = create_employee_populated();
    println!("{:?}", employee);
}
