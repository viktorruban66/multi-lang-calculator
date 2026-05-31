use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter expression: ");
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.split_whitespace().collect();
    let a: f64 = parts[0].parse().unwrap();
    let op = parts[1];
    let b: f64 = parts[2].parse().unwrap();
    let result = match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => if b != 0.0 { a / b } else { 0.0 },
        _ => 0.0,
    };
    println!("{}", result);
}
