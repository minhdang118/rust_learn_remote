fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn main() {
    let mut s = String::from("hello world");
    let mut t = String::from("hello");
    let word = first_word(&s[..]);
    // s.clear();

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &t[0..2];

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 4]);


}