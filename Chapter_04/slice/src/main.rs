fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of String 
    let word = first_word(&my_string[..]);
    
    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal);

    // Because string literals are string slices already,
    // this works too, without the slice syntax.
    let word = first_word(my_string_literal);
}

fn first_word(some_string: &str) -> &str {
    let bytes = some_string.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &some_string[0..i];
        }
    }

    &some_string[..]
}
