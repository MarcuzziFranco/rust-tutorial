use operations;
use std::io::stdin;

pub fn show_menu() {
    println!("Operation menu");
    println!("------------------------");
    println!("1-Add");
    println!("2-Sustration");
    println!("3-Multiply");
    println!("4-Divide");
}

pub fn get_input(label: &str) -> i32 {
    println!("{}", label);

    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    input_string = input_string.replace("\n", "").replace("\r", "");

    let value: i32 = input_string.parse::<i32>().unwrap();
    return value;
}

pub fn get_result_operation(option: i32, value_a: i32, value_b: i32) -> i32 {
    return match option {
        1 => operations::add(value_a, value_b),
        2 => operations::sustraction(value_a, value_b),
        3 => operations::multiply(value_a, value_b),
        4 => operations::divide(value_a, value_b),
        _ => 0,
    };
}
