use std::io;
fn last_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate().rev() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    &s[..]
}

fn main() {
    println!("Enter a sentence:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let word = last_word(&input);
   println!("The last word is: {}", word);
}
