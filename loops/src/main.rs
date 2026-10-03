fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    let temperature_fahr = 60;
    let temperature_cels = fahrenheit_to_celsius(temperature_fahr);
    println!("tempetature: {temperature_fahr:.1}ºF  {temperature_cels:.1}ºC");
    
    let n = 9;
    let nth_fibonacci = nth_fibonacci(n);
    println!("The {n}th Fibonacci number is {nth_fibonacci}");
    print_the_twelve_days_of_christmas();
}
//Convert temperatures between Fahrenheit and Celsius.
//Generate the nth Fibonacci number.
//Print the lyrics to the Christmas carol 
//“The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn print_the_twelve_days_of_christmas() {
    let days_of_christmas = ["first", "second", "third", "fourth", "fifth", "sixth",
    "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"];

    for day in 0..12 {
        println!("");
        println!("On the {} day of Christmas", days_of_christmas[day]);
        println!("My true love sent to me");
        if day == 11 {
            println!("Twelve drummers drumming");
        }
        if day >= 10 {
            println!("Eleven piper piping");
        }
        if day >= 9 {
            println!("Ten lords a-leaping");
        }
        if day >= 8 {
            println!("Nine ladies dancing");
        }
        if day >= 7 {
            println!("Eight maids a-milking");
        }
        if day >= 6 {
            println!("Seven swans a-swimming");
        }
        if day >= 5 {
            println!("Six geese a-laying");
        }
        if day >= 4 {
            println!("Five golden rings");
        }
        if day >= 3 {
            println!("Four calling birds");
        }
        if day >= 2 {
            println!("Three French hens");
        }
        if day >= 1 {
            println!("Two turtle doves and");   
        }
        println!("A partridge in a pear tree");
    }
}

fn fahrenheit_to_celsius(temperature_fahr: i32) -> f64 {
    (temperature_fahr - 32) as f64 / 1.8
}

fn nth_fibonacci(n: i64) -> i128 {
    match n {
        1 => 1,
        2 => 1,
        _ => nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
    }
}