fn main() {
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
