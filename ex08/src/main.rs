fn main() {
    even_and_odds(vec![1, 2, 3, 4, 5, 6, 6, 7, 8]);
}

fn even_and_odds(num_arr: Vec<u32>) {
    let mut odd = 0;
    let mut even = 0;
    for num in num_arr {
        if num % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }
    println!("\nThere were {even} even numbers and {odd} odd numbers");
}
