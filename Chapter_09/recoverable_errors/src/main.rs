use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => {
                    panic!("There was a problem creating the file: {:?}", error);
                }
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        },
    };

    let f = File::open("text.txt").unwrap();

}
