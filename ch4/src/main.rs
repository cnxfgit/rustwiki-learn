fn mut_string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn clone_string() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_wonership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn take_and_copy() {
    let s = String::from("value");
    takes_wonership(s);

    let n = 123;
    makes_copy(n);

    // println!("{}", s); //err
    println!("{}", n);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn take_and_give_back(some_string: String) -> String {
    some_string
}

fn ownership() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = take_and_give_back(s2);

    println!("{} {}", s1, s3);

    let (s3, len) = calculate_length(s3);
    println!("The length of '{}' is {}.", s3, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_length_borrow(s: &String) -> usize {
    s.len()
}

fn change_borrow(s: &mut String) {
    s.push_str(" world!");
}

fn borrow_func() {
    let s = String::from("hello");
    let len = calculate_length_borrow(&s);
    println!("The length of '{}' is {}.", s, len);

    let mut s = s;
    change_borrow(&mut s);
    println!("{}", s);
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

fn slice() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("word {} is useless", word);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let slice = first_word_slice(&s);
    println!("{}", slice);
}

fn main() {
    mut_string();
    clone_string();
    take_and_copy();
    ownership();
    borrow_func();
    slice();
}
