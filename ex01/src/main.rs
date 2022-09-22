use std::io;

fn main() {
    let input = get_input();
    let input = input.trim();

    match input.parse::<i32>() {
        Ok(val) => print_number(val),
        Err(_) => println!("{input} is not a valid number, try again."),
    }
}

fn get_input() -> String {
    let mut input = String::new();

    println!("Type a number:");
    io::stdin()
        .read_line(&mut input)
        .expect("something went wrong...");
    input
}

fn print_number(num: i32) {
    println!("\nthe number entered was {num}");
}
