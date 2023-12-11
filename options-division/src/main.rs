fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let res = division(5.1, 7.0);
    match res {
        Some(x) => println!("{:.10}", x),
        None => println!("cannot divide by 0"),
    }
}