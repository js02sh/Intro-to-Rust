use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

//Method
impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(width: u32, height: u32) -> Object {
        Object { 
            width, 
            height, 
        }
    }
}

//Related Functions
impl Object {
    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

//Debug formatting
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}



fn main() {
    let o = Object {
        width: 35,
        height: 55,
    };

    let obj = Object::new(57, 83);

    o.show();
    obj.show();

    println!("{}", o); //debug
    println!("{}", obj); //pretty debug

}
