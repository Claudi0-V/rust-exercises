fn main() {
    odds(31);
    odds(100);
}

fn odds(n: u32) {
    let mut x = 1;
    println!("\n...starting");
    while x <= n {
        println!("{x}");
        x += 2;
    }
    println!("...end of program. ;-] ");
}
