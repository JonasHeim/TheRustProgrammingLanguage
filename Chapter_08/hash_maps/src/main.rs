use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name);

    match score {
        None => (),
        Some(i) => println!("Score of team {} is {}", team_name, i),
    }
    
    for (key, value) in &scores {
        println!("Team {} has {} points.", key, value);
    }
}
