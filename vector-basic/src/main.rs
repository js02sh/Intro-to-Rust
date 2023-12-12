fn main() {
    let x = vec![1,2,3,4];
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    for i in x { //take ownership_ x no longer usable
        println!("{}",i);
    }

    for i in &v { // takes a reference
        println!("{}", i);
    }

    println!("{:?} {} {}", &v, v.len(), v.capacity());
    println!("{:?}", v.pop()); // pop the last value, the value removed
    println!("{:?}", v.pop());    

    let mut t: Vec<i32> = Vec::new();

    println!("{:?} {} {}", &t, t.len(), t.capacity());
    t.push(10);

    for i in &t {
        println!("{}", i);
    }
    println!("{:?}", t.pop()); // the value gone
    println!("{:?} {} {}", &t, t.len(), t.capacity());

    println!("{:?}", t.pop()); // none

}
