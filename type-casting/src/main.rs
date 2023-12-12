fn main() {
    let f = 24.4321_f32;
    let i = f as u8;
    let c = i as char;

    println!("{} {} {}", f , i, c);

    //println!("{}", 256 as char);  //out of range
    println!("{}", 255 as char);
    println!("{} {}", 181 as char, 214 as char);
}
