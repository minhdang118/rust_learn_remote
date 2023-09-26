fn main() {
    // println!("Hello, world!");
    another_fn(8, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = mul_2(7);

    println!("The value of x is: {x}");
}

fn another_fn(x: i32, l: char) {
    println!("Parameter as a string is {x}{l}");
}

fn mul_2(x: i32) -> i32 {
    x * 2
}