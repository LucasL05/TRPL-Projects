fn main() {
    let mut s = String::from("ahoy");

    takes_ownersip(&mut s);
    println!("One more time, folks: {s}!");
}

fn takes_ownersip(s: &mut String) {
        println!("{s}, world!");
        s.push_str("no?");
}