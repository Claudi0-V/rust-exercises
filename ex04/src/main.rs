fn main() {
    largest(2, 4);
    largest(7, 10);
    largest(10, 7);
}

fn largest(n1: i32, n2: i32) {
    println!("\nThe largest bestween {n1} and {n2} is:");
    if n1 > n2 {
        println!("{}", n1);
    } else {
        println!("{}", n2);
    }
}
