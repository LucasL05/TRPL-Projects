fn main() {
    let x: u16 = 5;
    println!("The value of x is: {x}");
    let a: [i16; 5] = [12; 5];
    println!("{}", a[1]);
    {
    let x = -20;
    
    println!("The value of x is: {x}");
    }
}
