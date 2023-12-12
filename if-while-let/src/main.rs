// learn about "if let", "while let"

fn main() {
    let s = Some('c');

    match s {
        Some(i) => println!("{}",i),
        _ => {},
    }

    if let Some(i) = s {
        println!("{}", i);
    } 
    // else { // no need
    //     {}
    // }
    println!("=============");

    let mut s = Some(0);
    
    while let Some(i) = s {
        if i > 19 {
            println!("Quit");
            s = None;
        } else {
            println!("{}", i);
            s = Some(i + 2);
        }
    }

    // loop {
    //     match s {
    //         Some(i) => if i > 19 {
    //             println!("Quit");
    //             s = None;
    //         } else {
    //             println!("{}", i);
    //             s = Some(i + 2);
    //         },
    //         _ => {
    //             break;
    //         }
    //     }
    // }
    // println!("using While let");

    
}
