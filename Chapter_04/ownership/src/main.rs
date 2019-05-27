fn main() {
    let s1 = gives_ownership();

    println!("The value of s1 is: {}", s1);

    let s2 = String::from("hello");

    println!("The value of s2 is: {}", s2);

    let s3 = takes_and_gives_ownership_back(s2);

    println!("The value of s3 is: {}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("world");
    some_string
}

fn takes_and_gives_ownership_back(a_string: String) -> String {
    a_string
}
