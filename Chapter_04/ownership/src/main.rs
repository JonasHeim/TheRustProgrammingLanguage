fn main() {
    let mut s1 = String::from("hello");
    
    change(&mut s1);

    println!("The value of s1 is: {}", s1);
}

fn change(some_string: &mut String) {
    some_string.push_str(" world");
}
