fn main() {
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("{}", s3);
    println!("{}", s2);
    // println!("{}", s1);
    // s1 not valid anymore, was moved at + operator
}
