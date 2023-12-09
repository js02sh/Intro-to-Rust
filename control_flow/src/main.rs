fn main() {
    let n = 2;

    if n < 5 {
        println!("true");
    } else {
        println!("n is less than five")
    }

    let a = vec![10, 20, 30, 40, 50];
    for i in a {
        println!("i: {}", i);   
    }

    for i in 1..11 {
        println!("i: {}", i);
    }

    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let n = 15;
    match n {
        1 => println!("One!"),
        2 | 3 | 5| 7| 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),

    }

    //let pair = (0, -2);
    let pair2 = (2,0);
    match pair2 {
        (0,y) => println!("y: {}", y),
        (x, 0) => println!("x: {}", x),
        _ => println!("No match"),
    }

    let pair3 = (6, -5);
    match pair3 {
        (x, y) if x==y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Equal Zero"),
        (x, _) if x % 2 == 0 => println!("X is even"),
        _ => println!("No match"),
    }

    let p = 5;
    match p {
        n @ 1 ..= 12 => println!("n: {}", n),
        n @ 13 ..= 19 => println!("n: {}", n),
        _ => println!("no match"),
    }

    let q = 5;
    let n = match q {
        n @ 1..=12 => n,
        n @ 13..=19 => n,
        _ => 0,
    };

    println!("n: {}", n);

}
