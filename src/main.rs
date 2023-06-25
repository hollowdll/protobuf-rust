use protobuf_rust::create_employee;

fn main() {
    let employee = create_employee();
    println!("{:?}", employee);
}
