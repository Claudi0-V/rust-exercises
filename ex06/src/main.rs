fn main() {
    operate(10, 12, "+");
    operate(3, -15, "*");
}

fn operate(num1: i32, num2: i32, operation: &str) {
    let result = calc(num1, num2, operation);
    let mut main_str = format!("\n{num1} {operation} {num2} = {result}");
    main_str.push_str(&positive_or_negative(result));
    main_str.push_str(&even_or_odd(result));
    println!("{}", main_str);
}

fn calc(num1: i32, num2: i32, operation: &str) -> i32 {
    if operation == "+" {
        num1 + num2
    } else if operation == "-" {
        num1 - num2
    } else if operation == "*" {
        num1 * num2
    } else {
        num1 / num2
    }
}

fn positive_or_negative(num: i32) -> String {
    let result = format!("\nthe number {num} is ");
    if num >= 0 {
        format!("{result} positive")
    } else {
        format!("{result} negative")
    }
}

fn even_or_odd(num: i32) -> String {
    let result = format!("\nthe number {num} is an");
    if num % 2 == 0 {
        format!("{result} even number;")
    } else {
        format!("{result} odd number;")
    }
}
