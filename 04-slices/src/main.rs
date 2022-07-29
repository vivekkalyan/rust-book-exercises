fn main() {
    let mut s = String::from("Hello World!");
    let word = first_word(&s);

    s.clear(); // this empties the string, making it equal to ""

    println!("{}", word);
    // word still has the value of 5 here, but there is no more string that we can meaningfully use
    // the value of 5 with. word is now totally invalid!
    //
    let mut s = String::from("Hello World!");
    let word = first_word_slice(&s);

    s.clear(); // error!

    println!("{}", word);

    let word = first_word_slice(&s[0..6]);

    println!("{}", word);
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
            return &s[0..i];
        }
    }

    &s
}
