fn main() {
    print_ifs_vowel("a");
    print_ifs_vowel("z");
}

fn print_ifs_vowel(letter: &str) {
    if is_vowel(&letter) {
        println!("\n{letter} is a vowel");
    } else {
        println!("\n{letter} is not a vowel")
    }
}

fn is_vowel(letter: &str) -> bool {
    let vowels = ["a", "e", "i", "o", "u"];
    for vowel in vowels {
        if vowel == letter {
            return true;
        }
    }
    return false;
}
