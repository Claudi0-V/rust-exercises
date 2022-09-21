fn main() {
    scores_average(6, 7, 8, 9);
    scores_average(6, 7, 9, 10);
}

fn scores_average(n1: i32, n2: i32, n3: i32, n4: i32) {
    let average = (n1 + n2 + n3 + n4) as f32 / 4.0;
    println!("the average of {n1}, {n2}, {n3}, {n4} is: {}", average);
}
