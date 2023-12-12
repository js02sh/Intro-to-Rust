// enum Result<T,E> { //alow us to see why the error occur
//     Ok(T),
//     Err(E),
// }

use std::fs::File;

fn main() {
    //let f = File::open("Nests.txt"); // <- Wrong file name
    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => {
            println!("We found the File");
            file
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    println!("{:?}", f);
}
