use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    hm.insert(String::from("random"), 12);
    hm.insert(String::from("strings"), 49);
    hm.insert(String::from("strings"), 44);

    for (k, v) in &hm {
        println!("{}: {}", k, v);
    }

    match hm.get(&String::from("random")) {
        Some(&n) => println!("{}", n),
        _ => println!("no match"),
    }

    println!("{:?}", hm.get(&String::from("strings")));

    println!("Delete the strings key");
    hm.remove(&String::from("strings"));
    println!("{:?}", hm.get(&String::from("strings")));

    for (k, v) in &hm {
        println!("{}: {}", k, v);
    }
    
}
