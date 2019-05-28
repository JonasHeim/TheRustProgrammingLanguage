fn main() {
    let v = vec![1, 2, 3, 45];

    let third: &i32 = &v[2];
    println!("third {}", third);
    let third: Option<&i32> = v.get(2);
}
