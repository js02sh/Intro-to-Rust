use std::io;

fn main() {
    println!("Here for practicing Match");
    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("Faidl to read line");

    let m = match n.trim().parse::<u32>().expect("Please Write valid Number") {
        1 => (n,"one"),
        2 | 3 | 5 | 7 | 11 => (n,"prime"),
        13..=19 => (n, "teen"),
        _ => (n, "nothing special"),
    };

    println!("{}", format_args!("Value: {}Description: {}", m.0, m.1)); // make format print

    println!("The Two number Test ");
    let (mut a, mut b) = (String::new(), String::new());
    println!("A: ");
    io::stdin().read_line(&mut a).expect("Faild to read line");
    println!("B: ");
    io::stdin().read_line(&mut b).expect("Faild to read line");
    let a = a.trim().parse::<u32>().expect("Please write valid number");
    let b = b.trim().parse::<u32>().expect("Please Write valid number");

    let (m,n) = match (a,b) {
        (a,b) if a==b => {
            println!("They are same");
            (a, b)
        },
        _ => {
            println!("They are different");
            (a,b)
        },
    };

    println!("{}", format_args!("A: {}, B: {}", m, n)); // make format print

    

}
