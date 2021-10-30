fn main() {
    // create string value
    // let mut s = String::from("Hello");
    // s.push_str(" World!");
    // println!("{}", s)

    // let s = String::from("Hello!");
    // let len = calculate_len(&s);
    // println!("{}", len)

    let mut s = String::from("Hello! world");
    // let word_index = first_word(&s);
    // println!("{}", word_index)
    // let hello = &s[0..5];

    let word_index = first_word_slice(&s[..]);
    println!("{}", word_index)
}
fn calculate_len(s: &String) -> usize {
    return s.len();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
