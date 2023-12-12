#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Text(String),
}
// Vector can only contain one type
// So using Enum type to use various types
fn main() {
    let r = vec![
        Example::Int(142),
        Example::Float(12.32),
        Example::Text(String::from("string")),
    ];

    println!("{:?}", &r);
}

