#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("Quarter from {state:?}");
                25
            }
        }
    }

    fn describe_state_quarter(&self) -> Option<String> {
        let Coin::Quarter(state) = self else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new"))
        }
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    Coin::Quarter(UsState::Alaska).value_in_cents();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(five);
    dbg!(six);
    dbg!(none);

}