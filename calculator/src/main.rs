use manager_inputs;

fn main() {
    manager_inputs::show_menu();
    let option: i32 = manager_inputs::get_input("Select option");
    let value_a: i32 = manager_inputs::get_input("First value");
    let value_b: i32 = manager_inputs::get_input("Second value");

    let result = manager_inputs::get_result_operation(option, value_a, value_b);
    println!("Result: {}", result);
}
