use operations;
use std::io::stdin;

fn main() {
    println!("Operation menu");
    println!("------------------------");
    println!("1-Add");
    println!("2-Sustration");
    println!("3-Multiply");
    println!("4-Divide");

    println!("Enter the number operation of the perform...");

    let mut option_select = String::new();
    let mut value_a = String::new();
    let mut value_b = String::new();
    stdin().read_line(&mut option_select).unwrap();

    option_select = option_select.replace("\n", "");
    option_select = option_select.replace("\r", "");

    let option_select_i32: i32 = option_select.parse::<i32>().unwrap(); //Parser String to i32.

    println!("First value: ");
    stdin().read_line(&mut value_a).unwrap();
    value_a = value_a.replace("\n", "").replace("\r", "");
    let value_a_i32: i32 = value_a.parse::<i32>().unwrap();

    println!("Second value: ");
    stdin().read_line(&mut value_b).unwrap();
    value_b = value_b.replace("\n", "").replace("\r", "");
    let value_b_i32: i32 = value_b.parse::<i32>().unwrap();

    let result = match option_select_i32 {
        1 => operations::add(value_a_i32, value_b_i32),
        2 => operations::sustraction(value_a_i32, value_b_i32),
        3 => operations::multiply(value_a_i32, value_b_i32),
        4 => operations::divide(value_a_i32, value_b_i32),
        _ => 0,
    };

    println!("Result {}", result);
}
