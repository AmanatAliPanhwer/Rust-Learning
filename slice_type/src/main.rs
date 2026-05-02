fn main() {
    let mut s: String = String::from("Hello World");
    let res: &str = find_first_word(&s);

    println!("for string {s} the Result is {}", res.len());
    s.clear();
}

fn find_first_word(input: &String) -> &str {
    let s: &[u8] = input.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }

    &input[..]
}