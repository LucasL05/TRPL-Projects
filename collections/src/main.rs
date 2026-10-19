use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut v  = Vec::new();
    v.push(5);
    v.push(8);

    let text = "it was the best of times it was the worst of times";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // count shouldn't be initialized as mut, since it holds a reference, not a value.
    }

    let mut s = String::new();

    let data = "initial contents";
    let string = data.to_string();
    let string2 = "initial contents".to_string();
    let hello = String::from("Hello!"); // A lot prettier tham the former option
    let mega_string = format!("{hello}, {data} {string}");

    println!("{map:?}");
    
}
